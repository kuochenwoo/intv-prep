//! Persistent per-problem countdowns. Reset archives elapsed time instead of erasing it.
use crate::session_tools::MockClock;
use rusqlite::{params, Connection, OptionalExtension};

pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS coding_clocks(id INTEGER PRIMARY KEY,problem_id TEXT NOT NULL,content_json TEXT NOT NULL,active INTEGER NOT NULL DEFAULT 1);
        CREATE UNIQUE INDEX IF NOT EXISTS coding_clock_active ON coding_clocks(problem_id) WHERE active=1;")
        .map_err(|e| e.to_string())
}

pub fn timing(
    c: &Connection,
    problem: &str,
    action: &str,
    minutes: Option<i64>,
    now: i64,
) -> Result<Option<MockClock>, String> {
    crate::content::find(problem)?;
    if !["get", "start", "pause", "resume", "reset"].contains(&action) {
        return Err("无效的计时操作".into());
    }
    let tx = c.unchecked_transaction().map_err(|e| e.to_string())?;
    let raw: Option<String> = tx
        .query_row(
            "SELECT content_json FROM coding_clocks WHERE problem_id=?1 AND active=1",
            [problem],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut old: Option<MockClock> = raw
        .map(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
        .transpose()?;
    if action == "get" {
        return Ok(old);
    }
    if action == "reset" {
        if let Some(mut clock) = old {
            clock.transition("stop", now)?;
            tx.execute("UPDATE coding_clocks SET content_json=?2,active=0 WHERE problem_id=?1 AND active=1", params![problem, serde_json::to_string(&clock).map_err(|e| e.to_string())?]).map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;
        return Ok(None);
    }
    if action == "start" {
        if old.is_some() {
            return Err("无法开始计时面试".into());
        }
        let budget = minutes
            .filter(|n| [10, 20, 30].contains(n))
            .ok_or("无效的计时操作")?
            * 60000;
        old = Some(MockClock {
            budget_ms: budget,
            remaining_ms: budget,
            deadline_ms: Some(now + budget),
            started_at: now,
            ended_at: None,
        });
        tx.execute(
            "INSERT INTO coding_clocks(problem_id,content_json) VALUES(?1,?2)",
            params![
                problem,
                serde_json::to_string(&old).map_err(|e| e.to_string())?
            ],
        )
        .map_err(|e| e.to_string())?;
    } else {
        old.as_mut()
            .ok_or("尚未开始计时面试")?
            .transition(action, now)?;
        tx.execute(
            "UPDATE coding_clocks SET content_json=?2 WHERE problem_id=?1 AND active=1",
            params![
                problem,
                serde_json::to_string(&old).map_err(|e| e.to_string())?
            ],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(old)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reload_pause_resume_reset_and_expiry_are_counted_once() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("timers.sqlite");
        let c = crate::db::open(&path).unwrap();
        assert!(timing(&c, "lc-1", "get", None, 1000).unwrap().is_none());
        timing(&c, "lc-1", "start", Some(10), 1000).unwrap();
        drop(c);
        let c = crate::db::open(&path).unwrap();
        assert_eq!(
            timing(&c, "lc-1", "get", None, 61000)
                .unwrap()
                .unwrap()
                .left(61000),
            540000
        );
        timing(&c, "lc-1", "pause", None, 61000).unwrap();
        let total = |now| {
            crate::review::overview(&c, now)
                .unwrap()
                .timed
                .iter()
                .map(|t| t.used_ms)
                .sum::<i64>()
        };
        assert_eq!(total(121000), 60000);
        assert!(timing(&c, "lc-2", "get", None, 121000).unwrap().is_none());
        timing(&c, "lc-1", "resume", None, 121000).unwrap();
        timing(&c, "lc-1", "reset", None, 181000).unwrap();
        assert_eq!(total(181000), 120000);
        assert!(timing(&c, "lc-1", "get", None, 181000).unwrap().is_none());
        timing(&c, "lc-1", "start", Some(20), 181000).unwrap();
        assert_eq!(total(2000000), 1320000);
        timing(&c, "lc-1", "reset", None, 2000000).unwrap();
        assert_eq!(total(3000000), 1320000);
        assert_eq!(crate::review::overview(&c, 3000000).unwrap().timed.len(), 2);
    }
    #[test]
    fn review_combines_lc_and_interview_time_with_start_day_attribution() {
        let dir = tempfile::tempdir().unwrap();
        let c = crate::db::open(&dir.path().join("timers.sqlite")).unwrap();
        let start = 86_400_000 - 60_000;
        timing(&c, "lc-1", "start", Some(10), start).unwrap();
        let interview = MockClock {
            budget_ms: 600_000,
            remaining_ms: 480_000,
            deadline_ms: None,
            started_at: start,
            ended_at: None,
        };
        c.execute(
            "INSERT INTO mock_clocks VALUES('p1','authored-local-fixture',?1)",
            [serde_json::to_string(&interview).unwrap()],
        )
        .unwrap();
        let review = crate::review::overview(&c, start + 120_000).unwrap();
        assert_eq!(review.timed.len(), 2);
        assert!(review.timed.iter().all(|t| t.started_ms == start));
        assert_eq!(review.timed.iter().map(|t| t.used_ms).sum::<i64>(), 240_000);
    }
    #[test]
    fn rejects_invalid_actions_and_lengths_without_overwriting() {
        let dir = tempfile::tempdir().unwrap();
        let c = crate::db::open(&dir.path().join("timers.sqlite")).unwrap();
        assert!(timing(&c, "no-problem", "start", Some(10), 0).is_err());
        for minutes in [0, 40, 60] {
            assert!(timing(&c, "lc-1", "start", Some(minutes), 0).is_err());
        }
        timing(&c, "lc-1", "start", Some(30), 0).unwrap();
        assert!(timing(&c, "lc-1", "start", Some(10), 1).is_err());
        assert!(timing(&c, "lc-1", "invalid", None, 1).is_err());
        assert!(timing(&c, "lc-1", "stop", None, 1).is_err());
        assert_eq!(
            timing(&c, "lc-1", "get", None, 1)
                .unwrap()
                .unwrap()
                .budget_ms,
            1800000
        );
    }
}
