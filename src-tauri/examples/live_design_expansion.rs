//! Explicit P2 content probe: eight real requests, scripted answers, isolated storage.
//! Reads existing .env credentials without writing keys or application history.
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::Lang,
    llm::InterviewProvider,
    system_design::*,
    system_design_store as store,
};
fn main() -> Result<(), String> {
    let provider = std::env::var("INTERVIEW_PROVIDER").unwrap_or_else(|_| "claude".into());
    let model = std::env::var("INTERVIEW_MODEL").unwrap_or_else(|_| {
        if provider == "deepseek" {
            "deepseek-flash"
        } else {
            "claude-haiku-5-5"
        }
        .into()
    });
    let config = LlmSettings {
        provider,
        model,
        timeout_seconds: 120,
    };
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let path = dir.path().join("p2-expansion.sqlite3");
    let c = db::open(&path)?;
    let api =
        InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(Lang::Zh);
    let generated = api.generate_design(&scenario("file-sync")?, Lang::Zh)?;
    let cached = store::cache_question(&c, "file-sync", Lang::Zh, &config, generated)?;
    let mut session = store::start(&c, &cached.id, &config)?;
    let answers = [
        (Stage::Requirements, "这是脚本化验收回答。MVP 为文件上传下载、断点续传、多端同步、版本历史与分享撤销，不做协同文本编辑。确认成功的版本必须持久化；离线并发修改生成冲突版本，不静默覆盖。先明确撤权时限、恢复点和跨区域故障范围，再决定同步复制边界。"),
        (Stage::Estimation, "用明确的示例假设估算：100万日活，每人每天变更5个文件，每个2 MB，十进制 MB，不先计去重收益。每天500万次变更，平均57.9次/秒、10 TB原始写入，平均115.7 MB/s。若峰值为5倍则约289次/秒、579 MB/s。30天全部保留为300 TB，三副本900 TB，尚未含当前文件长期存量、索引、日志、备份、元数据和下载带宽。若题设采用不同值应重新代入；增量传输需先测变更比例。"),
        (Stage::HighLevel, "客户端经鉴权API创建上传会话，分块直传对象存储并校验内容哈希；所有块达到约定持久化条件后，元数据服务用文件基准版本的条件更新发布manifest并追加变更日志，提交成功再确认。设备按游标补拉变化并下载缺块，推送只作提示。关系库保存租户、文件版本、权限和游标，消息队列异步驱动通知、索引与垃圾回收。"),
        (Stage::DeepDive, "两个离线设备基于同一版本提交时，只允许第一个条件更新成功，第二个保存可见冲突副本。上传会话有过期状态，但垃圾回收必须确认块没有已提交manifest引用并留安全窗口；不能因超时就删除正在提交的块。缓存不能作为权限真相；分享撤销查权威状态或使用明确短时授权，承认已下载的副本无法收回。同步日志可重放，设备幂等应用，游标过旧时重新拉快照。"),
        (Stage::Tradeoffs, "先用单区域跨可用区的元数据事务和对象存储，简化版本发布与冲突检测；跨区域异步复制可降低写延迟，但灾难后可能丢最近确认写入，必须声明RPO而非许诺零损失。若必须区域级RPO为零，需同步确认或其他等价协议并接受延迟和分区下可用性代价。按租户分片比按文件全局均匀分片便于权限事务，但大租户可能成热点。跨租户内容去重有信息泄漏风险，优先租户内去重。"),
    ];
    for (stage, answer) in answers {
        session = store::change_stage(&c, &session.id, stage)?;
        if stage == Stage::HighLevel {
            let mut workspace = session.workspace.clone();
            workspace.notes = "## Scripted acceptance architecture\nClient -> API -> metadata transaction/change log; client -> object store. Notifications are advisory.".into();
            workspace.diagram = "flowchart LR\n Client-->API\n Client-->Objects\n API-->Metadata\n Metadata-->Log\n Log-->Notifications".into();
            store::save_workspace(&c, &session.id, workspace)?;
        }
        store::begin_answer(&c, &session.id, answer)?;
        session = store::retry_pending(&c, &session.id, &api)?;
        if session.stage != stage {
            return Err("Model unexpectedly changed the stage".into());
        }
        println!("Completed real file-sync stage: {stage:?}");
    }
    store::begin_feedback(&c, &session.id)?;
    let finished = store::retry_pending(&c, &session.id, &api)?;
    if finished.status != "finished"
        || finished.feedback.as_ref().map(|f| f.assessment.len()) != Some(8)
    {
        return Err("Eight-dimension feedback incomplete".into());
    }
    // Save the first completed sample before making the independent English request.
    let report_path = "/tmp/intv-p2-expansion-live.json";
    std::fs::write(
        report_path,
        serde_json::to_string_pretty(&serde_json::json!({"fileSync":finished})).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    drop(c);
    let c = db::open(&path)?;
    let restored = store::session(&c, &finished.id)?;
    let reused = store::start(&c, &cached.id, &config)?;
    if reused.question.seed_id != "file-sync" || restored.workspace.diagram.is_empty() {
        return Err("Offline restore/reuse failed".into());
    }
    let english_api =
        InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(Lang::En);
    let metrics = english_api.generate_design(&scenario("metrics-platform")?, Lang::En)?;
    let metrics = store::cache_question(&c, "metrics-platform", Lang::En, &config, metrics)?;
    std::fs::write(
        report_path,
        serde_json::to_string_pretty(
            &serde_json::json!({"fileSync":finished,"metricsEnglish":metrics}),
        )
        .unwrap(),
    )
    .map_err(|e| e.to_string())?;
    println!("Completed eight real requests, bilingual generation, all five stages, feedback and offline restore/reuse; report {report_path}");
    Ok(())
}
