//! Last bounded continuation of the explicitly approved P1 recheck; no generation or history writes.
use serde_json::{json, Value};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::{Lang, LlmProvider},
    interview_store as store, llm, usage,
};
fn main() -> Result<(), String> {
    let path = Path::new("/tmp/intv-p1-practical-live.sqlite");
    let output = "/tmp/intv-p1-foundation-feedback-final.json";
    if Path::new(output).exists() {
        return Err("This approved continuation already ran".into());
    }
    let c = db::connect(path)?;
    let used = usage::overview(&c, None, None)?["records"]
        .as_array()
        .ok_or("Missing attempt ledger")?
        .len();
    let cap = Arc::new(AtomicUsize::new(used));
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 90,
    };
    let source: Value = serde_json::from_str(
        &std::fs::read_to_string("/tmp/intv-p1-practical-recheck.json")
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let sample = source["samples"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["topicId"] == "mysql-index-structure")
        .unwrap();
    let sid = sample["sessionId"].as_str().unwrap();
    let session = store::session(&c, sid)?;
    let question = store::question(&c, &session.question.id)?.1;
    let mut failures = vec![];
    let mut result = None;
    for _ in 0..2 {
        let guard = cap.clone();
        let api = llm::InterviewProvider::new(&config, credentials::read_key("deepseek")?)?
            .lang(Lang::Zh)
            .record_usage(usage::Context {
                path: path.into(),
                module: "p1".into(),
                session_id: Some(sid.into()),
                operation: "foundation_feedback_recheck".into(),
            })
            .before_request(move || loop {
                let n = guard.load(Ordering::SeqCst);
                if n >= 48 {
                    return Err("Approved scripted HTTP cap reached (48)".into());
                }
                if guard
                    .compare_exchange(n, n + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    return Ok(());
                }
            });
        match api.summarize(&question, &session.turns) {
            Ok(feedback) => {
                result = Some(feedback);
                break;
            }
            Err(error) => {
                let transport = error.contains("无法连接") || error.contains("超时");
                failures.push(error);
                if !transport {
                    break;
                }
            }
        }
    }
    let report = json!({"topicId":"mysql-index-structure","policy":swe_interview_prep::quality::POLICY_VERSION,"provider":"deepseek","model":"deepseek-flash","question":question,"turns":session.turns,"priorFeedback":session.feedback,"feedback":result,"failures":failures,"cumulativeScriptedHttpAttempts":cap.load(Ordering::SeqCst),"usage":usage::overview(&c,None,None)?,"scope":"Prepared existing sample only; does not rewrite stored scores or personal history"});
    std::fs::write(
        output,
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "Saved final foundation feedback check; {} cumulative scripted HTTP attempts",
        cap.load(Ordering::SeqCst)
    );
    if result.is_none() {
        return Err("Foundation feedback recheck failed; see preserved report".into());
    }
    Ok(())
}
