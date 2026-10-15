//! Independent P2 persistence. Each session is an atomic JSON document; pending work is durable.
use crate::{
    credentials::LlmSettings,
    interview::{Lang, Validated},
    system_design::*,
};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::BTreeMap;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS sd_questions(id TEXT PRIMARY KEY, prompt_version TEXT NOT NULL, content_json TEXT NOT NULL);
    CREATE TABLE IF NOT EXISTS sd_sessions(id TEXT PRIMARY KEY, question_id TEXT NOT NULL REFERENCES sd_questions(id), content_json TEXT NOT NULL);") .map_err(err)
}
fn identity(c: &Connection) -> Result<(String, String), String> {
    c.query_row(
        "SELECT lower(hex(randomblob(16))),strftime('%Y-%m-%dT%H:%M:%fZ','now')",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .map_err(err)
}
fn put(c: &Connection, s: &DesignSession) -> Result<(), String> {
    c.execute(
        "UPDATE sd_sessions SET content_json=?2 WHERE id=?1",
        params![s.id, serde_json::to_string(s).map_err(err)?],
    )
    .map_err(err)?;
    Ok(())
}
pub fn cache_question(
    c: &Connection,
    seed_id: &str,
    lang: Lang,
    config: &LlmSettings,
    content: DesignQuestion,
) -> Result<CachedQuestion, String> {
    scenario(seed_id)?;
    config.validate()?;
    content.validate()?;
    let (id, created_at) = identity(c)?;
    let q = CachedQuestion {
        id,
        created_at,
        seed_id: seed_id.into(),
        lang,
        provider: config.provider.clone(),
        model: config.model.clone(),
        content,
    };
    c.execute(
        "INSERT INTO sd_questions VALUES(?1,'system-design-v1',?2)",
        params![q.id, serde_json::to_string(&q).map_err(err)?],
    )
    .map_err(err)?;
    Ok(q)
}
pub fn question(c: &Connection, id: &str) -> Result<CachedQuestion, String> {
    let raw: String = c
        .query_row(
            "SELECT content_json FROM sd_questions WHERE id=?1",
            [id],
            |r| r.get(0),
        )
        .optional()
        .map_err(err)?
        .ok_or("系统设计缓存题目不存在")?;
    let q: CachedQuestion = serde_json::from_str(&raw).map_err(|_| "系统设计本地数据损坏")?;
    q.content.validate()?;
    Ok(q)
}
pub fn questions(c: &Connection) -> Result<Vec<CachedQuestion>, String> {
    let mut stmt = c
        .prepare("SELECT id FROM sd_questions ORDER BY rowid DESC")
        .map_err(err)?;
    let ids = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(err)?;
    ids.iter().map(|id| question(c, id)).collect()
}
pub fn session(c: &Connection, id: &str) -> Result<DesignSession, String> {
    let raw: String = c
        .query_row(
            "SELECT content_json FROM sd_sessions WHERE id=?1",
            [id],
            |r| r.get(0),
        )
        .optional()
        .map_err(err)?
        .ok_or("系统设计会话不存在")?;
    serde_json::from_str(&raw).map_err(|_| "系统设计本地数据损坏".into())
}
pub fn sessions(c: &Connection) -> Result<Vec<DesignSummary>, String> {
    let mut stmt = c
        .prepare("SELECT content_json FROM sd_sessions ORDER BY rowid DESC")
        .map_err(err)?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0)).map_err(err)?;
    rows.map(|r| {
        let s: DesignSession =
            serde_json::from_str(&r.map_err(err)?).map_err(|_| "系统设计本地数据损坏")?;
        Ok(DesignSummary {
            id: s.id,
            title: s.question.content.question,
            stage: s.stage,
            status: s.status,
            lang: s.question.lang,
            created_at: s.created_at,
        })
    })
    .collect()
}
pub fn start(c: &Connection, id: &str, config: &LlmSettings) -> Result<DesignSession, String> {
    config.validate()?;
    let q = question(c, id)?;
    let (id, created_at) = identity(c)?;
    let lang = q.lang;
    let s = DesignSession {
        id,
        created_at,
        question: q,
        provider: config.provider.clone(),
        model: config.model.clone(),
        stage: Stage::Requirements,
        status: "active".into(),
        workspace: Workspace {
            drafts: BTreeMap::new(),
            notes: String::new(),
            diagram: String::new(),
        },
        turns: vec![DesignTurn {
            role: "interviewer".into(),
            text: Stage::Requirements.guide(lang).into(),
            stage: Stage::Requirements,
            observation: None,
        }],
        suggested_stage: None,
        last_error: None,
        feedback: None,
    };
    c.execute(
        "INSERT INTO sd_sessions VALUES(?1,?2,?3)",
        params![s.id, s.question.id, serde_json::to_string(&s).map_err(err)?],
    )
    .map_err(err)?;
    Ok(s)
}
fn active(s: &DesignSession) -> Result<(), String> {
    if s.status != "active" {
        Err("系统设计会话有待处理请求或已结束".into())
    } else {
        Ok(())
    }
}
fn check_workspace(w: &Workspace) -> Result<(), String> {
    if w.notes.chars().count() > 20000
        || w.diagram.chars().count() > 12000
        || w.drafts.len() > 5
        || w.drafts
            .iter()
            .any(|(k, v)| *k == Stage::Feedback || v.chars().count() > 6000)
    {
        Err("系统设计草稿超出限制".into())
    } else {
        Ok(())
    }
}
pub fn save_workspace(c: &Connection, id: &str, w: Workspace) -> Result<(), String> {
    check_workspace(&w)?;
    let tx = c.unchecked_transaction().map_err(err)?;
    let mut s = session(&tx, id)?;
    active(&s)?;
    s.workspace = w;
    put(&tx, &s)?;
    tx.commit().map_err(err)
}
pub fn change_stage(c: &Connection, id: &str, stage: Stage) -> Result<DesignSession, String> {
    if stage == Stage::Feedback {
        return Err("请通过结束面试生成系统设计反馈".into());
    }
    let tx = c.unchecked_transaction().map_err(err)?;
    let mut s = session(&tx, id)?;
    active(&s)?;
    if s.stage != stage {
        if s.turns.len() >= 100 {
            return Err("系统设计会话已达长度上限，请结束面试".into());
        }
        s.stage = stage;
        s.suggested_stage = None;
        s.turns.push(DesignTurn {
            role: "interviewer".into(),
            text: stage.guide(s.question.lang).into(),
            stage,
            observation: None,
        });
        put(&tx, &s)?;
    }
    tx.commit().map_err(err)?;
    Ok(s)
}
pub fn begin_answer(c: &Connection, id: &str, answer: &str) -> Result<(), String> {
    if answer.trim().is_empty() || answer.chars().count() > 6000 {
        return Err("回答需要 1–6000 字符".into());
    }
    let tx = c.unchecked_transaction().map_err(err)?;
    let mut s = session(&tx, id)?;
    active(&s)?;
    if s.turns.len() + 2 > 100
        || s.turns
            .iter()
            .map(|t| t.text.chars().count())
            .sum::<usize>()
            + answer.chars().count()
            > 120000
    {
        return Err("系统设计会话已达长度上限，请结束面试".into());
    }
    s.turns.push(DesignTurn {
        role: "candidate".into(),
        text: answer.into(),
        stage: s.stage,
        observation: None,
    });
    s.workspace.drafts.insert(s.stage, answer.into());
    s.status = "awaiting_reply".into();
    s.last_error = None;
    put(&tx, &s)?;
    tx.commit().map_err(err)
}
pub fn begin_feedback(c: &Connection, id: &str) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(err)?;
    let mut s = session(&tx, id)?;
    active(&s)?;
    if s.workspace.drafts.values().any(|d| !d.trim().is_empty()) {
        return Err("系统设计仍有未提交的阶段草稿，请先提交或清空".into());
    }
    s.status = "feedback_pending".into();
    s.last_error = None;
    put(&tx, &s)?;
    tx.commit().map_err(err)
}
pub fn retry_pending(
    c: &Connection,
    id: &str,
    api: &impl DesignProvider,
) -> Result<DesignSession, String> {
    let s = session(c, id)?;
    // Never hold a SQLite transaction across the network call. The desktop serializes
    // model requests; compare the persisted state again before committing the result.
    let outcome: Result<DesignSession, String> = (|| {
        let (follow, feedback) = match s.status.as_str() {
            "awaiting_reply" => {
                let f = api.design_follow_up(&s)?;
                f.validate()?;
                (Some(f), None)
            }
            "feedback_pending" => {
                let f = api.design_feedback(&s)?;
                f.validate()?;
                (None, Some(f))
            }
            _ => return Err("系统设计会话没有待重试请求".into()),
        };
        let tx = c.unchecked_transaction().map_err(err)?;
        let mut current = session(&tx, id)?;
        if current.status != s.status || current.turns.len() != s.turns.len() {
            return Err("系统设计会话状态已变化，请刷新".into());
        }
        if let Some(f) = follow {
            current.turns.push(DesignTurn {
                role: "interviewer".into(),
                text: f.follow_up_question,
                stage: current.stage,
                observation: Some(f.observation),
            });
            current.suggested_stage = Some(f.suggested_stage);
            current.workspace.drafts.remove(&current.stage);
            current.status = "active".into();
        }
        if let Some(f) = feedback {
            current.feedback = Some(f);
            current.status = "finished".into();
            current.stage = Stage::Feedback;
            current.suggested_stage = None;
        }
        current.last_error = None;
        put(&tx, &current)?;
        tx.commit().map_err(err)?;
        Ok(current)
    })();
    if let Err(ref e) = outcome {
        let mut current = session(c, id)?;
        if current.status == s.status {
            current.last_error = Some(e.clone());
            put(c, &current)?;
        }
    }
    outcome
}
pub fn delete_session(c: &Connection, id: &str) -> Result<(), String> {
    c.execute("DELETE FROM sd_sessions WHERE id=?1", [id])
        .map_err(err)?;
    Ok(())
}
/// Deletes every system design session and cached question; P1 and coding data are untouched.
pub fn clear_history(c: &Connection) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(err)?;
    tx.execute("DELETE FROM sd_sessions", []).map_err(err)?;
    tx.execute("DELETE FROM sd_questions", []).map_err(err)?;
    tx.commit().map_err(err)
}
