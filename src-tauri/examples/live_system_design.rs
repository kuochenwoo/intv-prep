//! Explicit P2 live acceptance: real provider calls, isolated database, no credential writes.
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::{Lang, Validated},
    llm::InterviewProvider,
    system_design::*,
    system_design_store as store,
};
fn main() -> Result<(), String> {
    let provider = std::env::var("INTERVIEW_PROVIDER").unwrap_or_else(|_| "claude".into());
    let model = std::env::var("INTERVIEW_MODEL").unwrap_or_else(|_| {
        if provider == "deepseek" {
            "deepseek-flash".into()
        } else {
            "claude-haiku-5-5".into()
        }
    });
    let config = LlmSettings {
        provider,
        model,
        timeout_seconds: 120,
    };
    let lang = if std::env::args().any(|s| s == "--english") {
        Lang::En
    } else {
        Lang::Zh
    };
    let db_path =
        std::env::var("INTERVIEW_DB_PATH").unwrap_or_else(|_| "/tmp/intv-p2-live.sqlite3".into());
    let path = std::path::Path::new(&db_path);
    let c = db::open(path)?;
    let api = InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(lang);
    if std::env::args().any(|s| s == "--feedback-only") {
        let raw =
            std::fs::read_to_string("/tmp/intv-p2-live-report.json").map_err(|e| e.to_string())?;
        let previous: DesignSession = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        let feedback = api.design_feedback(&previous)?;
        feedback.validate()?;
        let report = serde_json::to_string_pretty(&feedback).map_err(|e| e.to_string())?;
        std::fs::write("/tmp/intv-p2-feedback-review.json", &report).map_err(|e| e.to_string())?;
        println!("Rechecked real eight-dimension feedback: {report}");
        return Ok(());
    }
    let q = api.generate_design(&scenario("url-shortener")?, lang)?;
    println!("Generated real system design task: {}", q.question);
    let q = store::cache_question(&c, "url-shortener", lang, &config, q)?;
    let mut s = store::start(&c, &q.id, &config)?;
    let answers = if lang == Lang::Zh {
        [
  "先确认 MVP 只需创建、重定向、自定义别名和过期。不做实时统计。题设规模作为目标；跳转优先可用性，新链接写成功后应可读，禁用链接不能无限期从缓存返回。请确认链接保留期与峰值持续时间。",
  "下面先用明确的训练假设演示估算：每日新增100万、读写100:1、峰值为平均10倍、每条200字节、保留365天。若与题设数字不同应重新代入。写入平均1000000/86400约11.6 QPS，跳转平均1157 QPS，峰值跳转约11574 QPS。裸数据每年约73GB，三副本约219GB，索引、备份和余量需额外计入。带宽需再假设响应大小。",
  "POST /links 创建链接，GET /{code} 跳转。负载均衡后为无状态API服务，关系数据库表存code唯一键、原URL、状态和过期时间；Redis只缓存热门链接。创建时生成随机code并用唯一约束处理碰撞，数据库提交成功再响应。跳转先缓存再数据库，查到记录后验证状态和过期。后台异步处理统计，统计失败不阻塞跳转。",
  "热点短码由缓存承担，miss使用请求合并，Redis不可用时回源需要限流保护数据库，超额明确失败。禁用链接提交后发失效事件，事件可重试；缓存设置短TTL作为遗漏失效的兜底，承认存在短暂陈旧。若禁用必须立即生效，应单独权威校验并接受可用性与延迟代价。数据库副本延迟可能影响新建后立即跳转，可短期读主库。",
  "方案A是关系库加缓存，当前规模低写高读，事务唯一约束直观、运维简单；方案B是分片KV加独立ID服务，写入容量更高但要处理分片、碰撞、失效传播和ID服务故障。先用A并验证热key、故障回源和主库写瓶颈；达到实测容量或保留数据增长后再迁移B。多区域主动写入提高地域可用性，但会增加冲突和禁用一致性难度，不直接宣称全部目标兼得。"
 ]
    } else {
        [
  "MVP supports creation, redirects, custom aliases and expiry; realtime analytics is out of scope. Prioritize redirect availability, define read-after-create and a bounded revocation delay. Confirm retention and peak duration before fixing estimates.",
  "For an explicit illustrative assumption of 1 million creates/day, read/write 100:1, peak factor 10, 200 bytes/record and 365 days: average writes are 1e6/86400=11.6 QPS, reads 1157 QPS and peak reads 11574 QPS. Raw annual storage is 73GB; three replicas need 219GB before indexes, backups and headroom. Substitute the scenario numbers if different; bandwidth needs response-size assumptions.",
  "POST /links creates records; GET /{code} redirects. A load balancer fronts stateless API instances. A relational table stores unique code, URL, state and expiry; Redis caches hot records. Random codes use a unique constraint and retry collisions. Acknowledge creation after database commit. Redirect reads cache then database and checks state and expiry. Analytics is asynchronous and does not block redirects.",
  "Coalesce misses and rate-limit database fallback when Redis fails. Revocation commits trigger retriable invalidation; short TTL bounds missed invalidation staleness. Immediate revocation requires authoritative checks with latency and availability costs. Replica lag affects read-after-create; temporarily use primary reads for new links.",
  "A relational store plus cache fits low-write high-read load with simple uniqueness and operations. A sharded KV plus ID service raises write capacity but adds partitioning, ID availability and invalidation complexity. Start with the former, measure failover and database bottlenecks, then migrate when needed. Multi-region writes improve regional availability but complicate conflicts and revocation consistency."
 ]
    };
    for (stage, answer) in [
        Stage::Requirements,
        Stage::Estimation,
        Stage::HighLevel,
        Stage::DeepDive,
        Stage::Tradeoffs,
    ]
    .into_iter()
    .zip(answers)
    {
        s = store::change_stage(&c, &s.id, stage)?;
        if stage == Stage::HighLevel {
            let mut w = s.workspace.clone();
            w.notes=if lang==Lang::Zh{"## 架构\n方案A：关系库 + Redis；方案B：分片KV + ID服务。\n\n核心数据流：客户端 → API → 缓存/数据库。"}else{"## Architecture\nAlternative A: relational database + Redis. Alternative B: sharded KV + ID service.\n\nFlow: client → API → cache/database."}.into();
            w.diagram =
                "flowchart LR\n Client-->API\n API-->Cache\n API-->Database\n API-->Queue".into();
            store::save_workspace(&c, &s.id, w)?;
        }
        store::begin_answer(&c, &s.id, answer)?;
        s = store::retry_pending(&c, &s.id, &api)?;
        println!(
            "Stage {:?}; suggestion {:?}; follow-up: {}",
            s.stage,
            s.suggested_stage,
            s.turns.last().unwrap().text
        );
    }
    store::begin_feedback(&c, &s.id)?;
    let finished = store::retry_pending(&c, &s.id, &api)?;
    if finished.status != "finished" || finished.feedback.as_ref().unwrap().assessment.len() != 8 {
        return Err("Live feedback incomplete".into());
    }
    std::fs::write(
        "/tmp/intv-p2-live-report.json",
        serde_json::to_string_pretty(&finished).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    drop(c);
    let c = db::open(path)?;
    let restored = store::session(&c, &finished.id)?;
    let reused = store::start(&c, &restored.question.id, &config)?;
    println!("Real P2 interview complete: eight dimensions, restored notes and {} turns; offline reuse {}; report /tmp/intv-p2-live-report.json",restored.turns.len(),reused.id);
    Ok(())
}
