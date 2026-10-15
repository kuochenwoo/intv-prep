//! Minimal authorized connectivity check. Never print response text or API credentials.
use swe_interview_prep::credentials;
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let key = credentials::read_key("deepseek")?;
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|_| "client")?;
    for model in ["deepseek-flash", "deepseek-v4-pro"] {
        let body = serde_json::json!({"model":model,"max_tokens":32,"thinking":{"type":"disabled"},"messages":[{"role":"user","content":"Reply OK."}]});
        let r = client
            .post("https://api.deepseek.com/chat/completions")
            .bearer_auth(&key)
            .json(&body)
            .send()
            .map_err(|e| {
                if e.is_timeout() {
                    "API 请求超时"
                } else {
                    "API 网络错误"
                }
            })?;
        let status = r.status().as_u16();
        let content_type = r.headers().get("content-type").and_then(|h|h.to_str().ok()).unwrap_or("missing").to_owned();
        let bytes = r.bytes().map_err(|_|"读取 API 响应失败")?;
        let parsed = serde_json::from_slice::<serde_json::Value>(&bytes);
        let valid_json = parsed.is_ok();
        let value = parsed.unwrap_or(serde_json::Value::Null);
        let succeeded = status == 200
            && value["choices"][0]["message"]["content"]
                .as_str()
                .is_some_and(|s| !s.is_empty());
        println!("{model}: HTTP {status}, valid completion = {succeeded}, content_type={content_type}, response_bytes={}, json={valid_json}, choices={}, content_bytes={}, reasoning_bytes={}, finish_stop={}, finish_length={}", bytes.len(), value["choices"].as_array().map_or(0,|v|v.len()),value["choices"][0]["message"]["content"].as_str().map_or(0,str::len),value["choices"][0]["message"]["reasoning_content"].as_str().map_or(0,str::len),value["choices"][0]["finish_reason"]=="stop",value["choices"][0]["finish_reason"]=="length");
    }
    Ok(())
}
