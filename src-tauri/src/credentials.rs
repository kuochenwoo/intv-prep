use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
const SERVICE: &str = "local.swe.interview-prep.llm";
// System backend retained for migration after testing; accounts are provider-specific.
fn entry(provider: &str) -> Result<keyring::Entry, String> {
    if !["deepseek", "openai", "claude"].contains(&provider) {
        return Err("未知凭据供应商".into());
    }
    keyring::Entry::new(SERVICE, provider).map_err(|_| "无法初始化系统凭据存储".into())
}
pub fn save_system_key(provider: &str, key: &str) -> Result<(), String> {
    let key = key.trim();
    if key.len() < 10 || key.len() > 512 || !key.is_ascii() || key.chars().any(char::is_whitespace)
    {
        return Err("API Key 格式无效，请检查输入".into());
    }
    entry(provider)?
        .set_password(key)
        .map_err(|_| "无法写入系统凭据存储，请检查钥匙串权限".into())
}
pub fn read_system_key(provider: &str) -> Result<String, String> {
    match entry(provider)?.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => Err("尚未保存此供应商的 API Key，请在设置中配置".into()),
        Err(_) => Err("无法读取系统凭据存储，请解锁钥匙串并检查访问权限".into()),
    }
}
pub fn delete_system_key(provider: &str) -> Result<(), String> {
    match entry(provider)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err("无法从系统凭据存储删除 Key".into()),
    }
}
// Temporary test mode requested by the user. No automatic Keychain fallback/writes.
pub fn env_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("project root")
        .join(".env")
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
        dotenvy::from_path_iter(path).map_err(|_| "无法读取项目 .env，请先配置 API Key")?;
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
    write_env_key(&env_path(), provider, key)
}
pub fn read_key(provider: &str) -> Result<String, String> {
    read_env_key(&env_path(), provider)
}
pub fn delete_key(provider: &str) -> Result<(), String> {
    write_env_key(&env_path(), provider, "")
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
pub fn view(c: &Connection) -> Result<SettingsView, String> {
    let config = settings(c)?;
    let key_status = match read_key(&config.provider) {
        Ok(_) => "saved",
        Err(e) if !env_path().exists() || e.contains("尚未配置") => "missing",
        Err(_) => "unavailable",
    }
    .into();
    Ok(SettingsView {
        credential_source: "env".into(),
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
