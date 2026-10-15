use crate::{credentials::LlmSettings, interview::*};
use rusqlite::{params, Connection, OptionalExtension};
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch("PRAGMA foreign_keys=ON;
    CREATE TABLE IF NOT EXISTS generated_questions (
        id TEXT PRIMARY KEY, seed_id TEXT NOT NULL, provider TEXT NOT NULL, model TEXT NOT NULL,
        prompt_version TEXT NOT NULL, content_json TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')));
    CREATE TABLE IF NOT EXISTS interview_sessions (
        id TEXT PRIMARY KEY, question_id TEXT NOT NULL REFERENCES generated_questions(id),
        provider TEXT NOT NULL, model TEXT NOT NULL,
        status TEXT NOT NULL CHECK(status IN ('active','awaiting_reply','feedback_pending','finished')),
        draft TEXT NOT NULL DEFAULT '', last_error TEXT, should_end INTEGER NOT NULL DEFAULT 0,
        created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')));
    CREATE TABLE IF NOT EXISTS interview_turns (
        id INTEGER PRIMARY KEY, session_id TEXT NOT NULL REFERENCES interview_sessions(id) ON DELETE CASCADE,
        role TEXT NOT NULL CHECK(role IN ('interviewer','candidate')), text TEXT NOT NULL,
        turn_index INTEGER NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
        UNIQUE(session_id,turn_index));
    CREATE TABLE IF NOT EXISTS interview_feedback (
        session_id TEXT PRIMARY KEY REFERENCES interview_sessions(id) ON DELETE CASCADE,
        feedback_json TEXT NOT NULL);").map_err(err)
}
fn new_id(c: &Connection) -> Result<String, String> {
    c.query_row("SELECT lower(hex(randomblob(16)))", [], |r| r.get(0))
        .map_err(err)
}
pub fn cache_question(
    c: &Connection,
    seed_id: &str,
    config: &LlmSettings,
    q: &GeneratedQuestion,
) -> Result<QuestionView, String> {
    topic(seed_id)?;
    config.validate()?;
    q.validate()?;
    let id = new_id(c)?;
    c.execute("INSERT INTO generated_questions(id,seed_id,provider,model,prompt_version,content_json) VALUES(?1,?2,?3,?4,'deep-dive-v1',?5)",params![id,seed_id,config.provider,config.model,serde_json::to_string(q).map_err(err)?]).map_err(err)?;
    question(c, &id).map(|r| r.0)
}
pub fn question(c: &Connection, id: &str) -> Result<(QuestionView, GeneratedQuestion), String> {
    let (seed_id,provider,model,raw,created_at):(String,String,String,String,String)=c.query_row("SELECT seed_id,provider,model,content_json,created_at FROM generated_questions WHERE id=?1",[id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).optional().map_err(err)?.ok_or("缓存题目不存在")?;
    let q: GeneratedQuestion = serde_json::from_str(&raw).map_err(|_| "本地题目数据损坏")?;
    q.validate()?;
    Ok((
        QuestionView {
            id: id.into(),
            seed_id,
            provider,
            model,
            question: q.question.clone(),
            background: q.background.clone(),
            constraints: q.constraints.clone(),
            created_at,
        },
        q,
    ))
}
pub fn questions(c: &Connection) -> Result<Vec<QuestionView>, String> {
    let mut s = c
        .prepare("SELECT id FROM generated_questions ORDER BY created_at DESC,rowid DESC")
        .map_err(err)?;
    let ids = s
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(err)?;
    ids.iter().map(|id| question(c, id).map(|r| r.0)).collect()
}
pub fn start(
    c: &Connection,
    question_id: &str,
    config: &LlmSettings,
) -> Result<SessionView, String> {
    config.validate()?;
    question(c, question_id)?;
    let id = new_id(c)?;
    c.execute("INSERT INTO interview_sessions(id,question_id,provider,model,status) VALUES(?1,?2,?3,?4,'active')",params![id,question_id,config.provider,config.model]).map_err(err)?;
    session(c, &id)
}
pub fn session(c: &Connection, id: &str) -> Result<SessionView, String> {
    let mut s=c.query_row("SELECT question_id,provider,model,status,draft,last_error,should_end,created_at FROM interview_sessions WHERE id=?1",[id],|r|Ok(SessionView{id:id.into(),question:QuestionView{id:r.get(0)?,seed_id:String::new(),provider:String::new(),model:String::new(),question:String::new(),background:String::new(),constraints:vec![],created_at:String::new()},provider:r.get(1)?,model:r.get(2)?,status:r.get(3)?,draft:r.get(4)?,last_error:r.get(5)?,should_end:r.get(6)?,turns:vec![],feedback:None,created_at:r.get(7)?})).optional().map_err(err)?.ok_or("面试会话不存在")?;
    s.question = question(c, &s.question.id)?.0;
    let mut stmt=c.prepare("SELECT role,text,created_at FROM interview_turns WHERE session_id=?1 ORDER BY turn_index").map_err(err)?;
    s.turns = stmt
        .query_map([id], |r| {
            Ok(Turn {
                role: r.get(0)?,
                text: r.get(1)?,
                timestamp: r.get(2)?,
            })
        })
        .map_err(err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(err)?;
    let raw: Option<String> = c
        .query_row(
            "SELECT feedback_json FROM interview_feedback WHERE session_id=?1",
            [id],
            |r| r.get(0),
        )
        .optional()
        .map_err(err)?;
    s.feedback = raw
        .map(|r| serde_json::from_str(&r).map_err(err))
        .transpose()?;
    Ok(s)
}
pub fn sessions(c: &Connection) -> Result<Vec<SessionSummary>, String> {
    let mut stmt=c.prepare("SELECT s.id,q.seed_id,s.status,s.provider,s.model,s.created_at,q.content_json FROM interview_sessions s JOIN generated_questions q ON q.id=s.question_id ORDER BY s.created_at DESC,s.rowid DESC").map_err(err)?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })
        .map_err(err)?;
    rows.map(|r| {
        let (id, seed_id, status, provider, model, created_at, raw) = r.map_err(err)?;
        let q: GeneratedQuestion = serde_json::from_str(&raw).map_err(err)?;
        Ok(SessionSummary {
            id,
            seed_id,
            title: q.question,
            status,
            provider,
            model,
            created_at,
        })
    })
    .collect()
}
pub fn save_draft(c: &Connection, id: &str, draft: &str) -> Result<(), String> {
    if draft.chars().count() > 6000 {
        return Err("回答最多 6000 字符".into());
    }
    let changed = c
        .execute(
            "UPDATE interview_sessions SET draft=?2 WHERE id=?1 AND status='active'",
            params![id, draft],
        )
        .map_err(err)?;
    if changed != 1 {
        return Err("会话不存在或正在等待模型，请先重试待处理请求".into());
    }
    Ok(())
}
fn add_turn(c: &Connection, id: &str, role: &str, text: &str) -> Result<(), String> {
    c.execute("INSERT INTO interview_turns(session_id,role,text,turn_index) SELECT ?1,?2,?3,COALESCE(MAX(turn_index),-1)+1 FROM interview_turns WHERE session_id=?1",params![id,role,text]).map_err(err)?;
    Ok(())
}
pub fn begin_answer(c: &Connection, id: &str, answer: &str) -> Result<(), String> {
    if answer.trim().is_empty() || answer.chars().count() > 6000 {
        return Err("回答需要 1–6000 字符".into());
    }
    let tx = c.unchecked_transaction().map_err(err)?;
    let s = session(&tx, id)?;
    if s.status != "active" {
        return Err("当前已有待处理请求或会话已结束，请使用重试".into());
    }
    if s.turns.iter().filter(|t| t.role == "candidate").count() >= 20 {
        return Err("已达 20 轮上限，请结束并生成反馈".into());
    }
    add_turn(&tx, id, "candidate", answer)?;
    tx.execute("UPDATE interview_sessions SET status='awaiting_reply',draft=?2,last_error=NULL WHERE id=?1",params![id,answer]).map_err(err)?;
    tx.commit().map_err(err)
}
pub fn begin_feedback(c: &Connection, id: &str) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(err)?;
    let s = session(&tx, id)?;
    if s.status != "active" {
        return Err("请先完成待处理追问或反馈".into());
    }
    if !s.draft.trim().is_empty() {
        return Err("有尚未提交的回答，请先提交或清空草稿".into());
    }
    if !s.turns.iter().any(|t| t.role == "candidate") {
        return Err("至少提交一轮回答后再生成反馈".into());
    }
    tx.execute(
        "UPDATE interview_sessions SET status='feedback_pending',last_error=NULL WHERE id=?1",
        [id],
    )
    .map_err(err)?;
    tx.commit().map_err(err)
}
pub fn retry_pending(
    c: &Connection,
    id: &str,
    provider: &impl LlmProvider,
) -> Result<SessionView, String> {
    let s = session(c, id)?;
    let q = question(c, &s.question.id)?.1;
    let outcome: Result<(), String> = (|| {
        match s.status.as_str() {
            "awaiting_reply" => {
                let next = provider.next_turn(&q, &s.turns)?;
                next.validate()?;
                let tx = c.unchecked_transaction().map_err(err)?;
                add_turn(&tx, id, "interviewer", &next.follow_up_question)?;
                tx.execute("UPDATE interview_sessions SET status='active',draft='',last_error=NULL,should_end=?2 WHERE id=?1",params![id,next.should_end]).map_err(err)?;
                tx.commit().map_err(err)?;
            }
            "feedback_pending" => {
                let feedback = provider.summarize(&q, &s.turns)?;
                feedback.validate()?;
                let tx = c.unchecked_transaction().map_err(err)?;
                tx.execute(
                    "INSERT INTO interview_feedback VALUES(?1,?2)",
                    params![id, serde_json::to_string(&feedback).map_err(err)?],
                )
                .map_err(err)?;
                tx.execute(
                    "UPDATE interview_sessions SET status='finished',last_error=NULL WHERE id=?1",
                    [id],
                )
                .map_err(err)?;
                tx.commit().map_err(err)?;
            }
            _ => return Err("会话没有待重试请求".into()),
        }
        Ok(())
    })();
    if let Err(e) = outcome {
        c.execute(
            "UPDATE interview_sessions SET last_error=?2 WHERE id=?1",
            params![id, e],
        )
        .map_err(err)?;
        return Err(e);
    }
    session(c, id)
}
pub fn delete_session(c: &Connection, id: &str) -> Result<(), String> {
    c.execute("DELETE FROM interview_sessions WHERE id=?1", [id])
        .map_err(err)?;
    Ok(())
}
pub fn clear_history(c: &Connection) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(err)?;
    tx.execute("DELETE FROM interview_sessions", [])
        .map_err(err)?;
    tx.execute("DELETE FROM generated_questions", [])
        .map_err(err)?;
    tx.commit().map_err(err)
}
