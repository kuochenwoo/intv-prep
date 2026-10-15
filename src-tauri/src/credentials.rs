use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
const SERVICE: &str = "local.swe.interview-prep.llm";
// Separate credential accounts survive future provider changes. No plaintext fallback.
fn entry(provider: &str) -> Result<keyring::Entry, String> {
    if !["deepseek", "openai", "claude"].contains(&provider) {
        return Err("未知凭据供应商".into());
    }
    keyring::Entry::new(SERVICE, provider).map_err(|_| "无法初始化系统凭据存储".into())
}
pub fn save_key(provider: &str, key: &str) -> Result<(), String> {
    let key = key.trim();
    if key.len() < 10 || key.len() > 512 || !key.is_ascii() || key.chars().any(char::is_whitespace)
    {
        return Err("API Key 格式无效，请检查输入".into());
    }
    entry(provider)?
        .set_password(key)
        .map_err(|_| "无法写入系统凭据存储，请检查钥匙串权限".into())
}
pub fn read_key(provider: &str) -> Result<String, String> {
    match entry(provider)?.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => Err("尚未保存此供应商的 API Key，请在设置中配置".into()),
        Err(_) => Err("无法读取系统凭据存储，请解锁钥匙串并检查访问权限".into()),
    }
}
pub fn delete_key(provider: &str) -> Result<(), String> {
    match entry(provider)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err("无法从系统凭据存储删除 Key".into()),
    }
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
            provider: "deepseek".into(),
            model: "deepseek-flash".into(),
            timeout_seconds: 60,
        }
    }
}
impl LlmSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.provider != "deepseek" {
            return Err("P1 当前仅启用 DeepSeek；其他供应商的已存凭据会保留".into());
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
    let key_status = match entry(&config.provider) {
        Ok(entry) => match entry.get_password() {
            Ok(_) => "saved",
            Err(keyring::Error::NoEntry) => "missing",
            Err(_) => "unavailable",
        },
        Err(_) => "unavailable",
    }
    .into();
    Ok(SettingsView { config, key_status })
}
pub fn save_settings(c: &Connection, config: &LlmSettings) -> Result<(), String> {
    config.validate()?;
    let json = serde_json::to_string(config).map_err(|e| e.to_string())?;
    c.execute("INSERT INTO settings VALUES('llm_config',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value",[json]).map_err(|e|e.to_string())?;
    Ok(())
}
