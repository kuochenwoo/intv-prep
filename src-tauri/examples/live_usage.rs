//! Explicitly authorized minimal live usage check; never reads interview history or settings.
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::{self, Lang, LlmProvider},
    llm, usage,
};
fn main() -> Result<(), String> {
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-chat".into(),
        timeout_seconds: 90,
    };
    let path = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("review/session-features-usage.sqlite");
    let c = db::open(&path)?;
    // Evaluation rates only; these deliberately illustrative rates are not current provider prices.
    usage::save_rates(
        &c,
        "deepseek",
        "deepseek-chat",
        &usage::Rates {
            input: 1.0,
            output: 2.0,
            cache_read: 0.1,
            cache_write: 0.0,
        },
    )?;
    let count = Arc::new(AtomicUsize::new(0));
    let cap = count.clone();
    let api = llm::InterviewProvider::new(&config, credentials::read_key("deepseek")?)?
        .lang(Lang::En)
        .record_usage(usage::Context {
            path: path.clone(),
            module: "p1".into(),
            session_id: None,
            operation: "generation".into(),
        })
        .before_request(move || {
            if cap.fetch_add(1, Ordering::SeqCst) >= 12 {
                Err("Live evaluation HTTP cap reached".into())
            } else {
                Ok(())
            }
        });
    let result = api.generate_question(&interview::topic("javascript-event-loop")?);
    let report = serde_json::json!({"provider":"deepseek","model":"deepseek-chat","status":if result.is_ok(){"success"}else{"failed"},"error":result.as_ref().err(),"question":result.ok(),"usage":usage::overview(&c,None,None)?,"priceNote":"Illustrative test rates only; not provider prices or billed amounts"});
    std::fs::write(
        "review/session-features-live-usage.json",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "Live usage report written; {} recorded HTTP attempts",
        report["usage"]["records"].as_array().unwrap().len()
    );
    if report["status"] != "success" {
        return Err("Real provider check failed; see sanitized report".into());
    }
    Ok(())
}
