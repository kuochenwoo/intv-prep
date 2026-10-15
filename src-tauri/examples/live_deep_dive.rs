//! Explicit live acceptance test: real provider calls; never prints credentials.
//! Optional first argument is the local SQLite path (defaults to a /tmp test DB).
use swe_interview_prep::{
    credentials, db,
    interview::{self, LlmProvider},
    interview_store as store,
    llm::DeepSeekProvider,
};
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/intv-p1-live.sqlite3".into());
    let c = db::open(std::path::Path::new(&path))?;
    let key = credentials::read_key("deepseek")?;
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
            .map_err(|_| "无法连接 DeepSeek 模型列表")?;
        if !response.status().is_success() {
            return Err(format!(
                "DeepSeek 模型列表 HTTP {}（未输出响应体或密钥）",
                response.status().as_u16()
            ));
        }
        let models: serde_json::Value = response.json().map_err(|_| "模型列表 JSON 无效")?;
        let ids: Vec<&str> = models["data"]
            .as_array()
            .ok_or("模型列表缺少 data")?
            .iter()
            .filter_map(|v| v["id"].as_str())
            .collect();
        println!("账户可用模型：{}", ids.join(", "));
        return Ok(());
    }
    let mut config = credentials::settings(&c)?;
    if let Ok(model) = std::env::var("INTERVIEW_MODEL") {
        config.model = model;
    }
    config.timeout_seconds = 120;
    let api = DeepSeekProvider::new(&config, key)?;
    let question = api.generate_question(&interview::topic("cache-selection")?)?;
    let cached = store::cache_question(&c, "cache-selection", &config, &question)?;
    println!("真实生成并缓存题目：{}", cached.id);
    let s = store::start(&c, &cached.id, &config)?;
    let answers=["我会先澄清读写比例和能否接受短暂陈旧。初步采用 Cache Aside，数据库为准，缓存用于降低读压力。", "如果是多实例部署，我倾向 Redis 共享缓存；本地缓存可能造成版本不一致。我会设置超时、请求合并与限流，Redis 故障时有保护地回源，不直接让所有请求压向数据库。"];
    for answer in answers {
        store::save_draft(&c, &s.id, answer)?;
        store::begin_answer(&c, &s.id, answer)?;
        let s = store::retry_pending(&c, &s.id, &api)?;
        println!("真实追问：{}", s.turns.last().ok_or("missing turn")?.text);
    }
    store::begin_feedback(&c, &s.id)?;
    let s = store::retry_pending(&c, &s.id, &api)?;
    let feedback = s.feedback.ok_or("missing feedback")?;
    println!(
        "真实反馈完成：{}；{} 个维度；{} 条会话消息",
        feedback.summary,
        feedback.dimensions.len(),
        s.turns.len()
    );
    drop(c);
    let c = db::open(std::path::Path::new(&path))?;
    let restored = store::session(&c, &s.id)?;
    if restored.status != "finished" || restored.feedback.is_none() {
        return Err("重开数据库恢复失败".into());
    }
    let _offline = store::start(&c, &cached.id, &config)?;
    println!("已验证重开恢复与离线缓存复用；测试数据位于 {path}");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("真实 API 验收未完成：{e}");
        std::process::exit(1);
    }
}
