//! Explicit live acceptance: AI answers the original question and a followup, then gets feedback.
//! Reuses a real cached question in an isolated test database, always starts a new test session.
use swe_interview_prep::{credentials, db, interview_store as store, llm::InterviewProvider};
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/intv-p1-live-claude-final.sqlite3".into());
    let c = db::open(std::path::Path::new(&path))?;
    let question = store::questions(&c)?
        .into_iter()
        .find(|q| q.provider == "claude")
        .ok_or("请先运行真实面试验收，保存一题 Claude 场景题")?;
    let config = credentials::LlmSettings {
        provider: question.provider.clone(),
        model: question.model.clone(),
        timeout_seconds: 120,
    };
    let api = InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?;
    let mut s = if let Ok(id) = std::env::var("INTERVIEW_SESSION") {
        store::session(&c, &id)?
    } else {
        store::start(&c, &question.id, &config)?
    };
    println!(
        "真实 AI 代答测试会话：{}；{} / {}",
        s.id, config.provider, config.model
    );
    if s.status == "awaiting_reply" {
        s = store::retry_pending(&c, &s.id, &api)?;
        println!(
            "已恢复待处理追问，未重复追加 AI 代答；现有 {} 条消息",
            s.turns.len()
        );
    }
    while s.turns.iter().filter(|t| t.role == "candidate").count() < 2 {
        let round = s.turns.len() / 2;
        s = store::submit_reference(&c, &s.id, s.turns.len(), &api)?;
        let answer = &s.turns[s.turns.len() - 2];
        let followup = s.turns.last().ok_or("缺少追问")?;
        if answer.role != "candidate" || !answer.ai_assisted || followup.role != "interviewer" {
            return Err("AI 代答或后续追问未正确保存".into());
        }
        println!(
            "第 {} 轮真实代答已提交：{} 字符；下一轮追问：{}",
            round + 1,
            answer.text.chars().count(),
            followup.text
        );
    }
    if s.status != "finished" {
        if s.status == "active" {
            store::begin_feedback(&c, &s.id)?;
        }
        s = store::retry_pending(&c, &s.id, &api)?;
    }
    println!(
        "真实反馈：{}",
        s.feedback.as_ref().ok_or("缺少反馈")?.summary
    );
    drop(c);
    let c = db::open(std::path::Path::new(&path))?;
    let restored = store::session(&c, &s.id)?;
    if restored.status != "finished"
        || restored.turns.len() != 4
        || restored.turns.iter().filter(|t| t.ai_assisted).count() != 2
        || restored.references.len() != 2
    {
        return Err("重开后 AI 代答或会话状态不一致".into());
    }
    println!("已验证原题和追问的自动代答→提交→继续追问、五维反馈及重开恢复。测试数据库：{path}");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("真实 AI 代答验收未完成：{e}");
        std::process::exit(1);
    }
}
