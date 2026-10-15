//! Explicit live acceptance test: real provider calls; never prints credentials.
//! Optional first argument is the local SQLite path (defaults to a /tmp test DB).
use swe_interview_prep::{
    credentials, db,
    interview::{self, LlmProvider},
    interview_store as store,
    llm::InterviewProvider,
};
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/intv-p1-live.sqlite3".into());
    let c = db::open(std::path::Path::new(&path))?;
    let mut config = credentials::settings(&c)?;
    if let Ok(provider) = std::env::var("INTERVIEW_PROVIDER") {
        config.provider = provider;
    }
    let key = credentials::read_key(&config.provider)?;
    if std::env::var_os("INTERVIEW_MODELS_ONLY").is_some() {
        let client = reqwest::blocking::Client::builder()
            .https_only(true)
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(|_| "HTTPS client")?;
        let response = client
            .get("https://api.deepseek.com/models")
            .bearer_auth(&key)
            .send()
            .map_err(|_| "Unable to fetch the DeepSeek model list")?;
        if !response.status().is_success() {
            return Err(format!(
                "DeepSeek model list: HTTP {} (response body and key omitted)",
                response.status().as_u16()
            ));
        }
        let models: serde_json::Value = response.json().map_err(|_| "Invalid model-list JSON")?;
        let ids: Vec<&str> = models["data"]
            .as_array()
            .ok_or("Model list is missing data")?
            .iter()
            .filter_map(|v| v["id"].as_str())
            .collect();
        println!("Available account models: {}", ids.join(", "));
        return Ok(());
    }
    if let Ok(model) = std::env::var("INTERVIEW_MODEL") {
        config.model = model;
    }
    config.timeout_seconds = 120;
    credentials::save_settings(&c, &config)?;
    println!("Live API: {} / {}", config.provider, config.model);
    let api = InterviewProvider::new(&config, key)?;
    if std::env::var_os("INTERVIEW_RESUME").is_some() {
        let pending = store::sessions(&c)?
            .into_iter()
            .find(|s| s.status == "feedback_pending" || s.status == "awaiting_reply")
            .ok_or("No live session is waiting for retry")?;
        let retried = store::retry_pending(&c, &pending.id, &api)?;
        println!(
            "Live retry succeeded: status {}; {} messages; {} feedback dimensions",
            retried.status,
            retried.turns.len(),
            retried.feedback.as_ref().map_or(0, |f| f.dimensions.len())
        );
        drop(c);
        let c = db::open(std::path::Path::new(&path))?;
        let restored = store::session(&c, &retried.id)?;
        if restored.status != "finished" || restored.feedback.is_none() {
            return Err("Feedback is not complete yet".into());
        }
        let _offline = store::start(&c, &restored.question.id, &config)?;
        println!("Verified restoration and offline cache reuse; test database: {path}");
        return Ok(());
    }
    let question = api.generate_question(&interview::topic("cache-selection")?)?;
    let cached = store::cache_question(&c, "cache-selection", &config, &question)?;
    println!("Generated and cached a real question: {}", cached.id);
    let s = store::start(&c, &cached.id, &config)?;
    let answers=["我会先澄清读写比例和能否接受短暂陈旧。初步采用 Cache Aside，数据库为准，缓存用于降低读压力。", "如果是多实例部署，我倾向 Redis 共享缓存；本地缓存可能造成版本不一致。我会设置超时、请求合并与限流，Redis 故障时有保护地回源，不直接让所有请求压向数据库。"];
    for answer in answers {
        store::save_draft(&c, &s.id, answer)?;
        store::begin_answer(&c, &s.id, answer)?;
        let s = store::retry_pending(&c, &s.id, &api)?;
        println!(
            "Live follow-up: {}",
            s.turns.last().ok_or("missing turn")?.text
        );
    }
    store::begin_feedback(&c, &s.id)?;
    let s = store::retry_pending(&c, &s.id, &api)?;
    let feedback = s.feedback.ok_or("missing feedback")?;
    println!(
        "Live feedback completed: {}; {} dimensions; {} conversation messages",
        feedback.summary,
        feedback.dimensions.len(),
        s.turns.len()
    );
    drop(c);
    let c = db::open(std::path::Path::new(&path))?;
    let restored = store::session(&c, &s.id)?;
    if restored.status != "finished" || restored.feedback.is_none() {
        return Err("Database restoration failed".into());
    }
    let _offline = store::start(&c, &cached.id, &config)?;
    println!("Verified restoration and offline cache reuse; test database: {path}");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Live API acceptance incomplete: {e}");
        std::process::exit(1);
    }
}
