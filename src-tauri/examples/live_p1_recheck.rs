//! Explicit P1 acceptance: DeepSeek only, scripted content, isolated storage, bounded HTTP attempts.
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
    interview::{self, Lang, LlmProvider},
    interview_store as store, llm, usage,
};
const REPORT: &str = "/tmp/intv-p1-practical-recheck.json";
const DATABASE: &str = "/tmp/intv-p1-practical-live.sqlite";
fn api(
    lang: Lang,
    session: Option<&str>,
    operation: &str,
    cap: Arc<AtomicUsize>,
) -> Result<llm::InterviewProvider, String> {
    let config = config();
    Ok(
        llm::InterviewProvider::new(&config, credentials::read_key("deepseek")?)?
            .lang(lang)
            .record_usage(usage::Context {
                path: DATABASE.into(),
                module: "p1".into(),
                session_id: session.map(str::to_owned),
                operation: operation.into(),
            })
            .before_request(move || loop {
                let used = cap.load(Ordering::SeqCst);
                if used >= 48 {
                    return Err("Live P1 HTTP cap reached".into());
                }
                if cap
                    .compare_exchange(used, used + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    return Ok(());
                }
            }),
    )
}
fn config() -> LlmSettings {
    LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 90,
    }
}
fn persist(samples: &[Value], cap: &Arc<AtomicUsize>) -> Result<(), String> {
    let c = db::connect(Path::new(DATABASE))?;
    let report = json!({"provider":"deepseek","model":"deepseek-flash","policy":swe_interview_prep::quality::POLICY_VERSION,
        "httpAttempts":cap.load(Ordering::SeqCst),"httpCap":60,"samples":samples,"usage":usage::overview(&c,None,None)?,
        "scope":"Prepared content only; no personal history, drafts or credentials. Transport completion is not semantic acceptance."});
    std::fs::write(
        REPORT,
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
fn retry_once<T>(
    label: &str,
    events: &mut Vec<Value>,
    mut call: impl FnMut() -> Result<T, String>,
) -> Result<T, String> {
    match call() {
        Err(error)
            if error.contains("无法连接")
                || error.contains("超时")
                || error.contains("HTTP 503") =>
        {
            events.push(json!({"stage":label,"firstFailure":error,"action":"One explicitly authorized probe retry; product behavior is unchanged"}));
            call()
        }
        result => result,
    }
}
fn resume_feedback() -> Result<(), String> {
    let mut report: Value =
        serde_json::from_str(&std::fs::read_to_string(REPORT).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let backup = "/tmp/intv-p1-practical-recheck-before-resume.json";
    if Path::new(backup).exists() {
        return Err("This explicit feedback continuation already ran".into());
    }
    std::fs::copy(REPORT, backup).map_err(|e| e.to_string())?;
    let cap = Arc::new(AtomicUsize::new(
        report["httpAttempts"].as_u64().ok_or("Missing count")? as usize,
    ));
    let c = db::connect(Path::new(DATABASE))?;
    let sample = report["samples"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|s| s["topicId"] == "mysql-index-structure")
        .unwrap();
    let sid = sample["sessionId"]
        .as_str()
        .ok_or("Missing saved session")?
        .to_owned();
    let before = store::session(&c, &sid)?;
    if before.status != "feedback_pending" {
        return Err("No pending feedback to resume".into());
    }
    let mut retries = vec![];
    let result = retry_once("resume_saved_feedback", &mut retries, || {
        store::retry_pending(&c, &sid, &api(Lang::Zh, Some(&sid), "retry", cap.clone())?)
    });
    sample["priorFailure"] = sample["error"].clone();
    sample["feedbackResume"] = json!({"retries":retries,"error":result.as_ref().err()});
    sample["session"] = json!(store::session(&c, &sid)?);
    if result.is_ok() {
        sample["status"] = json!("completed");
        sample.as_object_mut().unwrap().remove("error");
    }
    let samples = report["samples"].as_array().unwrap().clone();
    persist(&samples, &cap)?;
    println!(
        "Saved feedback continuation; {} cumulative scripted HTTP attempts",
        cap.load(Ordering::SeqCst)
    );
    result.map(|_| ())
}
fn main() -> Result<(), String> {
    if std::env::args().any(|arg| arg == "--resume-feedback") {
        return resume_feedback();
    }
    if Path::new(REPORT).exists() {
        return Err("Refusing to overwrite an earlier recheck report".into());
    }
    let original: Value = serde_json::from_str(
        &std::fs::read_to_string("/tmp/intv-p1-practical-live.json").map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let c = db::open(Path::new(DATABASE))?;
    let cap = Arc::new(AtomicUsize::new(
        original["httpAttempts"]
            .as_u64()
            .ok_or("Missing earlier attempt count")? as usize,
    ));
    let mut samples = vec![];
    let cases=[
        ("mysql-index-structure",Lang::Zh,"B+ 树一个节点可容纳较多索引项，较高扇出使树更矮，一次查找需要访问较少的页。等值查找沿键范围向下定位叶子；范围查询定位起点后可以沿叶子顺序遍历，避免每个结果都重新从根查找。实际耗时还与缓存命中和数据分布有关。"),
        ("network-tcp-reliability",Lang::En,"TCP presents an ordered byte stream using sequence numbers, acknowledgements and retransmission. That does not prove an application transaction committed: a transport acknowledgement and a business response belong to different layers. After a timeout, I would treat the business result as unknown until an application-level result or status lookup establishes it."),
        ("business-payment-callback",Lang::Zh,"先按支付机构约定验证回调真实性，再以稳定的支付流水或业务事件标识去重。入账明细的唯一约束与状态更新放进同一个本地事务，只有事务提交后才确认处理成功。相同事件重试返回既有结果，不能靠先查再写的无保护流程避免并发重复。"),
        ("javascript-event-loop",Lang::En,""),
        ("redis-cache-failures",Lang::Zh,"仅凭命中率下降和数据库请求增加还不能认定原因。我会先对比异常时段未命中键的分布，并抽样确认数据库查询是否有结果；随后把请求时间与相关键的过期记录对应起来。应使用受控采样和已有监控，不在生产执行阻塞全库扫描；取得证据前保留多种假设。"),
        ("sql-explain-slow-query",Lang::En,"I would inspect the query plan, actual row counts and the time spent scanning or waiting, using the supplied query and schema. I would compare estimates against actual cardinality and check recent data or statistics changes. A slow query alone is not proof that an index is missing, and I would avoid an unbounded production diagnostic query."),
    ];
    for (id, lang, answer) in cases {
        if ["network-tcp-reliability", "business-payment-callback"].contains(&id) {
            continue;
        }
        println!("Starting real P1 recheck: {id} {lang:?}");
        let mut sample = json!({"topicId":id,"lang":lang,"status":"in_progress"});
        let mut events = vec![];
        let result: Result<(), String> = (|| {
            if id == "redis-cache-failures" {
                let old = original["samples"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|s| s["topicId"] == id)
                    .unwrap();
                if let Some(sid) = old["sessionId"].as_str() {
                    let before = store::session(&c, sid)?;
                    if before.status == "awaiting_reply" {
                        let after = retry_once("recover_existing_follow_up", &mut events, || {
                            store::retry_pending(
                                &c,
                                sid,
                                &api(lang, Some(sid), "retry", cap.clone())?,
                            )
                        })?;
                        let count = |s: &interview::SessionView| {
                            s.turns
                                .iter()
                                .filter(|turn| turn.role == "candidate")
                                .count()
                        };
                        if count(&before) != count(&after) {
                            return Err("Pending retry duplicated a submitted answer".into());
                        }
                        sample["pendingRecovery"] = json!({"sessionId":sid,"before":before.status,"after":after.status,"candidateTurnsBefore":count(&before),"candidateTurnsAfter":count(&after)});
                    }
                }
            }
            let topic = interview::topic(id)?;
            sample["topic"] = json!(topic.localized(lang));
            let q = retry_once("generation", &mut events, || {
                api(lang, None, "generation", cap.clone())?.generate_question(&topic)
            })?;
            sample["question"] = json!(q);
            let cached = store::cache_question(&c, id, &config(), &q)?;
            let session = store::start(&c, &cached.id, &config())?;
            let sid = &session.id;
            sample["sessionId"] = json!(sid);
            if id == "javascript-event-loop" {
                let reference = retry_once("reference", &mut events, || {
                    store::generate_reference(
                        &c,
                        sid,
                        0,
                        &api(lang, Some(sid), "reference", cap.clone())?,
                    )
                })?;
                sample["reference"] = json!(reference);
                sample["referencePreviewDidNotSubmit"] =
                    json!(store::session(&c, sid)?.turns.is_empty());
                // The answer is already persisted on failure: retry pending, never resubmit.
                if let Err(error) = store::submit_reference(
                    &c,
                    sid,
                    0,
                    &api(lang, Some(sid), "follow_up", cap.clone())?,
                ) {
                    events.push(json!({"stage":"assisted_follow_up","firstFailure":error}));
                    retry_once("assisted_follow_up_retry", &mut events, || {
                        store::retry_pending(&c, sid, &api(lang, Some(sid), "retry", cap.clone())?)
                    })?;
                }
            } else {
                store::save_draft(&c, sid, answer)?;
                store::begin_answer(&c, sid, answer)?;
                retry_once("follow_up", &mut events, || {
                    store::retry_pending(&c, sid, &api(lang, Some(sid), "follow_up", cap.clone())?)
                })?;
                if ["redis-cache-failures", "sql-explain-slow-query"].contains(&id) {
                    let index = store::session(&c, sid)?.turns.len();
                    let before = store::session(&c, sid)?;
                    let reference = retry_once("reference", &mut events, || {
                        store::generate_reference(
                            &c,
                            sid,
                            index,
                            &api(lang, Some(sid), "reference", cap.clone())?,
                        )
                    })?;
                    let after = store::session(&c, sid)?;
                    sample["reference"] = json!(reference);
                    sample["referencePreviewDidNotSubmit"] = json!(
                        before.turns.len() == after.turns.len() && before.draft == after.draft
                    );
                    if id == "redis-cache-failures" {
                        if let Err(error) = store::submit_reference(
                            &c,
                            sid,
                            index,
                            &api(lang, Some(sid), "follow_up", cap.clone())?,
                        ) {
                            events.push(json!({"stage":"mixed_follow_up","firstFailure":error}));
                            retry_once("mixed_follow_up_retry", &mut events, || {
                                store::retry_pending(
                                    &c,
                                    sid,
                                    &api(lang, Some(sid), "retry", cap.clone())?,
                                )
                            })?;
                        }
                    }
                }
            }
            if [
                "mysql-index-structure",
                "javascript-event-loop",
                "redis-cache-failures",
            ]
            .contains(&id)
            {
                store::begin_feedback(&c, sid)?;
                retry_once("feedback", &mut events, || {
                    store::retry_pending(&c, sid, &api(lang, Some(sid), "feedback", cap.clone())?)
                })?;
            }
            sample["session"] = json!(store::session(&c, sid)?);
            let reopened = db::connect(Path::new(DATABASE))?;
            let offline = store::start(&reopened, &cached.id, &config())?;
            sample["offlineReuse"] =
                json!({"sessionId":offline.id,"seedId":offline.question.seed_id});
            Ok(())
        })();
        sample["status"] = json!(if result.is_ok() {
            "completed"
        } else {
            "failed"
        });
        if let Err(error) = result {
            sample["error"] = json!(error);
            if let Some(sid) = sample["sessionId"].as_str() {
                sample["session"] = json!(store::session(&c, sid)?);
            }
        }
        sample["retries"] = json!(events);
        println!("Finished recheck {id}: {}", sample["status"]);
        samples.push(sample);
        persist(&samples, &cap)?;
    }
    println!(
        "Saved {REPORT}; {} cumulative HTTP attempts (12 UI attempts reserved)",
        cap.load(Ordering::SeqCst)
    );
    if samples.iter().any(|s| s["status"] != "completed") {
        return Err("Some rechecks failed; inspect sanitized report".into());
    }
    Ok(())
}
