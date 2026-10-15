//! Session-local tools: persistent mock timing and credential-free exports.
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS mock_clocks(module TEXT NOT NULL,session_id TEXT NOT NULL,content_json TEXT NOT NULL,PRIMARY KEY(module,session_id));").map_err(|e|e.to_string())
}
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
fn validate(c: &Connection, module: &str, id: &str) -> Result<String, String> {
    match module {
        "p1" => Ok(crate::interview_store::session(c, id)?.status),
        "p2" => Ok(crate::system_design_store::session(c, id)?.status),
        _ => Err("未知面试模块".into()),
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockClock {
    pub budget_ms: i64,
    pub remaining_ms: i64,
    pub deadline_ms: Option<i64>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
}
impl MockClock {
    pub fn left(&self, now: i64) -> i64 {
        self.deadline_ms
            .map(|d| (d - now).max(0))
            .unwrap_or(self.remaining_ms)
    }
    pub fn transition(&mut self, action: &str, now: i64) -> Result<(), String> {
        if self.ended_at.is_some() {
            return Err("计时面试已结束".into());
        }
        match action {
            "pause" if self.deadline_ms.is_some() => {
                self.remaining_ms = self.left(now);
                self.deadline_ms = None;
            }
            "resume" if self.deadline_ms.is_none() && self.remaining_ms > 0 => {
                self.deadline_ms = Some(now + self.remaining_ms);
            }
            "stop" => {
                self.remaining_ms = self.left(now);
                self.deadline_ms = None;
                self.ended_at = Some(now);
            }
            _ => return Err("无效的计时操作".into()),
        }
        Ok(())
    }
}
pub fn clock(c: &Connection, module: &str, id: &str) -> Result<Option<MockClock>, String> {
    validate(c, module, id)?;
    let raw: Option<String> = c
        .query_row(
            "SELECT content_json FROM mock_clocks WHERE module=?1 AND session_id=?2",
            params![module, id],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    raw.map(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
        .transpose()
}
pub fn timing(
    c: &Connection,
    module: &str,
    id: &str,
    action: &str,
    minutes: Option<i64>,
) -> Result<Option<MockClock>, String> {
    let tx = c.unchecked_transaction().map_err(|e| e.to_string())?;
    let status = validate(&tx, module, id)?;
    let mut old = clock(&tx, module, id)?;
    if action == "get" {
        return Ok(old);
    }
    if status != "active" {
        return Err("计时面试已结束".into());
    }
    if action == "reset" {
        tx.execute(
            "DELETE FROM mock_clocks WHERE module=?1 AND session_id=?2",
            params![module, id],
        )
        .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        return Ok(None);
    }
    if action == "start" {
        if old.is_some() || status != "active" {
            return Err("无法开始计时面试".into());
        }
        let minutes = minutes
            .filter(|n| [10, 20, 30].contains(n))
            .ok_or("无效的计时操作")?;
        let now = now_ms();
        old = Some(MockClock {
            budget_ms: minutes * 60000,
            remaining_ms: minutes * 60000,
            deadline_ms: Some(now + minutes * 60000),
            started_at: now,
            ended_at: None,
        });
    } else {
        old.as_mut()
            .ok_or("尚未开始计时面试")?
            .transition(action, now_ms())?;
    }
    tx.execute("INSERT INTO mock_clocks VALUES(?1,?2,?3) ON CONFLICT(module,session_id) DO UPDATE SET content_json=excluded.content_json",params![module,id,json!(old).to_string()]).map_err(|e|e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(old)
}
pub fn stop(c: &Connection, module: &str, id: &str) -> Result<(), String> {
    if let Some(mut timer) = clock(c, module, id)? {
        if timer.ended_at.is_none() {
            timer.transition("stop", now_ms())?;
            c.execute(
                "UPDATE mock_clocks SET content_json=?3 WHERE module=?1 AND session_id=?2",
                params![module, id, json!(timer).to_string()],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {
    pub filename: String,
    pub text: String,
}
pub fn export(
    c: &Connection,
    module: &str,
    id: &str,
    format: &str,
    lang: &str,
) -> Result<Export, String> {
    validate(c, module, id)?;
    if !["json", "md"].contains(&format) || !["en", "zh"].contains(&lang) {
        return Err("无效的导出格式".into());
    }
    // Strip private P2 generation fields explicitly; only P1 already has a public question DTO.
    let mut session = if module == "p1" {
        json!(crate::interview_store::session(c, id)?)
    } else {
        json!(crate::system_design_store::session(c, id)?)
    };
    session.as_object_mut().unwrap().remove("lastError");
    if let Some(content) = session
        .get_mut("question")
        .and_then(|q| q.get_mut("content"))
        .and_then(Value::as_object_mut)
    {
        content.remove("evaluationChecklist");
        content.remove("followUpDirections");
        content.remove("answerTerms");
    }
    // Drafts are not submitted evidence. Exports include only saved notes/diagram and submitted turns.
    session.as_object_mut().unwrap().remove("draft");
    if let Some(w) = session.get_mut("workspace").and_then(Value::as_object_mut) {
        w.remove("drafts");
    }
    let content_lang = if module == "p2" {
        session["question"]["lang"].clone()
    } else {
        let text = session["question"]["question"].as_str().unwrap_or_default();
        json!(
            if text.chars().any(|x| ('\u{4e00}'..='\u{9fff}').contains(&x)) {
                "zh"
            } else {
                "en"
            }
        )
    };
    let usage = crate::usage::overview(c, Some(module), Some(id))?["records"].clone();
    let value = json!({"schemaVersion":1,"module":module,"contentLanguage":content_lang,"exportLanguage":lang,"exportedAtMs":now_ms(),"session":session,"timing":clock(c,module,id)?,"usage":usage,"billing":"estimates_only_not_invoices"});
    let text = if format == "json" {
        serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
    } else {
        markdown(&value, lang)
    };
    Ok(Export {
        filename: format!("interview-{module}-{id}.{format}"),
        text,
    })
}
fn markdown(v: &Value, lang: &str) -> String {
    let zh = lang == "zh";
    let s = &v["session"];
    let q = if v["module"] == "p1" {
        &s["question"]
    } else {
        &s["question"]["content"]
    };
    let label = |en: &str, cn: &str| if zh { cn.to_owned() } else { en.to_owned() };
    let mut out = format!(
        "# {}\n\n{} / {} · {}\n\n## {}\n\n{}\n\n{}\n\n",
        label("Interview", "面试记录"),
        s["provider"].as_str().unwrap_or_default(),
        s["model"].as_str().unwrap_or_default(),
        s["status"].as_str().unwrap_or_default(),
        label("Question", "题目"),
        q["question"].as_str().unwrap_or_default(),
        q["background"].as_str().unwrap_or_default()
    );
    if let Some(xs) = q["constraints"].as_array() {
        for x in xs {
            out.push_str(&format!("- {}\n", x.as_str().unwrap_or_default()));
        }
    }
    out.push_str(&format!("\n## {}\n", label("Conversation", "面试对话")));
    if let Some(xs) = s["turns"].as_array() {
        for x in xs {
            let assisted = x["aiAssisted"].as_bool().unwrap_or(false);
            out.push_str(&format!(
                "\n### {}{}{}\n\n{}\n",
                if x["role"] == "candidate" {
                    label("Candidate", "候选人")
                } else {
                    label("Interviewer", "面试官")
                },
                if assisted {
                    label(
                        " (AI-assisted; not independent evidence)",
                        "（AI 辅助，不作为独立能力证据）",
                    )
                } else {
                    String::new()
                },
                x["stage"]
                    .as_str()
                    .map(|t| format!(" · {t}"))
                    .unwrap_or_default(),
                x["text"].as_str().unwrap_or_default()
            ));
        }
    }
    if let Some(w) = s.get("workspace") {
        out.push_str(&format!(
            "\n## {}\n\n{}\n\n## {}\n\n",
            label("Design notes", "设计笔记"),
            w["notes"].as_str().unwrap_or_default(),
            label("Diagram source", "图表源代码")
        ));
        let diagram = w["diagram"].as_str().unwrap_or_default();
        let fence = "`".repeat(
            diagram
                .split('\n')
                .map(|l| l.chars().take_while(|c| *c == '`').count())
                .max()
                .unwrap_or(0)
                .max(2)
                + 1,
        );
        out.push_str(&format!("{fence}mermaid\n{diagram}\n{fence}\n"));
    }
    for (key, en, cn) in [
        ("references", "AI reference previews", "AI 参考回答"),
        ("feedback", "Feedback", "反馈"),
    ] {
        if !s[key].is_null() {
            out.push_str(&format!("\n## {}\n\n", label(en, cn)));
            render(&s[key], &mut out, 0);
        }
    }
    for (key, en, cn) in [
        ("timing", "Mock interview timing", "模拟面试计时"),
        (
            "usage",
            "API usage (estimated USD; not billed amounts)",
            "API 用量（美元估算，非账单）",
        ),
    ] {
        out.push_str(&format!("\n## {}\n\n", label(en, cn)));
        render(&v[key], &mut out, 0);
    }
    out
}
fn render(v: &Value, out: &mut String, depth: usize) {
    match v {
        Value::Object(m) => {
            for (k, v) in m {
                out.push_str(&format!("{}- **{}**: ", "  ".repeat(depth), k));
                if v.is_object() || v.is_array() {
                    out.push('\n');
                    render(v, out, depth + 1);
                } else {
                    out.push_str(&format!(
                        "{}\n",
                        v.as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| v.to_string())
                    ));
                }
            }
        }
        Value::Array(xs) => {
            for x in xs {
                if x.is_object() {
                    render(x, out, depth);
                } else {
                    out.push_str(&format!(
                        "{}- {}\n",
                        "  ".repeat(depth),
                        x.as_str().unwrap_or_default()
                    ));
                }
            }
        }
        _ => out.push_str(&format!("{v}\n")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clock_transitions_preserve_elapsed_and_expiry() {
        let mut c = MockClock {
            budget_ms: 60000,
            remaining_ms: 60000,
            deadline_ms: Some(61000),
            started_at: 1000,
            ended_at: None,
        };
        c.transition("pause", 11000).unwrap();
        assert_eq!(c.left(90000), 50000);
        c.transition("resume", 90000).unwrap();
        assert_eq!(c.left(100000), 40000);
        assert_eq!(c.left(200000), 0);
        c.transition("stop", 200000).unwrap();
        assert!(c.transition("resume", 200001).is_err());
    }
    #[test]
    fn diagram_fences_cannot_terminate_export_block() {
        let v = json!({"module":"p2","session":{"question":{"content":{}},"workspace":{"diagram":"```\nsource"},"turns":[{"role":"candidate","text":"answer","aiAssisted":true}]}});
        let md = markdown(&v, "en");
        assert!(md.contains("````mermaid\n```\nsource\n````"));
        assert!(md.contains("not independent evidence"));
    }
}
