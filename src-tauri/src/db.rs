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
    Ok(c)
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
