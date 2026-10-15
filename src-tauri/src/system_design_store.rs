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
        "INSERT INTO sd_questions VALUES(?1,'system-design-v3-reviewed',?2)",
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
            ai_assisted: false,
        }],
        suggested_stage: None,
        last_error: None,
        feedback: None,
        references: vec![],
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
            ai_assisted: false,
        });
        put(&tx, &s)?;
    }
    tx.commit().map_err(err)?;
    Ok(s)
}
pub fn begin_answer(c: &Connection, id: &str, answer: &str) -> Result<(), String> {
    begin_answer_kind(c, id, answer, false)
}
fn begin_answer_kind(c: &Connection, id: &str, answer: &str, ai: bool) -> Result<(), String> {
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
        ai_assisted: ai,
    });
    // Kept as the stage draft until the follow-up arrives; an AI answer never becomes a draft.
    if !ai {
        s.workspace.drafts.insert(s.stage, answer.into());
    }
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
    crate::session_tools::stop(&tx, "p2", id)?;
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
                ai_assisted: false,
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
    let tx = c.unchecked_transaction().map_err(err)?;
    tx.execute("DELETE FROM sd_sessions WHERE id=?1", [id])
        .map_err(err)?;
    tx.execute(
        "DELETE FROM mock_clocks WHERE module=?1 AND session_id=?2",
        params!["p2", id],
    )
    .map_err(err)?;
    tx.commit().map_err(err)
}
/// Deletes every system design session and cached question; P1 and coding data are untouched.
pub fn clear_history(c: &Connection) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(err)?;
    tx.execute("DELETE FROM sd_sessions", []).map_err(err)?;
    tx.execute("DELETE FROM sd_questions", []).map_err(err)?;
    tx.execute("DELETE FROM mock_clocks WHERE module=?1", ["p2"])
        .map_err(err)?;
    tx.commit().map_err(err)
}
/// Generates one reference answer for the current question; asking again returns the saved one.
pub fn generate_reference(
    c: &Connection,
    id: &str,
    api: &impl DesignProvider,
) -> Result<DesignSession, String> {
    let s = session(c, id)?;
    active(&s)?;
    let turn_index = s.turns.len();
    if s.references.iter().any(|r| r.turn_index == turn_index) {
        return Ok(s);
    }
    let answer = api.design_reference(&s)?;
    answer.validate()?;
    let tx = c.unchecked_transaction().map_err(err)?;
    let mut current = session(&tx, id)?;
    if current.status != "active" || current.turns.len() != turn_index {
        return Err("系统设计会话状态已变化，请刷新".into());
    }
    current.references.push(DesignReference {
        turn_index,
        stage: current.stage,
        answer: answer.answer,
        key_points: answer.key_points,
    });
    put(&tx, &current)?;
    tx.commit().map_err(err)?;
    Ok(current)
}
/// Submits the saved reference answer as an AI-assisted turn, then asks for the follow-up.
pub fn submit_reference(
    c: &Connection,
    id: &str,
    turn_index: usize,
    api: &impl DesignProvider,
) -> Result<DesignSession, String> {
    let s = session(c, id)?;
    active(&s)?;
    if turn_index != s.turns.len() {
        return Err("只能提交当前问题的参考答案".into());
    }
    let reference = s
        .references
        .iter()
        .find(|r| r.turn_index == turn_index)
        .ok_or("当前问题还没有参考答案")?;
    if s.workspace
        .drafts
        .get(&s.stage)
        .is_some_and(|d| !d.trim().is_empty())
    {
        return Err("请先提交或清空当前草稿，再使用 AI 代答".into());
    }
    begin_answer_kind(c, id, &reference.answer, true)?;
    retry_pending(c, id, api)
}
