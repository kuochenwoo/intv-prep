//! State-machine tests use explicit provider fixtures; live acceptance is a separate example.
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};
use swe_interview_prep::{
    credentials::LlmSettings,
    db,
    interview::{Lang, Rating, Validated},
    system_design::*,
    system_design_store as store,
};
fn config() -> LlmSettings {
    LlmSettings {
        provider: "claude".into(),
        model: "claude-haiku-5-5".into(),
        timeout_seconds: 120,
    }
}
fn question() -> DesignQuestion {
    DesignQuestion {
        question: "设计一个短链接服务".into(),
        background: "训练假设：每天新增100万个链接，读写比100:1。".into(),
        constraints: vec!["记录保存一年，单条200字节".into()],
        follow_up_directions: vec!["内部方向".into()],
        evaluation_checklist: vec!["内部检查点".into()],
    }
}
fn feedback() -> DesignFeedback {
    DesignFeedback {
        summary: "Fixture: unassessed dimensions are missing.".into(),
        assessment: DIMENSIONS
            .iter()
            .map(|name| {
                (
                    name.to_string(),
                    Assessment {
                        rating: Rating::Missing,
                        evidence: "Not assessed in this fixture.".into(),
                        improvement: "Practise one concrete example.".into(),
                    },
                )
            })
            .collect(),
        strengths: vec![],
        gaps: vec![],
        next_practice_topics: vec!["Estimate peak QPS".into()],
    }
}
struct Api {
    fail: Cell<bool>,
    invalid: Cell<bool>,
    calls: Cell<usize>,
    seen: RefCell<Vec<DesignSession>>,
}
impl Api {
    fn new() -> Self {
        Self {
            fail: Cell::new(false),
            invalid: Cell::new(false),
            calls: Cell::new(0),
            seen: RefCell::new(vec![]),
        }
    }
}
impl DesignProvider for Api {
    fn generate_design(&self, _: &Scenario, _: Lang) -> Result<DesignQuestion, String> {
        Ok(question())
    }
    fn design_follow_up(&self, s: &DesignSession) -> Result<DesignFollowUp, String> {
        self.calls.set(self.calls.get() + 1);
        self.seen.borrow_mut().push(s.clone());
        if self.fail.get() {
            return Err("fixture network failure".into());
        }
        Ok(DesignFollowUp {
            observation: if self.invalid.get() {
                String::new()
            } else {
                "Fixture observation".into()
            },
            follow_up_question: "What changes at peak load?".into(),
            suggested_stage: Stage::HighLevel,
        })
    }
    fn design_feedback(&self, s: &DesignSession) -> Result<DesignFeedback, String> {
        self.calls.set(self.calls.get() + 1);
        self.seen.borrow_mut().push(s.clone());
        if self.fail.get() {
            return Err("fixture feedback failure".into());
        }
        let mut f = feedback();
        if self.invalid.get() {
            f.assessment.remove("estimation");
        }
        Ok(f)
    }
}
fn start(c: &rusqlite::Connection) -> DesignSession {
    let q = store::cache_question(c, "url-shortener", Lang::En, &config(), question()).unwrap();
    store::start(c, &q.id, &config()).unwrap()
}
#[test]
fn fifteen_bilingual_scenarios_and_distinct_system_design_prompts() {
    let seeds = scenarios();
    assert_eq!(seeds.len(), 15);
    let unique: std::collections::HashSet<_> = seeds.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(unique.len(), 15);
    for id in [
        "url-shortener",
        "flash-sale",
        "chat",
        "activity-feed",
        "task-queue",
    ] {
        assert!(unique.contains(id), "missing original scenario: {id}");
    }
    for seed in seeds {
        for text in [&seed.text, &seed.en] {
            assert!(!text.title.trim().is_empty());
            assert!(!text.description.trim().is_empty());
            assert!(
                text.functional_requirements
                    .iter()
                    .chain(&text.scale_ranges)
                    .chain(&text.reliability_constraints)
                    .chain(&text.deep_dive_directions)
                    .all(|item| !item.trim().is_empty()),
                "{}",
                seed.id
            );
            assert!(text.functional_requirements.len() >= 3);
            assert!(text.scale_ranges.len() >= 2);
            assert!(text.reliability_constraints.len() >= 2);
            assert!(text.deep_dive_directions.len() >= 3);
        }
        assert!(!serde_json::to_string(&seed.en)
            .unwrap()
            .chars()
            .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)));
    }
    for task in [GENERATE[1], FOLLOW[1], FEEDBACK[1]] {
        let p = prompt(Lang::En, task, &feedback_schema());
        assert!(p.contains("cross-component"));
        assert!(!p.contains("Do not turn fundamentals into full system design"));
        assert!(!p.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)));
    }
    assert_eq!(
        feedback_schema()["properties"]["assessment"]["required"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
}
#[test]
fn expanded_scenarios_have_sources_and_roundtrip_bilingual_caches_offline() {
    let seeds = scenarios();
    let reviews: serde_json::Value =
        serde_json::from_str(include_str!("../../content/system-design/review.json")).unwrap();
    let mut reviewed = std::collections::HashSet::new();
    for entry in reviews.as_array().unwrap() {
        let id = entry["scenarioId"].as_str().unwrap();
        assert!(reviewed.insert(id), "duplicate review: {id}");
        assert!(seeds.iter().any(|s| s.id == id), "orphan review: {id}");
        assert!(!entry["reviewNote"].as_str().unwrap().trim().is_empty());
        assert!(!entry["capacityExample"].as_str().unwrap().trim().is_empty());
        let sources = entry["sources"].as_array().unwrap();
        assert!(!sources.is_empty());
        assert!(sources
            .iter()
            .all(|url| url.as_str().unwrap().starts_with("https://")));
    }
    assert_eq!(reviewed.len(), 10);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("expanded-scenarios.sqlite3");
    let c = db::open(&path).unwrap();
    let mut cached_ids = Vec::new();
    for (index, seed) in seeds.iter().enumerate() {
        if index >= 5 {
            assert!(
                reviewed.contains(seed.id.as_str()),
                "missing review: {}",
                seed.id
            );
        }
        for lang in [Lang::Zh, Lang::En] {
            // Explicit storage fixture, not a generated question or live-provider claim.
            let mut fixture = question();
            fixture.question = if lang == Lang::En {
                &seed.en.title
            } else {
                &seed.text.title
            }
            .clone();
            let cached = store::cache_question(&c, &seed.id, lang, &config(), fixture).unwrap();
            cached_ids.push((cached.id, seed.id.clone(), lang));
        }
    }
    drop(c);
    let c = db::open(&path).unwrap();
    assert_eq!(store::questions(&c).unwrap().len(), 30);
    for (id, seed_id, lang) in cached_ids {
        let restored = store::question(&c, &id).unwrap();
        assert_eq!(restored.seed_id, seed_id);
        assert_eq!(restored.lang, lang);
        let s = store::start(&c, &id, &config()).unwrap();
        assert_eq!(s.question.seed_id, seed_id);
        assert_eq!(s.question.lang, lang);
        assert_eq!(s.stage, Stage::Requirements);
        assert_eq!(s.turns.len(), 1);
        assert_eq!(s.turns[0].role, "interviewer");
        assert_eq!(s.turns[0].text, Stage::Requirements.guide(lang));
    }
}
#[test]
fn stages_preserve_separate_drafts_and_workspace_after_restart() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p2.sqlite3");
    let c = db::open(&path).unwrap();
    let s = start(&c);
    let mut w = s.workspace;
    w.notes = "## Architecture\nAPI -> store".into();
    w.diagram = "flowchart LR\n A-->B".into();
    w.drafts.insert(Stage::Requirements, "scope draft".into());
    store::save_workspace(&c, &s.id, w).unwrap();
    let s = store::change_stage(&c, &s.id, Stage::Estimation).unwrap();
    let mut w = s.workspace;
    w.drafts
        .insert(Stage::Estimation, "1m / 86400 = 11.6 writes/s".into());
    store::save_workspace(&c, &s.id, w).unwrap();
    drop(c);
    let c = db::open(&path).unwrap();
    let s = store::session(&c, &s.id).unwrap();
    assert_eq!(s.stage, Stage::Estimation);
    assert_eq!(s.workspace.drafts.len(), 2);
    assert!(s.workspace.notes.contains("Architecture"));
    assert!(s.workspace.diagram.contains("A-->B"));
    assert!(store::begin_feedback(&c, &s.id).is_err());
    let s = store::change_stage(&c, &s.id, Stage::Requirements).unwrap();
    assert_eq!(s.workspace.drafts[&Stage::Requirements], "scope draft");
    assert!(store::change_stage(&c, &s.id, Stage::Feedback).is_err());
}
#[test]
fn failed_followup_survives_reopen_without_duplicate_answers_or_automatic_stage_changes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("retry.sqlite3");
    let c = db::open(&path).unwrap();
    let s = start(&c);
    let s = store::change_stage(&c, &s.id, Stage::Estimation).unwrap();
    let api = Api::new();
    api.fail.set(true);
    store::begin_answer(
        &c,
        &s.id,
        "1m/day is about 11.6 writes/s; peak factor 10 gives 116 writes/s.",
    )
    .unwrap();
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    let pending = store::session(&c, &s.id).unwrap();
    assert_eq!(pending.status, "awaiting_reply");
    assert!(pending.last_error.is_some());
    assert!(store::begin_answer(&c, &s.id, "duplicate").is_err());
    assert!(store::change_stage(&c, &s.id, Stage::Tradeoffs).is_err());
    assert!(store::save_workspace(&c, &s.id, pending.workspace.clone()).is_err());
    drop(c);
    let c = db::open(&path).unwrap();
    api.fail.set(false);
    let s = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(s.stage, Stage::Estimation);
    assert_eq!(s.suggested_stage, Some(Stage::HighLevel));
    assert_eq!(s.turns.iter().filter(|t| t.role == "candidate").count(), 1);
    assert!(s.workspace.drafts.is_empty());
    assert!(s.last_error.is_none());
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    let s = store::change_stage(&c, &s.id, Stage::HighLevel).unwrap();
    assert_eq!(s.stage, Stage::HighLevel);
}
#[test]
fn invalid_followup_and_eight_dimension_feedback_are_not_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("invalid.sqlite3")).unwrap();
    let s = start(&c);
    let api = Api::new();
    api.invalid.set(true);
    store::begin_answer(&c, &s.id, "Scope").unwrap();
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    assert_eq!(store::session(&c, &s.id).unwrap().turns.len(), 2);
    api.invalid.set(false);
    store::retry_pending(&c, &s.id, &api).unwrap();
    store::begin_feedback(&c, &s.id).unwrap();
    api.invalid.set(true);
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    let s = store::session(&c, &s.id).unwrap();
    assert_eq!(s.status, "feedback_pending");
    assert!(s.feedback.is_none());
    assert_eq!(s.stage, Stage::Requirements);
    api.invalid.set(false);
    let s = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(s.status, "finished");
    assert_eq!(s.stage, Stage::Feedback);
    assert_eq!(s.feedback.unwrap().assessment.len(), 8);
    assert!(store::begin_answer(&c, &s.id, "after finish").is_err());
    let mut f = feedback();
    f.assessment.insert(
        "unexpected".into(),
        Assessment {
            rating: Rating::Good,
            evidence: "x".into(),
            improvement: "y".into(),
        },
    );
    assert!(f.validate().is_err());
}
#[test]
fn failed_feedback_restores_and_cache_reuse_is_offline_and_pinned() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("feedback.sqlite3");
    let c = db::open(&path).unwrap();
    let s = start(&c);
    let api = Api::new();
    api.fail.set(true);
    store::begin_feedback(&c, &s.id).unwrap();
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    drop(c);
    let c = db::open(&path).unwrap();
    api.fail.set(false);
    let restored = store::retry_pending(&c, &s.id, &api).unwrap();
    assert!(restored.feedback.is_some());
    assert_eq!(restored.provider, "claude");
    assert_eq!(restored.model, "claude-haiku-5-5");
    assert_eq!(restored.question.lang, Lang::En);
    let previous_calls = api.calls.get();
    let q = store::questions(&c).unwrap();
    let reused = store::start(&c, &q[0].id, &config()).unwrap();
    assert_ne!(reused.id, s.id);
    assert_eq!(reused.question.id, s.question.id);
    assert_eq!(api.calls.get(), previous_calls);
    assert_eq!(store::sessions(&c).unwrap().len(), 2);
}
#[test]
fn invalid_workspace_bounds_and_ids_do_not_write_partial_state() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("limits.sqlite3")).unwrap();
    let s = start(&c);
    let mut w = s.workspace.clone();
    w.notes = "x".repeat(20001);
    assert!(store::save_workspace(&c, &s.id, w).is_err());
    assert!(store::session(&c, &s.id)
        .unwrap()
        .workspace
        .notes
        .is_empty());
    let w = Workspace {
        notes: String::new(),
        diagram: String::new(),
        drafts: BTreeMap::from([(Stage::Feedback, "bad".into())]),
    };
    assert!(store::save_workspace(&c, &s.id, w).is_err());
    assert!(store::begin_answer(&c, &s.id, " ").is_err());
    assert!(store::session(&c, "bad-id").is_err());
    assert!(store::cache_question(&c, "bad-seed", Lang::Zh, &config(), question()).is_err());
}
#[test]
fn p1_and_coding_data_remain_untouched_by_p2() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("isolation.sqlite3")).unwrap();
    swe_interview_prep::db::save(&c, "lc-1", "function", "original code").unwrap();
    let p1 = swe_interview_prep::interview::GeneratedQuestion {
        question: "P1 unchanged".into(),
        background: "Fixture".into(),
        constraints: vec!["One mechanism".into()],
        follow_up_directions: vec!["One boundary".into()],
        evaluation_checklist: vec!["Evidence".into()],
    };
    let q =
        swe_interview_prep::interview_store::cache_question(&c, "cache-selection", &config(), &p1)
            .unwrap();
    let original = swe_interview_prep::interview_store::start(&c, &q.id, &config()).unwrap();
    let s = start(&c);
    store::change_stage(&c, &s.id, Stage::Tradeoffs).unwrap();
    store::begin_feedback(&c, &s.id).unwrap();
    store::retry_pending(&c, &s.id, &Api::new()).unwrap();
    assert_eq!(
        swe_interview_prep::interview_store::session(&c, &original.id)
            .unwrap()
            .status,
        "active"
    );
    assert_eq!(
        swe_interview_prep::interview_store::questions(&c)
            .unwrap()
            .len(),
        1
    );
    assert_eq!(db::load(&c).unwrap()[0].code, "original code");
    swe_interview_prep::interview_store::clear_history(&c).unwrap();
    assert_eq!(store::sessions(&c).unwrap().len(), 1);
    // Clearing system design history leaves coding progress alone, and vice versa above.
    store::clear_history(&c).unwrap();
    assert!(store::sessions(&c).unwrap().is_empty());
    assert!(store::questions(&c).unwrap().is_empty());
    assert_eq!(db::load(&c).unwrap()[0].code, "original code");
}
#[test]
fn deleting_one_design_session_keeps_the_others_and_the_cache() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("delete.sqlite3")).unwrap();
    let first = start(&c);
    let second = start(&c);
    store::delete_session(&c, &first.id).unwrap();
    let left: Vec<_> = store::sessions(&c)
        .unwrap()
        .into_iter()
        .map(|s| s.id)
        .collect();
    assert_eq!(left, vec![second.id]);
    assert_eq!(store::questions(&c).unwrap().len(), 2);
}

#[test]
fn conversation_limit_still_allows_finishing_without_a_model_followup() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("length.sqlite3")).unwrap();
    let mut s = start(&c);
    while s.turns.len() < 100 {
        let next = if s.stage == Stage::Requirements {
            Stage::Estimation
        } else {
            Stage::Requirements
        };
        s = store::change_stage(&c, &s.id, next).unwrap();
    }
    assert!(store::begin_answer(&c, &s.id, "Another round").is_err());
    assert!(store::change_stage(&c, &s.id, Stage::Tradeoffs).is_err());
    store::begin_feedback(&c, &s.id).unwrap();
    assert_eq!(
        store::retry_pending(&c, &s.id, &Api::new()).unwrap().status,
        "finished"
    );
}
