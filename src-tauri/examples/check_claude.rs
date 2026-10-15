//! Real, bounded connectivity probe. Records metadata only, never credentials or response text.
use serde_json::{json, Value};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use swe_interview_prep::credentials;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = credentials::read_key("claude")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let report = runtime.block_on(probe(key));
    let report = serde_json::to_string_pretty(&report)?;
    std::fs::write("/tmp/intv-claude-probe.json", &report)?;
    println!("{report}");
    Ok(())
}

async fn probe(key: String) -> Value {
    let start = Instant::now();
    let mut report = json!({
        "started_unix_seconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "endpoint": "https://api.anthropic.com/v1/messages",
        "model": "claude-haiku-5-5", "stream": false,
        "prompt": "Reply OK.", "max_tokens": 512,
        "usage": null, "completed": false
    });
    let operation = async {
        let client = reqwest::Client::builder()
            .https_only(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "client_error")?;
        let mut response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &key)
            .header("anthropic-version", "2023-06-01")
            .json(
                &json!({"model":"claude-haiku-5-5","stream":false,"max_tokens":512,
                "output_config":{"effort":"low"},"messages":[{"role":"user","content":"Reply OK."}]}),
            )
            .send()
            .await
            .map_err(|_| "transport_error")?;
        report["http_status"] = json!(response.status().as_u16());
        for name in ["content-type", "request-id", "x-request-id", "date"] {
            if let Some(value) = response.headers().get(name).and_then(|v| v.to_str().ok()) {
                report["headers"][name] = json!(value.chars().take(200).collect::<String>());
            }
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "body_read_error")? {
            bytes.extend_from_slice(&chunk);
            report["response_bytes"] = json!(bytes.len());
            if bytes.len() > 65536 {
                return Err("response_too_large");
            }
        }
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| "invalid_json")?;
        report["completion_id"] = value["id"].clone();
        // Copy only numeric usage fields, never arbitrary server text.
        for name in [
            "input_tokens",
            "output_tokens",
            "cache_creation_input_tokens",
            "cache_read_input_tokens",
        ] {
            if let Some(count) = value["usage"][name].as_u64() {
                if report["usage"].is_null() {
                    report["usage"] = json!({});
                }
                report["usage"][name] = json!(count);
            }
        }
        report["completed"] = json!(
            response.status().is_success()
                && value["stop_reason"] == "end_turn"
                && value["content"]
                    .as_array()
                    .is_some_and(|blocks| blocks.iter().any(|b| b["type"] == "text"
                        && b["text"].as_str().is_some_and(|s| !s.trim().is_empty())))
        );
        Ok::<(), &str>(())
    };
    match tokio::time::timeout(Duration::from_secs(30), operation).await {
        Err(_) => report["error"] = json!("overall_timeout_30s"),
        Ok(Err(error)) => report["error"] = json!(error),
        Ok(Ok(())) => {}
    }
    report["elapsed_ms"] = json!(start.elapsed().as_millis() as u64);
    report
}
