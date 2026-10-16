//! Explicit library acceptance: scripted seeds only, no voice, isolated usage, bounded fallback.
use serde_json::{json, Value};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc, Mutex,
    },
};
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::{self, Lang, LlmProvider},
    llm, usage,
};
const DATABASE: &str = "/tmp/intv-p1-library-expansion-20261016.sqlite";
const REPORT: &str = "/tmp/intv-p1-library-expansion-20261016.json";
const CAP: usize = 180;
const CASES: &[&str] = &[
    "db-unique-race",
    "db-window-functions",
    "db-long-transaction",
    "redis-replication-ack",
    "redis-watch-transaction",
    "network-flow-congestion",
    "network-cdn-cache",
    "os-zombie-process",
    "os-fd-exhaustion",
    "concurrency-bounded-buffer",
    "mq-offset-commit",
    "mq-exactly-once",
    "distributed-clock",
    "distributed-quorum-read",
    "security-object-access",
    "ds-amortized-growth",
    "engineering-container-isolation",
    "engineering-health-probes",
    "engineering-git-recovery",
    "storage-lsm-compaction",
    "business-coupon-redeem",
    "runtime-shallow-copy",
    "runtime-async-blocking",
    "cache-selection",
    "idempotency-retries",
    "database-concurrency",
    "replication-failover",
    "sharding-growth",
    "rpc-tail-latency",
    "redis-cache-failures",
];
fn api_failure(error: &str) -> bool {
    error.contains("HTTP ")
        || [
            "无法连接 API",
            "超时",
            "读取 API 响应失败",
            "API 流式响应失败",
            "服务暂时不可用",
        ]
        .iter()
        .any(|s| error.contains(s))
}
fn main() -> Result<(), String> {
    if std::env::args().any(|a| a == "--seed-recheck") {
        return recheck(2);
    }
    if std::env::args().any(|a| a == "--scope-recheck") {
        return recheck(1);
    }
    if std::env::args().any(|a| a == "--recheck") {
        return recheck(0);
    }
    if Path::new(DATABASE).exists() || Path::new(REPORT).exists() {
        return Err(
            "Refusing to rerun or overwrite this paid batch; choose a newly authorized batch"
                .into(),
        );
    }
    drop(db::open(Path::new(DATABASE))?);
    let cap = Arc::new(AtomicUsize::new(0));
    let fallback = Arc::new(AtomicBool::new(false));
    let next = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    std::thread::scope(|scope| {
        for _ in 0..3 {
            let (cap, fallback, next, tx) =
                (cap.clone(), fallback.clone(), next.clone(), tx.clone());
            scope.spawn(move || loop {
                let index = next.fetch_add(1, Ordering::SeqCst);
                let Some(id) = CASES.get(index) else { break };
                let lang = if index % 2 == 0 { Lang::Zh } else { Lang::En };
                let topic = interview::topic(id).expect("bundled acceptance seed");
                let mut attempts = vec![];
                let mut result = None;
                for _ in 0..2 {
                    let provider = if fallback.load(Ordering::SeqCst) { "claude" } else { "deepseek" };
                    let config = LlmSettings { provider:provider.into(), model:if provider == "claude" {"claude-haiku-5-5"} else {"deepseek-flash"}.into(), timeout_seconds:90 };
                    let reviews = Arc::new(Mutex::new(Vec::<Value>::new()));
                    let trace = reviews.clone();
                    let guard = cap.clone();
                    let outcome = (|| {
                        let api = llm::InterviewProvider::new(&config, credentials::read_key(provider)?)?
                            .lang(lang)
                            .record_usage(usage::Context { path:DATABASE.into(), module:"p1".into(), session_id:None, operation:format!("library_generation:{id}") })
                            .on_review(move |proposal, verdict| trace.lock().unwrap().push(json!({"proposal":proposal,"verdict":verdict})))
                            .before_request(move || loop {
                                let used=guard.load(Ordering::SeqCst);
                                if used>=CAP { return Err("Library acceptance HTTP cap reached".into()); }
                                if guard.compare_exchange(used,used+1,Ordering::SeqCst,Ordering::SeqCst).is_ok() { return Ok(()); }
                            });
                        api.generate_question(&topic)
                    })();
                    let error = outcome.as_ref().err().cloned();
                    attempts.push(json!({"provider":provider,"model":config.model,"error":error,"reviews":*reviews.lock().unwrap()}));
                    match outcome {
                        Ok(question) => { result=Some(question); break; },
                        Err(error) if provider=="deepseek" && api_failure(&error) => { fallback.store(true,Ordering::SeqCst); },
                        Err(_) => break,
                    }
                }
                let row=json!({"index":index,"topicId":id,"lang":lang,"topic":topic.localized(lang),"attempts":attempts,"question":result,"status":if result.is_some(){"generated_and_reviewed"}else{"failed"}});
                if tx.send(row).is_err() { break; }
            });
        }
        drop(tx);
        let mut samples = vec![];
        for sample in rx {
            println!(
                "{} {} {}",
                sample["topicId"], sample["lang"], sample["status"]
            );
            samples.push(sample);
            samples.sort_by_key(|s| s["index"].as_u64().unwrap());
            let c = db::connect(Path::new(DATABASE)).expect("isolated usage database");
            let report = json!({"policy":swe_interview_prep::quality::POLICY_VERSION,"httpAttempts":cap.load(Ordering::SeqCst),"httpCap":CAP,"plannedSamples":CASES.len(),"samples":samples,"usage":usage::overview(&c,None,None).expect("usage ledger"),"scope":"P1 seed generation only. No personal history, drafts, source code, voice calls or application configuration writes. Provider review is not independent manual certification."});
            std::fs::write(REPORT, serde_json::to_string_pretty(&report).unwrap())
                .expect("preserve incremental report");
        }
    });
    println!(
        "Saved {REPORT}; {} HTTP attempts",
        cap.load(Ordering::SeqCst)
    );
    Ok(())
}

fn recheck(mode: u8) -> Result<(), String> {
    let output = if mode == 2 {
        "/tmp/intv-p1-library-expansion-seed-recheck-20261016.json"
    } else if mode == 1 {
        "/tmp/intv-p1-library-expansion-scope-recheck-20261016.json"
    } else {
        "/tmp/intv-p1-library-expansion-recheck-20261016.json"
    };
    if Path::new(output).exists() {
        return Err("This bounded recheck already ran".into());
    }
    let original: Value =
        serde_json::from_str(&std::fs::read_to_string(REPORT).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if original["samples"].as_array().map(Vec::len) != Some(CASES.len()) {
        return Err("Initial batch is not complete".into());
    }
    let c = db::connect(Path::new(DATABASE))?;
    let used = usage::overview(&c, None, None)?["records"]
        .as_array()
        .ok_or("Missing usage ledger")?
        .len();
    let cap = Arc::new(AtomicUsize::new(used));
    let mut provider =
        if original["samples"].as_array().unwrap().iter().any(|s| {
            s["attempts"].as_array().unwrap().iter().any(|a| {
                a["provider"] == "deepseek" && a["error"].as_str().is_some_and(api_failure)
            })
        }) {
            "claude"
        } else {
            "deepseek"
        };
    let mut rows = vec![];
    let cases = if mode == 2 {
        vec![
            ("replication-failover", Lang::Zh, false),
            ("replication-failover", Lang::En, false),
            ("rpc-tail-latency", Lang::Zh, false),
            ("rpc-tail-latency", Lang::En, false),
        ]
    } else if mode == 1 {
        vec![
            ("concurrency-bounded-buffer", Lang::En, true),
            ("replication-failover", Lang::Zh, true),
            ("rpc-tail-latency", Lang::Zh, true),
            ("network-cdn-cache", Lang::Zh, true),
            ("db-long-transaction", Lang::Zh, true),
            ("concurrency-bounded-buffer", Lang::En, false),
            ("replication-failover", Lang::Zh, false),
            ("rpc-tail-latency", Lang::Zh, false),
            ("network-cdn-cache", Lang::Zh, false),
            ("db-long-transaction", Lang::Zh, false),
            ("db-long-transaction", Lang::En, false),
        ]
    } else {
        vec![
            ("db-unique-race", Lang::Zh, true),
            ("db-long-transaction", Lang::Zh, true),
            ("db-unique-race", Lang::Zh, false),
            ("db-unique-race", Lang::En, false),
            ("db-long-transaction", Lang::Zh, false),
            ("db-long-transaction", Lang::En, false),
        ]
    };
    for (id, lang, negative) in cases {
        let topic = interview::topic(id)?;
        let mut row = json!({"topicId":id,"lang":lang,"negativeProbe":negative,"attempts":[]});
        for _ in 0..2 {
            let guard = cap.clone();
            let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
            let sink = trace.clone();
            let config = LlmSettings {
                provider: provider.into(),
                model: if provider == "claude" {
                    "claude-haiku-5-5"
                } else {
                    "deepseek-flash"
                }
                .into(),
                timeout_seconds: 90,
            };
            let outcome: Result<Value, String> = (|| {
                let api = llm::InterviewProvider::new(&config, credentials::read_key(provider)?)?
                    .lang(lang)
                    .record_usage(usage::Context {
                        path: DATABASE.into(),
                        module: "p1".into(),
                        session_id: None,
                        operation: format!("library_recheck:{id}"),
                    })
                    .on_review(move |proposal, verdict| {
                        sink.lock()
                            .unwrap()
                            .push(json!({"proposal":proposal,"verdict":verdict}))
                    })
                    .before_request(move || loop {
                        let n = guard.load(Ordering::SeqCst);
                        if n >= CAP {
                            return Err("Library acceptance HTTP cap reached".into());
                        }
                        if guard
                            .compare_exchange(n, n + 1, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            return Ok(());
                        }
                    });
                if negative {
                    let previous = original["samples"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|s| s["topicId"] == id)
                        .ok_or("Missing original failure")?;
                    api.review_quality(
                        lang,
                        "fundamentals_question",
                        json!({"topic":topic.localized(lang)}),
                        previous["question"].clone(),
                    )
                    .map(|v| json!({"verdict":v}))
                } else {
                    api.generate_question(&topic).map(|v| json!({"question":v}))
                }
            })();
            row["attempts"].as_array_mut().unwrap().push(json!({"provider":provider,"model":config.model,"error":outcome.as_ref().err(),"reviews":*trace.lock().unwrap()}));
            match outcome {
                Ok(value) => {
                    row["result"] = value;
                    break;
                }
                Err(error) if provider == "deepseek" && api_failure(&error) => provider = "claude",
                Err(_) => break,
            }
        }
        println!("Rechecked {id} {lang:?} negative={negative}");
        rows.push(row);
        let report = json!({"policy":swe_interview_prep::quality::POLICY_VERSION,"httpCap":CAP,"cumulativeHttpAttempts":cap.load(Ordering::SeqCst),"initialHttpAttempts":used,"samples":rows,"usage":usage::overview(&c,None,None)?,"scope":if mode == 2 {"Final seed clarification within the same 180-attempt batch: four bilingual openings; no voice or personal history."} else if mode == 1 {"Continuation within the same 180-attempt library batch: five preserved scope probes and six corrected openings; no voice or user history."} else {"Continuation within the same 180-attempt library batch: two preserved negative probes and four bilingual corrected openings; no voice or user history."}});
        std::fs::write(output, serde_json::to_string_pretty(&report).unwrap())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
