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
const REPORT: &str = "/tmp/intv-p1-practical-live.json";
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
                if used >= 60 {
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
fn main() -> Result<(), String> {
    if Path::new(DATABASE).exists() {
        return Err("Refusing to overwrite an earlier live acceptance database".into());
    }
    let c = db::open(Path::new(DATABASE))?;
    let cap = Arc::new(AtomicUsize::new(0));
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
        println!("Starting real P1 sample: {id} {lang:?}");
        let mut sample = json!({"topicId":id,"lang":lang,"status":"in_progress","steps":[]});
        let result: Result<(), String> = (|| {
            let topic = interview::topic(id)?;
            sample["topic"] = json!(topic.localized(lang));
            let q = api(lang, None, "generation", cap.clone())?.generate_question(&topic)?;
            sample["question"] = json!(q);
            let cached = store::cache_question(&c, id, &config(), &q)?;
            let session = store::start(&c, &cached.id, &config())?;
            sample["sessionId"] = json!(session.id);
            if id == "javascript-event-loop" {
                let reference = store::generate_reference(
                    &c,
                    &session.id,
                    0,
                    &api(lang, Some(&session.id), "reference", cap.clone())?,
                )?;
                sample["reference"] = json!(reference);
                let preview = store::session(&c, &session.id)?;
                if !preview.turns.is_empty() || !preview.draft.is_empty() {
                    return Err("Reference preview unexpectedly submitted an answer".into());
                }
                sample["referencePreviewDidNotSubmit"] = json!(true);
                store::submit_reference(
                    &c,
                    &session.id,
                    0,
                    &api(lang, Some(&session.id), "follow_up", cap.clone())?,
                )?;
            } else {
                store::save_draft(&c, &session.id, answer)?;
                store::begin_answer(&c, &session.id, answer)?;
                store::retry_pending(
                    &c,
                    &session.id,
                    &api(lang, Some(&session.id), "follow_up", cap.clone())?,
                )?;
                if id == "redis-cache-failures" {
                    let index = store::session(&c, &session.id)?.turns.len();
                    let reference = store::generate_reference(
                        &c,
                        &session.id,
                        index,
                        &api(lang, Some(&session.id), "reference", cap.clone())?,
                    )?;
                    sample["reference"] = json!(reference);
                    store::submit_reference(
                        &c,
                        &session.id,
                        index,
                        &api(lang, Some(&session.id), "follow_up", cap.clone())?,
                    )?;
                }
                if id == "sql-explain-slow-query" {
                    let index = store::session(&c, &session.id)?.turns.len();
                    let before = store::session(&c, &session.id)?;
                    let reference = store::generate_reference(
                        &c,
                        &session.id,
                        index,
                        &api(lang, Some(&session.id), "reference", cap.clone())?,
                    )?;
                    let after = store::session(&c, &session.id)?;
                    if before.turns.len() != after.turns.len() || before.draft != after.draft {
                        return Err("Reference preview modified the interview".into());
                    }
                    sample["reference"] = json!(reference);
                    sample["referencePreviewDidNotSubmit"] = json!(true);
                }
            }
            if [
                "mysql-index-structure",
                "javascript-event-loop",
                "redis-cache-failures",
            ]
            .contains(&id)
            {
                store::begin_feedback(&c, &session.id)?;
                store::retry_pending(
                    &c,
                    &session.id,
                    &api(lang, Some(&session.id), "feedback", cap.clone())?,
                )?;
            }
            sample["session"] = json!(store::session(&c, &session.id)?);
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
            if let Some(id) = sample["sessionId"].as_str() {
                sample["session"] = json!(store::session(&c, id)?);
            }
        }
        println!("Finished {id}: {}", sample["status"]);
        samples.push(sample);
        persist(&samples, &cap)?;
    }
    println!(
        "Saved {REPORT}; {} HTTP attempts",
        cap.load(Ordering::SeqCst)
    );
    if samples.iter().any(|sample| sample["status"] != "completed") {
        return Err("Some real P1 samples failed; inspect the sanitized report".into());
    }
    Ok(())
}
