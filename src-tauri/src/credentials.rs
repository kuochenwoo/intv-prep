use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

static ENV_WRITE_LOCK: Mutex<()> = Mutex::new(());

static ENV_PATH: OnceLock<PathBuf> = OnceLock::new();

#[cfg(debug_assertions)]
fn development_env_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("project root")
        .join(".env")
}

#[cfg(any(not(debug_assertions), test))]
fn release_env_path(config_dir: &Path) -> Result<PathBuf, String> {
    if !config_dir.is_absolute() {
        return Err("密钥配置目录必须是绝对路径".into());
    }
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(config_dir)
        .map_err(|_| "无法创建密钥配置目录")?;
    Ok(config_dir.join(".env"))
}

/// Release storage comes from Tauri's runtime app-config directory, never the checkout.
/// Development and explicit manual probes keep using the ignored project .env.
pub fn initialize_storage(_config_dir: &Path) -> Result<(), String> {
    #[cfg(debug_assertions)]
    let path = development_env_path();
    #[cfg(not(debug_assertions))]
    let path = release_env_path(_config_dir)?;
    if let Some(current) = ENV_PATH.get() {
        return if current == &path {
            Ok(())
        } else {
            Err("密钥存储已初始化，不能更换目录".into())
        };
    }
    ENV_PATH
        .set(path)
        .map_err(|_| "密钥存储已初始化，不能更换目录".into())
}

pub fn env_path() -> Result<PathBuf, String> {
    if let Some(path) = ENV_PATH.get() {
        return Ok(path.clone());
    }
    #[cfg(debug_assertions)]
    {
        Ok(development_env_path())
    }
    #[cfg(not(debug_assertions))]
    {
        Err("密钥存储尚未初始化".into())
    }
}

fn variable(provider: &str) -> Result<&'static str, String> {
    match provider {
        "deepseek" => Ok("DEEPSEEK_API_KEY"),
        "openai" => Ok("OPENAI_API_KEY"),
        "claude" => Ok("CLAUDE_API_KEY"),
        _ => Err("未知凭据供应商".into()),
    }
}
fn validate_env_value(key: &str) -> Result<(), String> {
    if key.len() < 10
        || key.len() > 512
        || !key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
    {
        return Err("Key 格式无效（只接受字母、数字、连字符和下划线）".into());
    }
    Ok(())
}
pub fn read_env_key(path: &std::path::Path, provider: &str) -> Result<String, String> {
    let wanted = variable(provider)?;
    let values =
        dotenvy::from_path_iter(path).map_err(|_| "无法读取密钥 .env，请先配置 API Key")?;
    for item in values {
        let (name, value) = item.map_err(|_| ".env 格式错误，请检查配置（未输出文件内容）")?;
        if name == wanted {
            if value.trim().is_empty() {
                return Err(".env 中尚未配置此 Provider 的 Key".into());
            }
            validate_env_value(&value)?;
            return Ok(value);
        }
    }
    Err(".env 中尚未配置此 Provider 的 Key".into())
}
pub fn write_env_key(path: &std::path::Path, provider: &str, key: &str) -> Result<(), String> {
    use std::io::Write;
    let _guard = ENV_WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let wanted = variable(provider)?;
    let key = key.trim();
    if !key.is_empty() {
        validate_env_value(key)?;
    }
    let previous = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(_) => return Err("无法读取 .env，未覆盖文件".into()),
    };
    let mut lines: Vec<String> = previous
        .lines()
        .filter(|line| {
            let line = line.trim().strip_prefix("export ").unwrap_or(line.trim());
            !line
                .split_once('=')
                .is_some_and(|(name, _)| name.trim() == wanted)
        })
        .map(str::to_owned)
        .collect();
    lines.push(format!("{wanted}={key}"));
    let parent = path.parent().ok_or(".env 路径无效")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|_| "无法创建 .env 临时文件")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        temp.as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))
            .map_err(|_| "无法设置 .env 权限")?;
    }
    temp.write_all((lines.join("\n") + "\n").as_bytes())
        .map_err(|_| "写入 .env 失败")?;
    temp.as_file().sync_all().map_err(|_| "同步 .env 失败")?;
    temp.persist(path)
        .map_err(|_| "无法原子替换 .env，原文件保留")?;
    Ok(())
}
pub fn save_key(provider: &str, key: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("Key 不可为空".into());
    }
    write_env_key(&env_path()?, provider, key)
}
pub fn read_key(provider: &str) -> Result<String, String> {
    read_env_key(&env_path()?, provider)
}
pub fn delete_key(provider: &str) -> Result<(), String> {
    write_env_key(&env_path()?, provider, "")
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LlmSettings {
    pub provider: String,
    pub model: String,
    pub timeout_seconds: u64,
}
impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            provider: "claude".into(),
            model: "claude-haiku-5-5".into(),
            timeout_seconds: 60,
        }
    }
}
impl LlmSettings {
    pub fn validate(&self) -> Result<(), String> {
        if !["deepseek", "claude"].contains(&self.provider.as_str()) {
            return Err("P1 支持 DeepSeek 和 Claude；其他供应商的已存凭据会保留".into());
        }
        if self.model.is_empty()
            || self.model.len() > 100
            || !self
                .model
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
            || !(15..=120).contains(&self.timeout_seconds)
        {
            return Err("模型名称或超时设置无效（15–120 秒）".into());
        }
        Ok(())
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsView {
    pub credential_source: String,
    pub credential_path: String,
    pub config: LlmSettings,
    pub key_status: String,
}
pub fn settings(c: &Connection) -> Result<LlmSettings, String> {
    let raw: Option<String> = c
        .query_row(
            "SELECT value FROM settings WHERE key='llm_config'",
            [],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let config = match raw {
        Some(s) => serde_json::from_str(&s).map_err(|_| "模型配置损坏")?,
        None => LlmSettings::default(),
    };
    config.validate()?;
    Ok(config)
}
pub fn key_status(provider: &str) -> &'static str {
    match read_key(provider) {
        Ok(_) => "saved",
        Err(e) if env_path().is_ok_and(|p| !p.exists()) || e.contains("尚未配置") => "missing",
        Err(_) => "unavailable",
    }
}
pub fn view(c: &Connection) -> Result<SettingsView, String> {
    let config = settings(c)?;
    let key_status = key_status(&config.provider).into();
    Ok(SettingsView {
        credential_source: "env".into(),
        credential_path: env_path()?.to_string_lossy().into_owned(),
        config,
        key_status,
    })
}
pub fn save_settings(c: &Connection, config: &LlmSettings) -> Result<(), String> {
    config.validate()?;
    let json = serde_json::to_string(config).map_err(|e| e.to_string())?;
    c.execute("INSERT INTO settings VALUES('llm_config',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value",[json]).map_err(|e|e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod storage_tests {
    use super::*;
    #[test]
    fn release_storage_is_private_runtime_based_and_does_not_copy_project_keys() {
        let dir = tempfile::tempdir().unwrap();
        let config = dir.path().join("fresh-app-config");
        let path = release_env_path(&config).unwrap();
        assert_eq!(path, config.join(".env"));
        assert!(!path.exists());
        write_env_key(&path, "claude", "fixture-release-claude-key").unwrap();
        write_env_key(&path, "deepseek", "fixture-release-deepseek-key").unwrap();
        assert_eq!(
            read_env_key(&path, "claude").unwrap(),
            "fixture-release-claude-key"
        );
        assert_eq!(
            read_env_key(&path, "deepseek").unwrap(),
            "fixture-release-deepseek-key"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(config).unwrap().permissions().mode() & 0o777,
                0o700
            );
            assert_eq!(
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }
    #[test]
    fn concurrent_provider_saves_preserve_both_keys_and_unrelated_settings() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");
        std::fs::write(&path, "OTHER_SETTING=keep\n").unwrap();
        for _ in 0..20 {
            let barrier = std::sync::Barrier::new(2);
            std::thread::scope(|scope| {
                for (provider, key) in [
                    ("deepseek", "fixture-text-key"),
                    ("openai", "fixture-voice-key"),
                ] {
                    let (path, barrier) = (&path, &barrier);
                    scope.spawn(move || {
                        barrier.wait();
                        write_env_key(path, provider, key).unwrap();
                    });
                }
            });
            assert_eq!(read_env_key(&path, "deepseek").unwrap(), "fixture-text-key");
            assert_eq!(read_env_key(&path, "openai").unwrap(), "fixture-voice-key");
            assert!(std::fs::read_to_string(&path)
                .unwrap()
                .contains("OTHER_SETTING=keep"));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }

    #[test]
    fn relative_release_storage_is_rejected() {
        assert!(release_env_path(Path::new("relative-config")).is_err());
    }
}
