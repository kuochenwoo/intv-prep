//! Local per-HTTP-attempt accounting. Prices are user supplied snapshots, never invoices.
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS api_usage(
        id INTEGER PRIMARY KEY, created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
        provider TEXT NOT NULL, model TEXT NOT NULL, module TEXT NOT NULL, session_id TEXT,
        operation TEXT NOT NULL, reviewer INTEGER NOT NULL, status TEXT NOT NULL DEFAULT 'interrupted',
        input_tokens INTEGER, output_tokens INTEGER, cache_read_tokens INTEGER, cache_write_tokens INTEGER,
        estimated_usd REAL, rates_json TEXT);
        CREATE TABLE IF NOT EXISTS api_prices(provider TEXT NOT NULL,model TEXT NOT NULL,rates_json TEXT NOT NULL,PRIMARY KEY(provider,model));")
        .map_err(|e| e.to_string())
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rates {
    pub input: f64,
    pub output: f64,
    pub cache_read: f64,
    pub cache_write: f64,
}
impl Rates {
    pub fn validate(&self) -> Result<(), String> {
        if [self.input, self.output, self.cache_read, self.cache_write]
            .iter()
            .any(|x| !x.is_finite() || *x < 0.0 || *x > 1_000_000.0)
        {
            return Err("无效的费用设置".into());
        }
        Ok(())
    }
}
pub fn save_rates(
    c: &Connection,
    provider: &str,
    model: &str,
    rates: &Rates,
) -> Result<(), String> {
    rates.validate()?;
    if !["claude", "deepseek"].contains(&provider) || model.is_empty() || model.len() > 120 {
        return Err("无效的费用设置".into());
    }
    c.execute("INSERT INTO api_prices VALUES(?1,?2,?3) ON CONFLICT(provider,model) DO UPDATE SET rates_json=excluded.rates_json",params![provider,model,json!(rates).to_string()]).map_err(|e|e.to_string())?;
    Ok(())
}
#[derive(Clone)]
pub struct Context {
    pub path: PathBuf,
    pub module: String,
    pub session_id: Option<String>,
    pub operation: String,
}
pub fn begin(ctx: &Context, provider: &str, model: &str, reviewer: bool) -> Result<i64, String> {
    let c = crate::db::connect(&ctx.path)?;
    c.execute("INSERT INTO api_usage(provider,model,module,session_id,operation,reviewer,rates_json) VALUES(?1,?2,?3,?4,?5,?6,(SELECT rates_json FROM api_prices WHERE provider=?1 AND model=?2))",params![provider,model,ctx.module,ctx.session_id,ctx.operation,reviewer]).map_err(|e|e.to_string())?;
    Ok(c.last_insert_rowid())
}
/// Provider fields have different cache semantics. Missing counters remain unknown.
pub fn tokens(provider: &str, v: &Value) -> [Option<u64>; 4] {
    if provider == "claude" {
        [
            v["input_tokens"].as_u64(),
            v["output_tokens"].as_u64(),
            v["cache_read_input_tokens"].as_u64(),
            v["cache_creation_input_tokens"].as_u64(),
        ]
    } else {
        [
            v["prompt_tokens"].as_u64(),
            v["completion_tokens"].as_u64(),
            v["prompt_cache_hit_tokens"]
                .as_u64()
                .or_else(|| v["prompt_tokens_details"]["cached_tokens"].as_u64()),
            Some(0),
        ]
    }
}
pub fn estimate(provider: &str, t: [Option<u64>; 4], r: &Rates) -> Option<f64> {
    let [i, o, read, write] = t;
    let (i, o) = (i?, o?);
    // DeepSeek prompt_tokens includes hits; Claude input_tokens excludes cache reads/writes.
    let (read, write) = (read?, write?);
    let uncached = if provider == "deepseek" {
        i.checked_sub(read)?
    } else {
        i
    };
    Some(
        (uncached as f64 * r.input
            + o as f64 * r.output
            + read as f64 * r.cache_read
            + write as f64 * r.cache_write)
            / 1_000_000.0,
    )
}
pub fn finish(
    path: &Path,
    id: i64,
    status: &str,
    provider: &str,
    usage: &Value,
    complete: bool,
) -> Result<(), String> {
    let c = crate::db::connect(path)?;
    let raw: Option<String> = c
        .query_row("SELECT rates_json FROM api_usage WHERE id=?1", [id], |r| {
            r.get(0)
        })
        .map_err(|e| e.to_string())?;
    let t = tokens(provider, usage);
    let stored = t.map(|n| n.and_then(|n| i64::try_from(n).ok()));
    let cost = if complete {
        raw.and_then(|s| serde_json::from_str::<Rates>(&s).ok())
            .and_then(|r| estimate(provider, t, &r))
    } else {
        None
    };
    c.execute("UPDATE api_usage SET status=?2,input_tokens=?3,output_tokens=?4,cache_read_tokens=?5,cache_write_tokens=?6,estimated_usd=?7 WHERE id=?1",params![id,status,stored[0],stored[1],stored[2],stored[3],cost]).map_err(|e|e.to_string())?;
    Ok(())
}
pub fn overview(
    c: &Connection,
    module: Option<&str>,
    session: Option<&str>,
) -> Result<Value, String> {
    let mut s=c.prepare("SELECT id,created_at,provider,model,module,session_id,operation,reviewer,status,input_tokens,output_tokens,cache_read_tokens,cache_write_tokens,estimated_usd,rates_json FROM api_usage WHERE (?1 IS NULL OR module=?1) AND (?2 IS NULL OR session_id=?2) ORDER BY id DESC").map_err(|e|e.to_string())?;
    let rows=s.query_map(params![module,session],|r|Ok(json!({"id":r.get::<_,i64>(0)?,"createdAt":r.get::<_,String>(1)?,"provider":r.get::<_,String>(2)?,"model":r.get::<_,String>(3)?,"module":r.get::<_,String>(4)?,"sessionId":r.get::<_,Option<String>>(5)?,"operation":r.get::<_,String>(6)?,"reviewer":r.get::<_,bool>(7)?,"status":r.get::<_,String>(8)?,"inputTokens":r.get::<_,Option<i64>>(9)?,"outputTokens":r.get::<_,Option<i64>>(10)?,"cacheReadTokens":r.get::<_,Option<i64>>(11)?,"cacheWriteTokens":r.get::<_,Option<i64>>(12)?,"estimatedUsd":r.get::<_,Option<f64>>(13)?,"ratesUsdPerMillionTokens":r.get::<_,Option<String>>(14)?.and_then(|s|serde_json::from_str::<Value>(&s).ok())}))).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
    let mut p = c
        .prepare("SELECT provider,model,rates_json FROM api_prices")
        .map_err(|e| e.to_string())?;
    let prices=p.query_map([],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?))).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?.into_iter().map(|(p,m,r)|json!({"provider":p,"model":m,"rates":serde_json::from_str::<Value>(&r).unwrap_or(Value::Null)})).collect::<Vec<_>>();
    Ok(json!({"records":rows,"prices":prices}))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_costs_and_unknown_are_distinct() {
        let r = Rates {
            input: 2.,
            output: 4.,
            cache_read: 0.2,
            cache_write: 3.,
        };
        assert_eq!(
            estimate("deepseek", [Some(100), Some(10), Some(40), Some(0)], &r),
            Some(0.000168)
        );
        assert_eq!(
            estimate("claude", [Some(100), Some(10), Some(40), Some(20)], &r),
            Some(0.000308)
        );
        assert_eq!(
            estimate("claude", [Some(100), None, Some(0), Some(0)], &r),
            None
        );
        assert_eq!(
            estimate("deepseek", [Some(10), Some(10), Some(20), Some(0)], &r),
            None
        );
    }
    #[test]
    fn records_survive_restart_and_snapshot_prices() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("test.sqlite");
        let c = crate::db::open(&path).unwrap();
        let ctx = Context {
            path: path.clone(),
            module: "p1".into(),
            session_id: Some("session".into()),
            operation: "feedback".into(),
        };
        let r = Rates {
            input: 2.,
            output: 4.,
            cache_read: 0.2,
            cache_write: 3.,
        };
        save_rates(&c, "claude", "model", &r).unwrap();
        let id = begin(&ctx, "claude", "model", true).unwrap();
        assert_eq!(
            overview(&c, Some("p1"), Some("session")).unwrap()["records"][0]["status"],
            "interrupted"
        );
        finish(&path,id,"content_error","claude",&json!({"input_tokens":100,"output_tokens":10,"cache_read_input_tokens":0,"cache_creation_input_tokens":0}),true).unwrap();
        save_rates(&c, "claude", "model", &Rates { input: 200., ..r }).unwrap();
        drop(c);
        let c = crate::db::open(&path).unwrap();
        let rows = overview(&c, None, None).unwrap();
        assert_eq!(rows["records"][0]["estimatedUsd"], 0.00024);
        assert!(overview(&c, Some("p2"), None).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty());
        let id = begin(&ctx, "claude", "model", false).unwrap();
        finish(&path, id, "transport_error", "claude", &Value::Null, false).unwrap();
        assert!(overview(&c, None, None).unwrap()["records"][0]["inputTokens"].is_null());
    }
}

/// Link successful generation attempts to the first interview using that question.
/// Reusing a cached question makes no new request and incurs no new generation charge.
pub fn latest_id(c: &Connection) -> Result<i64, String> {
    c.query_row("SELECT COALESCE(MAX(id),0) FROM api_usage", [], |r| {
        r.get(0)
    })
    .map_err(|e| e.to_string())
}
pub fn link_question(
    c: &Connection,
    module: &str,
    question_id: &str,
    after: i64,
) -> Result<(), String> {
    c.execute(
        "UPDATE api_usage SET session_id=?1 WHERE module=?2 AND session_id IS NULL AND id>?3",
        params![format!("question:{question_id}"), module, after],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub fn link_session(
    c: &Connection,
    module: &str,
    question_id: &str,
    session_id: &str,
) -> Result<(), String> {
    c.execute(
        "UPDATE api_usage SET session_id=?1 WHERE module=?2 AND session_id=?3",
        params![session_id, module, format!("question:{question_id}")],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Attribute combined reference/submit actions and retry requests to their actual model task.
pub fn purpose(schema: &Value, context: &Value, reviewer: bool) -> Option<&'static str> {
    let fields = if reviewer {
        &context["proposal"]
    } else {
        &schema["properties"]
    };
    if fields.get("answer").is_some() {
        Some("reference")
    } else if fields.get("dimensions").is_some() || fields.get("assessment").is_some() {
        Some("feedback")
    } else if fields.get("followUpQuestion").is_some() {
        Some("follow_up")
    } else if fields.get("question").is_some() {
        Some("generation")
    } else {
        None
    }
}
#[cfg(test)]
mod purpose_tests {
    use super::*;
    #[test]
    fn combined_actions_and_reviews_keep_actual_task_purpose() {
        assert_eq!(
            purpose(&json!({"properties":{"answer":{}}}), &Value::Null, false),
            Some("reference")
        );
        assert_eq!(
            purpose(
                &json!({"properties":{"followUpQuestion":{}}}),
                &Value::Null,
                false
            ),
            Some("follow_up")
        );
        assert_eq!(
            purpose(&Value::Null, &json!({"proposal":{"assessment":{}}}), true),
            Some("feedback")
        );
        assert_eq!(
            purpose(
                &Value::Null,
                &json!({"proposal":{"question":"question"}}),
                true
            ),
            Some("generation")
        );
    }
}

/// Transcription reports mixed audio/text input tokens. Preserve totals, but do
/// not apply chat-model rates or invent a cost from locally measured duration.
pub fn finish_transcription(
    path: &Path,
    id: i64,
    status: &str,
    usage: &Value,
) -> Result<(), String> {
    let c = crate::db::connect(path)?;
    let counter = |name: &str| usage[name].as_u64().and_then(|n| i64::try_from(n).ok());
    c.execute("UPDATE api_usage SET status=?2,input_tokens=?3,output_tokens=?4,estimated_usd=NULL WHERE id=?1",
        params![id,status,counter("input_tokens"),counter("output_tokens")]).map_err(|e|e.to_string())?;
    Ok(())
}
