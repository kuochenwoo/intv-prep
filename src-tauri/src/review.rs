//! Review tab: daily activity, weak points and spaced review.
//!
//! Everything here is derived from local history (judge submissions, finished
//! interviews, timers and usage rows). Nothing calls a provider, so the tab is
//! offline and free. The only new state is the judge-run log, which starts when
//! this feature ships, and the list of weak points the user dismissed.
use crate::interview::Rating;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::BTreeMap;

const DAY_MS: i64 = 86_400_000;
/// Days until the next review, indexed by how many spaced good outcomes followed
/// the last weak one.
const INTERVAL_DAYS: [i64; 3] = [1, 3, 7];
/// Good outcomes in a row (each a review apart) that resolve a weak point.
pub const RESOLVED_AFTER: u32 = 3;
/// A good outcome only counts as a review when it comes at least this long after
/// the previous counted outcome; resubmitting the same fix minutes later does not.
const MIN_GAP_MS: i64 = 20 * 3_600_000;
const EVIDENCE_LIMIT: usize = 6;

fn err(e: impl ToString) -> String {
    e.to_string()
}
pub fn migrate(c: &Connection) -> Result<(), String> {
    c.execute_batch(
        "CREATE TABLE IF NOT EXISTS judge_runs(
        id INTEGER PRIMARY KEY, problem_id TEXT NOT NULL, mode TEXT NOT NULL, status TEXT NOT NULL,
        passed INTEGER NOT NULL, total INTEGER NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')));
        CREATE TABLE IF NOT EXISTS review_dismissed(concept TEXT PRIMARY KEY,
        dismissed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')));",
    )
    .map_err(err)
}
/// Record one full submission (not a sample run).
pub fn log_run(
    c: &Connection,
    problem_id: &str,
    mode: &str,
    status: &str,
    passed: usize,
    total: usize,
) -> Result<(), String> {
    c.execute(
        "INSERT INTO judge_runs(problem_id,mode,status,passed,total) VALUES(?1,?2,?3,?4,?5)",
        params![problem_id, mode, status, passed as i64, total as i64],
    )
    .map_err(err)?;
    Ok(())
}
/// Hide a weak point until new weak evidence for it appears.
pub fn dismiss(c: &Connection, concept: &str) -> Result<(), String> {
    if concept.is_empty() || concept.len() > 200 {
        return Err("无效的复习条目".into());
    }
    c.execute("INSERT INTO review_dismissed(concept) VALUES(?1) ON CONFLICT(concept) DO UPDATE SET dismissed_at=excluded.dismissed_at",[concept]).map_err(err)?;
    Ok(())
}

/// Milliseconds since the epoch for `YYYY-MM-DDTHH:MM:SS[.fff]Z` or SQLite's
/// `YYYY-MM-DD HH:MM:SS` (both UTC).
pub fn parse_ms(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let num = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
    let (hh, mm, ss) = (num(11..13)?, num(14..16)?, num(17..19)?);
    let frac = match s.get(19..20) {
        Some(".") => {
            let digits: String = s[20..].chars().take_while(|c| c.is_ascii_digit()).collect();
            format!("{digits:0<3}")[..3].parse::<i64>().ok()?
        }
        _ => 0,
    };
    // Days from civil (Howard Hinnant's algorithm).
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * DAY_MS + (hh * 3600 + mm * 60 + ss) * 1000 + frac)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Outcome {
    pub ms: i64,
    pub good: bool,
}
#[derive(Debug, PartialEq)]
pub struct Schedule {
    /// Spaced good outcomes since the last weak one.
    pub stage: u32,
    /// Next review time; `None` once resolved.
    pub due_ms: Option<i64>,
    pub first_weak_ms: i64,
    pub last_weak_ms: i64,
    pub weak_count: u32,
}
/// Spaced-review state from a concept's outcomes. `None` when it was never weak.
pub fn schedule(outcomes: &[Outcome]) -> Option<Schedule> {
    let mut sorted = outcomes.to_vec();
    sorted.sort_by_key(|o| o.ms);
    let mut state: Option<Schedule> = None;
    let mut last = 0;
    for o in sorted {
        match (&mut state, o.good) {
            (None, true) => {}
            (None, false) => {
                state = Some(Schedule {
                    stage: 0,
                    due_ms: None,
                    first_weak_ms: o.ms,
                    last_weak_ms: o.ms,
                    weak_count: 1,
                });
                last = o.ms;
            }
            (Some(s), false) => {
                s.stage = 0;
                s.last_weak_ms = o.ms;
                s.weak_count += 1;
                last = o.ms;
            }
            (Some(s), true) => {
                if o.ms - last >= MIN_GAP_MS && s.stage < RESOLVED_AFTER {
                    s.stage += 1;
                    last = o.ms;
                }
            }
        }
    }
    state.map(|mut s| {
        s.due_ms =
            (s.stage < RESOLVED_AFTER).then(|| last + INTERVAL_DAYS[s.stage as usize] * DAY_MS);
        s
    })
}
pub fn status(stage: u32) -> &'static str {
    match stage {
        0 => "open",
        s if s < RESOLVED_AFTER => "improving",
        _ => "resolved",
    }
}

#[derive(Serialize, Clone, Default)]
pub struct Title {
    pub zh: String,
    pub en: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub at: String,
    /// `lc`, `p1` or `p2`.
    pub module: &'static str,
    /// Problem id for `lc`, session id otherwise.
    pub ref_id: String,
    pub title: Title,
    /// Judge status for `lc`, session status otherwise.
    pub status: String,
    pub passed: Option<usize>,
    pub total: Option<usize>,
    /// The problem's first logged submission, and it was accepted.
    pub first_try: bool,
    /// Feedback dimensions rated good / below good (finished interviews only).
    pub good: u32,
    pub weak: u32,
    /// System design stages the user answered in.
    pub stages: u32,
}
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub at: String,
    /// `answer`, `feedback` or `judge`.
    pub kind: &'static str,
    pub text: String,
    /// Feedback dimension, or judge status.
    pub label: String,
    pub session_id: Option<String>,
    pub source: Title,
    pub ai_assisted: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeakPoint {
    /// Stable key: `lc:<problem>`, `p1:<topic>` or `p2:<dimension>`.
    pub concept: String,
    pub module: &'static str,
    /// Empty for `p2`; the UI labels dimensions itself.
    pub title: Title,
    /// Problem, topic or scenario to practice.
    pub practice_id: String,
    pub session_id: Option<String>,
    pub status: &'static str,
    pub stage: u32,
    pub weak_count: u32,
    pub first_weak_ms: i64,
    pub last_ms: i64,
    pub due_ms: Option<i64>,
    pub evidence: Vec<Evidence>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timed {
    pub started_ms: i64,
    pub used_ms: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub at: String,
    pub usd: Option<f64>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    pub now_ms: i64,
    pub activity: Vec<Activity>,
    pub weak_points: Vec<WeakPoint>,
    pub timed: Vec<Timed>,
    pub costs: Vec<Cost>,
}

/// Evidence and outcomes collected for one concept before scheduling.
#[derive(Default)]
struct Concept {
    module: &'static str,
    title: Title,
    outcomes: Vec<Outcome>,
    evidence: Vec<(i64, Evidence)>,
    /// (time, practice id, session id) of the latest weak outcome.
    latest_weak: Option<(i64, String, Option<String>)>,
}
impl Concept {
    fn add(&mut self, ms: i64, good: bool, practice: &str, session: Option<&str>) {
        self.outcomes.push(Outcome { ms, good });
        if !good && self.latest_weak.as_ref().is_none_or(|(t, ..)| ms >= *t) {
            self.latest_weak = Some((ms, practice.into(), session.map(Into::into)));
        }
    }
}
fn clip(s: &str, n: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= n {
        t.into()
    } else {
        format!("{}…", t.chars().take(n).collect::<String>())
    }
}
fn weak_rating(r: &Rating) -> bool {
    matches!(r, Rating::Partial | Rating::Incorrect)
}
fn problem_title(id: &str) -> Title {
    match crate::content::find(id) {
        Ok(p) => Title {
            zh: p.title.clone(),
            en: p
                .en
                .as_ref()
                .map_or_else(|| p.title.clone(), |e| e.title.clone()),
        },
        Err(_) => Title {
            zh: id.into(),
            en: id.into(),
        },
    }
}
fn topic_title(id: &str) -> Title {
    match crate::interview::topic(id) {
        Ok(t) => Title {
            en: t
                .en
                .as_ref()
                .map_or_else(|| t.title.clone(), |e| e.title.clone()),
            zh: t.title,
        },
        Err(_) => Title {
            zh: id.into(),
            en: id.into(),
        },
    }
}
fn scenario_title(id: &str) -> Title {
    match crate::system_design::scenario(id) {
        Ok(s) => Title {
            zh: s.text.title,
            en: s.en.title,
        },
        Err(_) => Title {
            zh: id.into(),
            en: id.into(),
        },
    }
}
/// Judge failures that say nothing about the algorithm.
fn tooling_failure(status: &str) -> bool {
    status.starts_with("compile")
}

fn judge_history(
    c: &Connection,
    activity: &mut Vec<Activity>,
    concepts: &mut BTreeMap<String, Concept>,
) -> Result<(), String> {
    let mut stmt = c
        .prepare("SELECT problem_id,status,passed,total,created_at FROM judge_runs ORDER BY id")
        .map_err(err)?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, String>(4)?,
            ))
        })
        .map_err(err)?;
    let mut seen = std::collections::HashSet::new();
    for row in rows {
        let (id, status, passed, total, at) = row.map_err(err)?;
        let Some(ms) = parse_ms(&at) else { continue };
        let title = problem_title(&id);
        let first = seen.insert(id.clone());
        activity.push(Activity {
            at: at.clone(),
            module: "lc",
            ref_id: id.clone(),
            title: title.clone(),
            status: status.clone(),
            passed: Some(passed as usize),
            total: Some(total as usize),
            first_try: first && status == "accepted",
            good: 0,
            weak: 0,
            stages: 0,
        });
        if tooling_failure(&status) {
            continue;
        }
        let good = status == "accepted";
        let concept = concepts.entry(format!("lc:{id}")).or_default();
        concept.module = "lc";
        concept.title = title.clone();
        concept.add(ms, good, &id, None);
        if !good {
            concept.evidence.push((
                ms,
                Evidence {
                    at,
                    kind: "judge",
                    text: format!("{passed}/{total}"),
                    label: status,
                    session_id: None,
                    source: title,
                    ai_assisted: false,
                },
            ));
        }
    }
    Ok(())
}

fn deep_dive_history(
    c: &Connection,
    activity: &mut Vec<Activity>,
    concepts: &mut BTreeMap<String, Concept>,
) -> Result<(), String> {
    for summary in crate::interview_store::sessions(c)? {
        // A damaged session should not hide the rest of the tab.
        let Ok(s) = crate::interview_store::session(c, &summary.id) else {
            continue;
        };
        let answers: Vec<_> = s.turns.iter().filter(|t| t.role == "candidate").collect();
        let Some(last) = answers.last() else { continue };
        let at = last.timestamp.clone();
        let Some(ms) = parse_ms(&at) else { continue };
        let title = topic_title(&summary.seed_id);
        let fb = s.feedback.as_ref().filter(|_| s.status == "finished");
        let (good, weak) = fb.map_or((0, 0), |f| {
            (
                f.dimensions
                    .iter()
                    .filter(|d| d.rating == Rating::Good)
                    .count() as u32,
                f.dimensions
                    .iter()
                    .filter(|d| weak_rating(&d.rating))
                    .count() as u32,
            )
        });
        activity.push(Activity {
            at: at.clone(),
            module: "p1",
            ref_id: s.id.clone(),
            title: title.clone(),
            status: s.status.clone(),
            passed: None,
            total: None,
            first_try: false,
            good,
            weak,
            stages: 0,
        });
        let Some(f) = fb else { continue };
        // Feedback on AI-only answers says nothing about the user.
        if answers.iter().all(|t| t.ai_assisted) {
            continue;
        }
        if good + weak == 0 {
            continue;
        }
        let concept = concepts
            .entry(format!("p1:{}", summary.seed_id))
            .or_default();
        concept.module = "p1";
        concept.title = title.clone();
        concept.add(ms, weak == 0, &summary.seed_id, Some(&s.id));
        if weak == 0 {
            continue;
        }
        let ev = |kind, text: String, label: String, ai| Evidence {
            at: at.clone(),
            kind,
            text,
            label,
            session_id: Some(s.id.clone()),
            source: title.clone(),
            ai_assisted: ai,
        };
        // Feedback carries its own scoped evidence. Never guess a matching answer
        // by selecting the last independent turn of a multi-question interview.
        for d in f.dimensions.iter().filter(|d| weak_rating(&d.rating)) {
            concept.evidence.push((
                ms,
                ev("feedback", d.evidence.clone(), d.name.clone(), false),
            ));
        }
        if let Some(a) = answers.iter().find(|t| t.ai_assisted) {
            concept
                .evidence
                .push((ms, ev("answer", clip(&a.text, 160), String::new(), true)));
        }
    }
    Ok(())
}

fn design_history(
    c: &Connection,
    activity: &mut Vec<Activity>,
    concepts: &mut BTreeMap<String, Concept>,
) -> Result<(), String> {
    for summary in crate::system_design_store::sessions(c)? {
        let Ok(s) = crate::system_design_store::session(c, &summary.id) else {
            continue;
        };
        let answers: Vec<_> = s.turns.iter().filter(|t| t.role == "candidate").collect();
        if answers.is_empty() {
            continue;
        }
        // Design turns carry no timestamps; the session start stands in.
        let at = s.created_at.clone();
        let Some(ms) = parse_ms(&at) else { continue };
        let seed = s.question.seed_id.clone();
        let title = scenario_title(&seed);
        let fb = s.feedback.as_ref().filter(|_| s.status == "finished");
        let (good, weak) = fb.map_or((0, 0), |f| {
            (
                f.assessment
                    .values()
                    .filter(|a| a.rating == Rating::Good)
                    .count() as u32,
                f.assessment
                    .values()
                    .filter(|a| matches!(a.rating, Rating::Partial | Rating::Incorrect))
                    .count() as u32,
            )
        });
        let stages = answers
            .iter()
            .map(|t| t.stage)
            .collect::<std::collections::BTreeSet<_>>()
            .len() as u32;
        activity.push(Activity {
            at: at.clone(),
            module: "p2",
            ref_id: s.id.clone(),
            title: title.clone(),
            status: s.status.clone(),
            passed: None,
            total: None,
            first_try: false,
            good,
            weak,
            stages,
        });
        let Some(f) = fb else { continue };
        if answers.iter().all(|t| t.ai_assisted) {
            continue;
        }
        for (dim, a) in &f.assessment {
            // Missing usually means the stage was never discussed, which is not a weakness.
            let good = match a.rating {
                Rating::Good => true,
                Rating::Partial | Rating::Incorrect => false,
                _ => continue,
            };
            let concept = concepts.entry(format!("p2:{dim}")).or_default();
            concept.module = "p2";
            concept.add(ms, good, &seed, Some(&s.id));
            if !good {
                concept.evidence.push((
                    ms,
                    Evidence {
                        at: at.clone(),
                        kind: "feedback",
                        text: clip(
                            &format!("{} {}", a.evidence.trim(), a.improvement.trim()),
                            320,
                        ),
                        label: dim.clone(),
                        session_id: Some(s.id.clone()),
                        source: title.clone(),
                        ai_assisted: false,
                    },
                ));
            }
        }
    }
    Ok(())
}

pub fn overview(c: &Connection, now_ms: i64) -> Result<Overview, String> {
    let mut activity = vec![];
    let mut concepts = BTreeMap::new();
    judge_history(c, &mut activity, &mut concepts)?;
    deep_dive_history(c, &mut activity, &mut concepts)?;
    design_history(c, &mut activity, &mut concepts)?;
    activity.sort_by(|a, b| b.at.cmp(&a.at));

    let mut dismissed = BTreeMap::new();
    {
        let mut stmt = c
            .prepare("SELECT concept,dismissed_at FROM review_dismissed")
            .map_err(err)?;
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .map_err(err)?;
        for row in rows {
            let (k, at) = row.map_err(err)?;
            dismissed.insert(k, parse_ms(&at).unwrap_or(i64::MAX));
        }
    }
    let mut weak_points = vec![];
    for (key, mut concept) in concepts {
        let Some(sched) = schedule(&concept.outcomes) else {
            continue;
        };
        if dismissed
            .get(&key)
            .is_some_and(|&d| d >= sched.last_weak_ms)
        {
            continue;
        }
        let (_, practice_id, session_id) = concept.latest_weak.take().unwrap_or_default();
        concept.evidence.sort_by(|a, b| b.0.cmp(&a.0));
        weak_points.push(WeakPoint {
            module: concept.module,
            title: concept.title,
            practice_id,
            session_id,
            status: status(sched.stage),
            stage: sched.stage,
            weak_count: sched.weak_count,
            first_weak_ms: sched.first_weak_ms,
            last_ms: concept.outcomes.iter().map(|o| o.ms).max().unwrap_or(0),
            due_ms: sched.due_ms,
            evidence: concept
                .evidence
                .into_iter()
                .take(EVIDENCE_LIMIT)
                .map(|(_, e)| e)
                .collect(),
            concept: key,
        });
    }
    weak_points.sort_by_key(|w| (w.due_ms.unwrap_or(i64::MAX), -w.last_ms));

    let mut timed = vec![];
    {
        let mut stmt = c
            .prepare("SELECT content_json FROM mock_clocks UNION ALL SELECT content_json FROM coding_clocks")
            .map_err(err)?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0)).map_err(err)?;
        for row in rows {
            let Ok(clock) =
                serde_json::from_str::<crate::session_tools::MockClock>(&row.map_err(err)?)
            else {
                continue;
            };
            timed.push(Timed {
                started_ms: clock.started_at,
                used_ms: (clock.budget_ms - clock.left(now_ms)).max(0),
            });
        }
    }
    let mut costs = vec![];
    {
        let mut stmt = c
            .prepare("SELECT created_at,estimated_usd FROM api_usage")
            .map_err(err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Cost {
                    at: r.get(0)?,
                    usd: r.get(1)?,
                })
            })
            .map_err(err)?;
        for row in rows {
            costs.push(row.map_err(err)?);
        }
    }
    Ok(Overview {
        now_ms,
        activity,
        weak_points,
        timed,
        costs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const H: i64 = 3_600_000;
    fn o(hours: i64, good: bool) -> Outcome {
        Outcome {
            ms: hours * H,
            good,
        }
    }
    #[test]
    fn parses_both_timestamp_formats() {
        assert_eq!(parse_ms("1970-01-02T00:00:00Z"), Some(DAY_MS));
        assert_eq!(
            parse_ms("2026-10-15T07:01:00.5Z"),
            parse_ms("2026-10-15 07:01:00").map(|m| m + 500)
        );
        assert_eq!(parse_ms("2000-03-01T00:00:00.000Z"), Some(951_868_800_000));
        assert_eq!(parse_ms("bad"), None);
    }
    #[test]
    fn never_weak_is_not_scheduled() {
        assert_eq!(schedule(&[o(0, true), o(30, true)]), None);
    }
    #[test]
    fn weak_is_due_the_next_day() {
        let s = schedule(&[o(10, false)]).unwrap();
        assert_eq!((s.stage, s.due_ms), (0, Some(34 * H)));
        assert_eq!(status(s.stage), "open");
    }
    #[test]
    fn quick_resubmits_do_not_count_as_reviews() {
        // Failed twice, then accepted minutes later: still due tomorrow.
        let s = schedule(&[o(0, false), o(0, false), o(1, true), o(2, true)]).unwrap();
        assert_eq!((s.stage, s.weak_count, s.due_ms), (0, 2, Some(24 * H)));
    }
    #[test]
    fn spaced_goods_advance_and_resolve() {
        let s = schedule(&[o(0, false), o(24, true)]).unwrap();
        assert_eq!((s.stage, s.due_ms), (1, Some(24 * H + 3 * DAY_MS)));
        assert_eq!(status(1), "improving");
        let s = schedule(&[o(0, false), o(24, true), o(96, true), o(264, true)]).unwrap();
        assert_eq!((s.stage, s.due_ms), (3, None));
        assert_eq!(status(3), "resolved");
    }
    #[test]
    fn a_new_weak_outcome_resets_the_schedule() {
        let s = schedule(&[o(0, false), o(24, true), o(100, false)]).unwrap();
        assert_eq!(
            (s.stage, s.last_weak_ms, s.due_ms),
            (0, 100 * H, Some(124 * H))
        );
    }
    #[test]
    fn logs_runs_and_builds_lc_weak_points() {
        let c = Connection::open_in_memory().unwrap();
        crate::session_tools::migrate(&c).unwrap();
        crate::coding_timer::migrate(&c).unwrap();
        crate::usage::migrate(&c).unwrap();
        crate::interview_store::migrate(&c).unwrap();
        crate::system_design_store::migrate(&c).unwrap();
        migrate(&c).unwrap();
        let id = crate::content::problems()[0].id.clone();
        log_run(&c, &id, "function", "compile_error", 0, 3).unwrap();
        log_run(&c, &id, "function", "wrong_answer", 2, 3).unwrap();
        log_run(&c, &id, "function", "accepted", 3, 3).unwrap();
        let v = overview(&c, parse_ms("2100-01-01T00:00:00Z").unwrap()).unwrap();
        assert_eq!(v.activity.len(), 3);
        assert!(v.activity.iter().all(|a| !a.first_try));
        assert_eq!(v.weak_points.len(), 1);
        let w = &v.weak_points[0];
        assert_eq!(
            (w.status, w.weak_count, w.practice_id.as_str()),
            ("open", 1, id.as_str())
        );
        assert_eq!(w.evidence[0].label, "wrong_answer");
        dismiss(&c, &w.concept).unwrap();
        assert!(overview(&c, 0).unwrap().weak_points.is_empty());
    }
    fn local_db() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().unwrap();
        let c = crate::db::open(&dir.path().join("review.sqlite")).unwrap();
        (dir, c)
    }
    // Authored history fixtures exercise review aggregation, never simulate an AI call.
    fn p1_history(
        c: &Connection,
        at: &str,
        principles: Rating,
        other: Rating,
        turns: &[(&str, &str, bool)],
    ) -> String {
        use crate::{credentials::LlmSettings, interview::*, interview_store as store};
        let config = LlmSettings {
            provider: "claude".into(),
            model: "claude-haiku-5-5".into(),
            timeout_seconds: 60,
        };
        let q = GeneratedQuestion {
            question: "What can this reader observe?".into(),
            background: "An authored transaction scenario.".into(),
            constraints: vec!["READ COMMITTED".into()],
            follow_up_directions: vec!["Private extension".into()],
            evaluation_checklist: vec!["Private checkpoint".into()],
            answer_terms: vec![],
        };
        let q = store::cache_question(c, "cache-selection", &config, &q).unwrap();
        let session = store::start(c, &q.id, &config).unwrap();
        for (index, (role, text, ai)) in turns.iter().enumerate() {
            c.execute("INSERT INTO interview_turns(session_id,role,text,ai_assisted,turn_index,created_at) VALUES(?1,?2,?3,?4,?5,?6)",params![session.id,role,text,ai,index as i64,at]).unwrap();
        }
        let f = Feedback {
            summary: "Local review regression fixture".into(),
            dimensions: DIMENSIONS
                .iter()
                .map(|name| Dimension {
                    name: (*name).into(),
                    rating: if *name == "principles" {
                        principles.clone()
                    } else {
                        other.clone()
                    },
                    evidence: if *name == "principles" && principles == Rating::Incorrect {
                        "The first answer incorrectly claimed FIRST_WRONG.".into()
                    } else if (*name == "principles" && principles == Rating::Good)
                        || (*name != "principles" && other == Rating::Good)
                    {
                        "A correct independent explanation was demonstrated.".into()
                    } else {
                        "This dimension was not assessed.".into()
                    },
                    improvement: "Revisit the first causal mechanism.".into(),
                })
                .collect(),
            strengths: vec![],
            gaps: vec![],
            next_practice_topics: vec!["One local follow-up".into()],
        };
        c.execute(
            "INSERT INTO interview_feedback VALUES(?1,?2)",
            params![session.id, serde_json::to_string(&f).unwrap()],
        )
        .unwrap();
        c.execute(
            "UPDATE interview_sessions SET status='finished' WHERE id=?1",
            [&session.id],
        )
        .unwrap();
        session.id
    }
    #[test]
    fn p1_unassessed_dimensions_do_not_create_weak_points_or_count_as_reviews() {
        let (_dir, c) = local_db();
        p1_history(
            &c,
            "2026-10-01T00:00:00Z",
            Rating::Good,
            Rating::Missing,
            &[("candidate", "Correct independent explanation", false)],
        );
        let v = overview(&c, 0).unwrap();
        assert!(v.weak_points.is_empty());
        assert_eq!(v.activity[0].weak, 0);
        p1_history(
            &c,
            "2026-10-02T00:00:00Z",
            Rating::Incorrect,
            Rating::Missing,
            &[("candidate", "FIRST_WRONG", false)],
        );
        p1_history(
            &c,
            "2026-10-03T00:00:00Z",
            Rating::Missing,
            Rating::Uncertain,
            &[("candidate", "I do not know", false)],
        );
        let v = overview(&c, 0).unwrap();
        let w = &v.weak_points[0];
        assert_eq!(w.weak_count, 1);
        assert_eq!(w.stage, 0);
        p1_history(
            &c,
            "2026-10-04T00:00:00Z",
            Rating::Good,
            Rating::Missing,
            &[("candidate", "Correct independent explanation", false)],
        );
        let v = overview(&c, 0).unwrap();
        let w = &v.weak_points[0];
        assert_eq!(w.weak_count, 1);
        assert_eq!(w.stage, 1);
    }
    #[test]
    fn p1_evidence_uses_the_assessed_claim_not_the_last_answer_or_advice() {
        let (_dir, c) = local_db();
        let session = p1_history(
            &c,
            "2026-10-01T00:00:00Z",
            Rating::Incorrect,
            Rating::Good,
            &[
                ("candidate", "FIRST_WRONG", false),
                ("interviewer", "A different question", false),
                ("candidate", "LAST_CORRECT", false),
                ("interviewer", "An optional extension", false),
                ("candidate", "AI_REFERENCE", true),
            ],
        );
        let v = overview(&c, 0).unwrap();
        let w = &v.weak_points[0];
        let evidence = w.evidence.iter().find(|e| e.kind == "feedback").unwrap();
        assert_eq!(
            evidence.text,
            "The first answer incorrectly claimed FIRST_WRONG."
        );
        assert_eq!(evidence.label, "principles");
        assert_eq!(evidence.session_id.as_deref(), Some(session.as_str()));
        assert!(!evidence.ai_assisted);
        assert!(!w.evidence.iter().any(|e| !e.ai_assisted
            && (e.text.contains("LAST_CORRECT") || e.text.contains("Revisit the first"))));
        assert!(w
            .evidence
            .iter()
            .any(|e| e.ai_assisted && e.text == "AI_REFERENCE"));
    }
    #[test]
    fn p1_ai_only_histories_neither_create_nor_resolve_weak_points() {
        let (_dir, c) = local_db();
        p1_history(
            &c,
            "2026-10-01T00:00:00Z",
            Rating::Incorrect,
            Rating::Partial,
            &[("candidate", "AI authored weak answer", true)],
        );
        assert!(overview(&c, 0).unwrap().weak_points.is_empty());
        p1_history(
            &c,
            "2026-10-02T00:00:00Z",
            Rating::Incorrect,
            Rating::Missing,
            &[("candidate", "FIRST_WRONG", false)],
        );
        p1_history(
            &c,
            "2026-10-03T00:00:00Z",
            Rating::Good,
            Rating::Good,
            &[("candidate", "AI authored correct answer", true)],
        );
        let v = overview(&c, 0).unwrap();
        assert_eq!(v.weak_points[0].stage, 0);
        assert_eq!(v.weak_points[0].weak_count, 1);
    }
    #[test]
    fn cost_rows_retain_unknown_zero_and_nonzero_estimates() {
        let (_dir, c) = local_db();
        for amount in [None, Some(0.0), Some(0.25)] {
            c.execute("INSERT INTO api_usage(provider,model,module,operation,reviewer,estimated_usd) VALUES('claude','local-fixture','p1','feedback',0,?1)",[amount]).unwrap();
        }
        let v = overview(&c, 0).unwrap();
        assert_eq!(v.costs.len(), 3);
        assert_eq!(v.costs.iter().filter(|c| c.usd.is_none()).count(), 1);
        assert!(v.costs.iter().any(|c| c.usd == Some(0.0)));
        assert!(v.costs.iter().any(|c| c.usd == Some(0.25)));
        let wire = serde_json::to_value(v).unwrap();
        assert!(wire["costs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["usd"].is_null()));
    }
}
