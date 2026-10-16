use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::path::Path;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub problem_id: String,
    pub mode: String,
    pub code: String,
    pub status: String,
    pub last_result: Option<String>,
}
/// Open a connection to an already-migrated database. Cheap: no schema work.
pub fn connect(path: &Path) -> Result<Connection, String> {
    let c = Connection::open(path).map_err(|e| e.to_string())?;
    c.busy_timeout(std::time::Duration::from_secs(3))
        .map_err(|e| e.to_string())?;
    // Per-connection setting; session deletes rely on ON DELETE CASCADE.
    c.execute_batch("PRAGMA foreign_keys=ON;")
        .map_err(|e| e.to_string())?;
    Ok(c)
}
/// Create or upgrade the database, then return a connection. The app calls this
/// once at startup; migrations are idempotent so reopening an old file is safe.
pub fn open(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let c = connect(path)?;
    c.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE IF NOT EXISTS coding_progress(problem_id TEXT NOT NULL, mode TEXT NOT NULL, code TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'in_progress', last_result TEXT, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY(problem_id,mode)); CREATE TABLE IF NOT EXISTS settings(key TEXT PRIMARY KEY, value TEXT NOT NULL);").map_err(|e|e.to_string())?;
    crate::interview_store::migrate(&c)?;
    crate::system_design_store::migrate(&c)?;
    crate::usage::migrate(&c)?;
    crate::session_tools::migrate(&c)?;
    crate::review::migrate(&c)?;
    crate::coding_timer::migrate(&c)?;
    slim_results(&c)?;
    Ok(c)
}
/// Older versions stored every hidden test's full input and answer with each result.
/// Shrink those once to the summary the judge stores now; small results are untouched.
fn slim_results(c: &Connection) -> Result<(), String> {
    let rows: Vec<(String, String, String)> = {
        let mut s = c
            .prepare("SELECT problem_id,mode,last_result FROM coding_progress WHERE length(last_result) > 32768")
            .map_err(|e| e.to_string())?;
        let rows = s
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<_, _>>().map_err(|e| e.to_string())?
    };
    for (id, mode, text) in rows {
        let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };
        for case in v["cases"].as_array_mut().into_iter().flatten() {
            let passed = case["status"] == "accepted";
            for field in ["input", "actual", "stderr"] {
                let old = case[field].as_str().unwrap_or_default().to_string();
                case[field] = if passed && field != "stderr" {
                    "".into()
                } else {
                    crate::code_runner::excerpt(old).into()
                };
            }
            case["expected"] = if passed {
                serde_json::Value::Null
            } else {
                crate::code_runner::bounded(&case["expected"])
            };
        }
        c.execute(
            "UPDATE coding_progress SET last_result=?3 WHERE problem_id=?1 AND mode=?2",
            params![id, mode, v.to_string()],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn load(c: &Connection) -> Result<Vec<Progress>, String> {
    let mut s = c
        .prepare("SELECT problem_id,mode,code,status,last_result FROM coding_progress")
        .map_err(|e| e.to_string())?;
    let rows = s
        .query_map([], |r| {
            Ok(Progress {
                problem_id: r.get(0)?,
                mode: r.get(1)?,
                code: r.get(2)?,
                status: r.get(3)?,
                last_result: r.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}
pub fn save(c: &Connection, id: &str, mode: &str, code: &str) -> Result<(), String> {
    crate::content::find(id)?;
    if !["function", "acm"].contains(&mode) || code.len() > 256 * 1024 {
        return Err("无效的草稿".into());
    }
    c.execute("INSERT INTO coding_progress(problem_id,mode,code) VALUES(?1,?2,?3) ON CONFLICT(problem_id,mode) DO UPDATE SET code=excluded.code,updated_at=CURRENT_TIMESTAMP",params![id,mode,code]).map_err(|e|e.to_string())?;
    Ok(())
}
pub fn record(
    c: &Connection,
    id: &str,
    mode: &str,
    result: &str,
    accepted: bool,
) -> Result<(), String> {
    c.execute("UPDATE coding_progress SET last_result=?3,status=CASE WHEN ?4 THEN 'completed' ELSE status END,updated_at=CURRENT_TIMESTAMP WHERE problem_id=?1 AND mode=?2",params![id,mode,result,accepted]).map_err(|e|e.to_string())?;
    Ok(())
}
/// Delete saved code and results for one problem (both modes), or for every problem.
/// The judge-run log behind the Review tab is kept.
pub fn clear(c: &Connection, problem_id: Option<&str>) -> Result<(), String> {
    match problem_id {
        Some(id) => {
            crate::content::find(id)?;
            c.execute("DELETE FROM coding_progress WHERE problem_id=?1", [id])
        }
        None => c.execute("DELETE FROM coding_progress", []),
    }
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub fn compiler(c: &Connection) -> Result<String, String> {
    c.query_row("SELECT value FROM settings WHERE key='compiler'", [], |r| {
        r.get(0)
    })
    .optional()
    .map(|v| v.unwrap_or_default())
    .map_err(|e| e.to_string())
}
pub fn set_compiler(c: &Connection, value: &str) -> Result<(), String> {
    c.execute("INSERT INTO settings VALUES('compiler',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value",[value]).map_err(|e|e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clear_removes_one_problem_or_all() {
        let dir = tempfile::tempdir().unwrap();
        let c = open(&dir.path().join("t.sqlite3")).unwrap();
        let ids: Vec<_> = crate::content::problems()[..2]
            .iter()
            .map(|p| p.id.clone())
            .collect();
        for id in &ids {
            save(&c, id, "function", "a").unwrap();
            save(&c, id, "acm", "b").unwrap();
        }
        crate::review::log_run(&c, &ids[0], "function", "accepted", 1, 1).unwrap();
        clear(&c, Some(&ids[0])).unwrap();
        assert!(load(&c).unwrap().iter().all(|p| p.problem_id == ids[1]));
        assert_eq!(load(&c).unwrap().len(), 2);
        assert!(clear(&c, Some("no-such-problem")).is_err());
        clear(&c, None).unwrap();
        assert!(load(&c).unwrap().is_empty());
        let runs: i64 = c
            .query_row("SELECT COUNT(*) FROM judge_runs", [], |r| r.get(0))
            .unwrap();
        assert_eq!(runs, 1);
    }
    #[test]
    fn large_stored_results_are_slimmed_on_open() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.sqlite3");
        let c = open(&path).unwrap();
        let id = crate::content::problems()[0].id.clone();
        save(&c, &id, "function", "a").unwrap();
        let big = "1 ".repeat(40_000);
        let result = serde_json::json!({"status":"wrong_answer","cases":[
            {"id":"h1","status":"accepted","input":big,"expected":[1],"actual":"1","stderr":""},
            {"id":"h2","status":"wrong_answer","input":big,"expected":[2],"actual":"3","stderr":""}]});
        record(&c, &id, "function", &result.to_string(), false).unwrap();
        drop(c);
        let c = open(&path).unwrap();
        let row = load(&c).unwrap().remove(0).last_result.unwrap();
        assert!(row.len() < 10_000);
        let v: serde_json::Value = serde_json::from_str(&row).unwrap();
        assert_eq!(v["cases"][0]["input"], "");
        assert!(v["cases"][0]["expected"].is_null());
        assert!(v["cases"][1]["input"].as_str().unwrap().contains("bytes total"));
        assert_eq!(v["cases"][1]["actual"], "3");
    }
}
