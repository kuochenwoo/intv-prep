//! Real, bounded connectivity probe. Records metadata only, never credentials or response text.
use serde_json::{json, Value};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use swe_interview_prep::credentials;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = credentials::read_key("deepseek")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let report = runtime.block_on(probe(key));
    let report = serde_json::to_string_pretty(&report)?;
    std::fs::write("/tmp/intv-deepseek-probe.json", &report)?;
    println!("{report}");
    Ok(())
}

async fn probe(key: String) -> Value {
    let start = Instant::now();
    let mut report = json!({
        "started_unix_seconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "endpoint": "https://api.deepseek.com/chat/completions",
        "model": "deepseek-flash", "stream": false,
        "prompt": "Reply OK.", "max_tokens": 32,
        "usage": null, "completed": false
    });
    let operation = async {
        let client = reqwest::Client::builder()
            .https_only(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "client_error")?;
        let mut response = client
            .post("https://api.deepseek.com/chat/completions")
            .bearer_auth(&key)
            .json(
                &json!({"model":"deepseek-flash","stream":false,"max_tokens":32,
                "thinking":{"type":"disabled"},"messages":[{"role":"user","content":"Reply OK."}]}),
            )
            .send()
            .await
            .map_err(|_| "transport_error")?;
        report["http_status"] = json!(response.status().as_u16());
        for name in ["content-type", "x-request-id", "x-ds-trace-id", "date"] {
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
        for name in ["prompt_tokens", "completion_tokens", "total_tokens"] {
            if let Some(count) = value["usage"][name].as_u64() {
                if report["usage"].is_null() {
                    report["usage"] = json!({});
                }
                report["usage"][name] = json!(count);
            }
        }
        report["completed"] = json!(
            response.status().is_success()
                && value["choices"][0]["finish_reason"] == "stop"
                && value["choices"][0]["message"]["content"]
                    .as_str()
                    .is_some_and(|s| !s.trim().is_empty())
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
