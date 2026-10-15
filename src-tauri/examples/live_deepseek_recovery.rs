//! Real HTTP authentication failure followed by real provider recovery, with an isolated DB.
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db, interview_store as store,
    llm::InterviewProvider,
};
fn main() -> Result<(), String> {
    let path = std::path::Path::new("/tmp/intv-reliability-deepseek.sqlite3");
    let c = db::open(path)?;
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 120,
    };
    let q = store::questions(&c)?
        .into_iter()
        .find(|q| q.provider == "deepseek")
        .ok_or("Run the real DeepSeek deep-dive probe first")?;
    let s = store::start(&c, &q.id, &config)?;
    let answer="B 删除缓存时 key 还不存在，A 已读到 v1 且尚未回填。删除只作用于当时已存在的值，不会取消 A；A 之后仍可把 v1 回填，所以旧值残留。";
    store::save_draft(&c, &s.id, answer)?;
    store::begin_answer(&c, &s.id, answer)?;
    // This intentionally invalid credential is synthetic, not a replacement for a stored key.
    let invalid = InterviewProvider::new(
        &config,
        "reliability-deliberately-invalid-credential".into(),
    )?;
    let error = store::retry_pending(&c, &s.id, &invalid)
        .err()
        .ok_or("Invalid credential unexpectedly succeeded")?;
    if !error.contains("HTTP 401") && !error.contains("HTTP 403") {
        return Err(format!(
            "Expected a real authentication error, got: {error}"
        ));
    }
    drop(c);
    let c = db::open(path)?;
    let pending = store::session(&c, &s.id)?;
    if pending.status != "awaiting_reply" || pending.draft != answer || pending.turns.len() != 1 {
        return Err("Pending answer failed to survive restart".into());
    }
    let real = InterviewProvider::new(&config, credentials::read_key("deepseek")?)?;
    let recovered = store::retry_pending(&c, &s.id, &real)?;
    if recovered.status != "active"
        || recovered.turns.len() != 2
        || recovered
            .turns
            .iter()
            .filter(|t| t.role == "candidate")
            .count()
            != 1
    {
        return Err("Recovery duplicated or lost the candidate answer".into());
    }
    store::begin_feedback(&c, &s.id)?;
    let finished = store::retry_pending(&c, &s.id, &real)?;
    if finished.feedback.as_ref().map_or(0, |f| f.dimensions.len()) != 5 {
        return Err("Real recovery feedback incomplete".into());
    }
    println!("Verified real HTTP 401/403, durable pending answer after reopen, successful DeepSeek retry without duplicate submission, and five-dimension feedback. Stored credentials were not modified.");
    Ok(())
}
