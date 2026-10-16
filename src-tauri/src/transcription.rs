//! OpenAI speech input only. Interview generation stays on the existing LLM provider.
use crate::{credentials, db, interview::Lang, usage};
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    io::Read,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const MODEL: &str = "gpt-4o-mini-transcribe";
const MAX_AUDIO_BYTES: usize = 20 * 1024 * 1024;

pub fn settings() -> Value {
    let status = match credentials::read_key("openai") {
        Ok(_) => "saved",
        Err(e) if !credentials::env_path().exists() || e.contains("尚未配置") => "missing",
        Err(_) => "unavailable",
    };
    json!({"model": MODEL, "keyStatus": status})
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcript {
    pub text: String,
    pub usage_warning: bool,
}

/// Encode the small fixed set of multipart fields without another dependency.
fn multipart(audio: &[u8], mime: &str, lang: Lang) -> Result<(String, Vec<u8>), String> {
    if audio.is_empty() || audio.len() > MAX_AUDIO_BYTES {
        return Err("录音为空或超过 20 MiB，请缩短录音".into());
    }
    let ext = match mime {
        "audio/mp4" => "mp4",
        "audio/webm" => "webm",
        "audio/ogg" => "ogg",
        _ => return Err("不支持此录音格式，请更换录音环境".into()),
    };
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut boundary = format!("swe-audio-{}-{nonce}", std::process::id());
    while audio
        .windows(boundary.len())
        .any(|part| part == boundary.as_bytes())
    {
        boundary.push('x');
    }
    // Generic language context only: no question, private checkpoints, history or suggested answer.
    let prompt = match lang {
        Lang::Zh => "中文夹杂 English 技术术语的软件工程面试回答。",
        Lang::En => {
            "A software engineering interview answer in English, possibly mixed with Chinese."
        }
    };
    let mut body = Vec::with_capacity(audio.len() + 1024);
    for (name, value) in [
        ("model", MODEL),
        ("response_format", "json"),
        ("prompt", prompt),
    ] {
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes());
    }
    body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"answer.{ext}\"\r\nContent-Type: {mime}\r\n\r\n").as_bytes());
    body.extend_from_slice(audio);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    Ok((boundary, body))
}

pub fn transcribe(
    path: &Path,
    module: &str,
    session_id: &str,
    audio: &[u8],
    mime: &str,
    lang: Lang,
) -> Result<Transcript, String> {
    {
        let c = db::connect(path)?;
        let active = match module {
            "p1" => crate::interview_store::session(&c, session_id)?.status == "active",
            "p2" => crate::system_design_store::session(&c, session_id)?.status == "active",
            _ => return Err("未知面试模块".into()),
        };
        if !active {
            return Err("当前会话不能接收语音输入".into());
        }
    }
    let (boundary, body) = multipart(audio, mime, lang)?;
    let key = credentials::read_key("openai")?;
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(90))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "无法初始化语音转写请求")?;
    let request = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(key)
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(body)
        .build()
        .map_err(|_| "无法初始化语音转写请求")?;
    let ctx = usage::Context {
        path: path.to_owned(),
        module: module.into(),
        session_id: Some(session_id.into()),
        operation: "transcription".into(),
    };
    // Exactly one HTTP attempt per explicit Transcribe click. No automatic retry.
    let id = usage::begin(&ctx, "openai", MODEL, false)?;
    let response = match client.execute(request) {
        Ok(r) => r,
        Err(_) => {
            let _ = usage::finish_transcription(path, id, "transport_error", &Value::Null);
            return Err("语音转写连接失败或超时，录音已保留，可手动重试".into());
        }
    };
    let status = response.status();
    if !status.is_success() {
        let _ = usage::finish_transcription(path, id, "transport_error", &Value::Null);
        return Err(match status.as_u16() {
            401 | 403 => "OpenAI 转写认证失败，请在设置中检查 Key 和项目权限",
            429 => "OpenAI 转写额度或速率受限，请检查账户后重试",
            413 => "录音为空或超过 20 MiB，请缩短录音",
            _ => "OpenAI 转写服务返回错误，录音已保留，可手动重试",
        }
        .into());
    }
    let mut bytes = Vec::new();
    if response
        .take(2 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .is_err()
        || bytes.len() > 2 * 1024 * 1024
    {
        let _ = usage::finish_transcription(path, id, "transport_error", &Value::Null);
        return Err("语音转写响应读取失败，录音已保留，可手动重试".into());
    }
    let value: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => {
            let _ = usage::finish_transcription(path, id, "content_error", &Value::Null);
            return Err("语音转写响应无效，录音已保留，可手动重试".into());
        }
    };
    let Some(text) = response_text(&value) else {
        let _ = usage::finish_transcription(path, id, "content_error", &value["usage"]);
        return Err("未识别到有效文本，请检查录音或重新录制".into());
    };
    // Never discard a paid transcript just because local accounting failed.
    let usage_warning = usage::finish_transcription(path, id, "success", &value["usage"]).is_err();
    Ok(Transcript {
        text,
        usage_warning,
    })
}

fn response_text(value: &Value) -> Option<String> {
    value["text"]
        .as_str()
        .map(str::trim)
        .filter(|t| !t.is_empty() && t.chars().count() <= 20000)
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multipart_uses_only_transcription_fields_and_preserves_audio() {
        for (mime, extension) in [
            ("audio/mp4", "mp4"),
            ("audio/webm", "webm"),
            ("audio/ogg", "ogg"),
        ] {
            let audio = b"\x00\x01\xff\r\nrecording";
            let (boundary, body) = multipart(audio, mime, Lang::Zh).unwrap();
            assert!(body.windows(audio.len()).any(|s| s == audio));
            let text = String::from_utf8_lossy(&body);
            assert_eq!(text.matches("Content-Disposition:").count(), 4);
            assert!(text.contains(MODEL));
            assert!(text.contains("name=\"response_format\"\r\n\r\njson"));
            assert!(text.contains(&format!("filename=\"answer.{extension}\"")));
            assert!(text.ends_with(&format!("\r\n--{boundary}--\r\n")));
            assert!(!text.contains("Authorization"));
            assert!(!text.contains("evaluationChecklist"));
        }
        let (_, body) = multipart(b"audio", "audio/mp4", Lang::En).unwrap();
        assert!(String::from_utf8(body)
            .unwrap()
            .contains("possibly mixed with Chinese"));
    }
    #[test]
    fn invalid_audio_and_mime_are_rejected_before_a_request() {
        assert!(multipart(b"", "audio/mp4", Lang::Zh).is_err());
        assert!(multipart(&vec![0; MAX_AUDIO_BYTES + 1], "audio/mp4", Lang::Zh).is_err());
        assert!(multipart(b"audio", "audio/mp4\r\nInjected: value", Lang::Zh).is_err());
        assert!(multipart(b"audio", "text/plain", Lang::Zh).is_err());
    }
    #[test]
    fn transcript_response_requires_actual_bounded_text() {
        assert_eq!(
            response_text(&json!({"text":"  我用 Redis 做 cache-aside。  "})),
            Some("我用 Redis 做 cache-aside。".into())
        );
        for value in [
            json!({"text":" "}),
            json!({"text":42}),
            json!({"error":{"message":"not a transcript"}}),
            json!({"text":"a".repeat(20001)}),
        ] {
            assert!(response_text(&value).is_none());
        }
    }
    #[test]
    fn speech_usage_keeps_unknowns_and_never_applies_chat_prices() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("voice.sqlite");
        let c = db::open(&path).unwrap();
        let ctx = usage::Context {
            path: path.clone(),
            module: "p1".into(),
            session_id: Some("authored-fixture".into()),
            operation: "transcription".into(),
        };
        // Even a pre-existing chat-style rate must never price audio tokens.
        c.execute(
            "INSERT INTO api_prices VALUES('openai',?1,?2)",
            rusqlite::params![
                MODEL,
                json!({"input":1000,"output":1000,"cacheRead":1000,"cacheWrite":1000}).to_string()
            ],
        )
        .unwrap();
        assert!(credentials::LlmSettings {
            provider: "openai".into(),
            model: MODEL.into(),
            timeout_seconds: 60
        }
        .validate()
        .is_err());
        let id = usage::begin(&ctx, "openai", MODEL, false).unwrap();
        usage::finish_transcription(
            &path,
            id,
            "success",
            &json!({"input_tokens":42,"output_tokens":8}),
        )
        .unwrap();
        let rows = usage::overview(&c, None, None).unwrap();
        assert_eq!(rows["records"][0]["inputTokens"], 42);
        assert_eq!(rows["records"][0]["outputTokens"], 8);
        assert_eq!(rows["records"][0]["provider"], "openai");
        assert!(rows["records"][0]["estimatedUsd"].is_null());
        let id = usage::begin(&ctx, "openai", MODEL, false).unwrap();
        usage::finish_transcription(&path, id, "transport_error", &Value::Null).unwrap();
        drop(c);
        let c = db::open(&path).unwrap();
        let rows = usage::overview(&c, None, None).unwrap();
        assert!(rows["records"][0]["inputTokens"].is_null());
        assert_eq!(rows["records"].as_array().unwrap().len(), 2);
    }
}
