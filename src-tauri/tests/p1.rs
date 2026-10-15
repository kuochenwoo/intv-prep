use std::cell::Cell;
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::*,
    interview_store as store, llm,
};
fn question() -> GeneratedQuestion {
    GeneratedQuestion {
        question: "测试夹具：如何选择缓存？".into(),
        background: "测试背景".into(),
        constraints: vec!["多实例部署".into()],
        follow_up_directions: vec!["PRIVATE_DIRECTION".into()],
        evaluation_checklist: vec!["PRIVATE_CHECKLIST".into()],
    }
}
fn feedback() -> Feedback {
    Feedback {
        summary: "测试夹具反馈".into(),
        dimensions: DIMENSIONS
            .iter()
            .map(|n| Dimension {
                name: n.to_string(),
                rating: Rating::Partial,
                evidence: "测试回答提到了缓存".into(),
                improvement: "测试建议：解释一致性边界".into(),
            })
            .collect(),
        strengths: vec![],
        gaps: vec!["故障恢复待补充".into()],
        next_practice_topics: vec!["幂等与重试".into()],
    }
}
struct FixtureProvider {
    fail: Cell<bool>,
    generated: Cell<usize>,
    turns: Cell<usize>,
    summaries: Cell<usize>,
}
impl FixtureProvider {
    fn new() -> Self {
        Self {
            fail: Cell::new(false),
            generated: Cell::new(0),
            turns: Cell::new(0),
            summaries: Cell::new(0),
        }
    }
}
impl LlmProvider for FixtureProvider {
    fn generate_question(&self, _: &Topic) -> Result<GeneratedQuestion, String> {
        self.generated.set(self.generated.get() + 1);
        Ok(question())
    }
    fn next_turn(&self, _: &GeneratedQuestion, turns: &[Turn]) -> Result<FollowUp, String> {
        self.turns.set(self.turns.get() + 1);
        assert_eq!(turns.last().unwrap().role, "candidate");
        if self.fail.get() {
            Err("测试网络/解析失败".into())
        } else {
            Ok(FollowUp {
                observation: "测试观察".into(),
                follow_up_question: "测试追问：Redis 不可用时如何保护数据库？".into(),
                should_end: false,
            })
        }
    }
    fn summarize(&self, _: &GeneratedQuestion, turns: &[Turn]) -> Result<Feedback, String> {
        self.summaries.set(self.summaries.get() + 1);
        assert!(turns.iter().any(|t| t.role == "candidate"));
        if self.fail.get() {
            Err("测试反馈失败".into())
        } else {
            Ok(feedback())
        }
    }
}
#[test]
fn topics_and_contracts_are_valid_and_feedback_has_five_evidence_based_dimensions() {
    let topics = topics();
    assert!(topics.len() >= 10);
    let mut ids = std::collections::HashSet::new();
    for t in topics {
        assert!(ids.insert(t.id));
        assert!(!t.focus.is_empty());
        assert!(!t.scenario_constraints.is_empty());
    }
    question().validate().unwrap();
    feedback().validate().unwrap();
    let mut f = feedback();
    f.dimensions[0].name = f.dimensions[1].name.clone();
    assert!(f.validate().is_err());
    let mut v = serde_json::to_value(feedback()).unwrap();
    v["dimensions"][0]["rating"] = serde_json::json!("100");
    assert!(serde_json::from_value::<Feedback>(v).is_err());
    assert!(llm::feedback_schema().to_string().contains("uncertain"));
}
#[test]
fn cache_reuse_is_offline_and_private_checkpoints_never_enter_candidate_view() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("prep.sqlite3")).unwrap();
    let api = FixtureProvider::new();
    let config = LlmSettings::default();
    let q = api
        .generate_question(&topic("cache-selection").unwrap())
        .unwrap();
    let cached = store::cache_question(&c, "cache-selection", &config, &q).unwrap();
    for _ in 0..2 {
        let session = store::start(&c, &cached.id, &config).unwrap();
        let public = serde_json::to_string(&session).unwrap();
        assert!(!public.contains("PRIVATE_CHECKLIST"));
        assert!(!public.contains("PRIVATE_DIRECTION"));
        assert!(session.turns.is_empty());
    }
    assert_eq!(api.generated.get(), 1);
    assert_eq!(api.turns.get(), 0);
    assert_eq!(store::questions(&c).unwrap().len(), 1);
    assert_eq!(store::sessions(&c).unwrap().len(), 2);
}
#[test]
fn pending_answer_and_feedback_survive_reopen_and_retry_without_duplicate_turns() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("prep.sqlite3");
    let config = LlmSettings::default();
    let api = FixtureProvider::new();
    let c = db::open(&path).unwrap();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    store::save_draft(&c, &s.id, "先使用 Redis").unwrap();
    store::begin_answer(&c, &s.id, "先使用 Redis").unwrap();
    api.fail.set(true);
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    assert!(store::begin_answer(&c, &s.id, "重复提交").is_err());
    assert!(store::save_draft(&c, &s.id, "覆盖待重试内容").is_err());
    drop(c);
    let c = db::open(&path).unwrap();
    let restored = store::session(&c, &s.id).unwrap();
    assert_eq!(restored.status, "awaiting_reply");
    assert_eq!(restored.draft, "先使用 Redis");
    assert_eq!(restored.turns.len(), 1);
    assert!(restored.last_error.is_some());
    api.fail.set(false);
    let next = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(next.status, "active");
    assert_eq!(next.turns.len(), 2);
    assert!(next.draft.is_empty());
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    store::save_draft(&c, &s.id, "尚未提交").unwrap();
    assert!(store::begin_feedback(&c, &s.id).is_err());
    store::save_draft(&c, &s.id, "").unwrap();
    store::begin_feedback(&c, &s.id).unwrap();
    api.fail.set(true);
    assert!(store::retry_pending(&c, &s.id, &api).is_err());
    drop(c);
    let c = db::open(&path).unwrap();
    assert_eq!(
        store::session(&c, &s.id).unwrap().status,
        "feedback_pending"
    );
    api.fail.set(false);
    let finished = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(finished.status, "finished");
    assert!(finished.feedback.is_some());
    assert_eq!(finished.turns.len(), 2);
    assert!(store::save_draft(&c, &s.id, "after finish").is_err());
    assert!(store::begin_feedback(&c, &s.id).is_err());
}
#[test]
fn settings_and_drafts_restore_and_clear_only_affects_interview_data() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("prep.sqlite3");
    let c = db::open(&path).unwrap();
    db::save(&c, "lc-1", "function", "LC draft").unwrap();
    let mut config = LlmSettings::default();
    config.model = "test-model".into();
    credentials::save_settings(&c, &config).unwrap();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    store::save_draft(&c, &s.id, "恢复这份回答").unwrap();
    drop(c);
    let c = db::open(&path).unwrap();
    assert_eq!(store::session(&c, &s.id).unwrap().draft, "恢复这份回答");
    assert_eq!(credentials::settings(&c).unwrap().model, "test-model");
    let raw: String = c
        .query_row(
            "SELECT value FROM settings WHERE key='llm_config'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(!raw.contains("apiKey"));
    assert!(!raw.contains("api_key"));
    store::delete_session(&c, &s.id).unwrap();
    assert_eq!(store::questions(&c).unwrap().len(), 1);
    store::start(&c, &q.id, &config).unwrap();
    store::clear_history(&c).unwrap();
    assert!(store::questions(&c).unwrap().is_empty());
    assert!(store::sessions(&c).unwrap().is_empty());
    assert_eq!(db::load(&c).unwrap()[0].code, "LC draft");
    assert_eq!(credentials::settings(&c).unwrap().model, "test-model");
}
#[test]
fn invalid_states_and_twenty_round_limit_are_enforced() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("prep.sqlite3")).unwrap();
    let config = LlmSettings::default();
    let api = FixtureProvider::new();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    assert!(store::begin_feedback(&c, &s.id).is_err());
    assert!(store::begin_answer(&c, &s.id, "").is_err());
    assert!(store::save_draft(&c, &s.id, &"x".repeat(6001)).is_err());
    assert!(store::start(&c, "missing", &config).is_err());
    for _ in 0..20 {
        store::begin_answer(&c, &s.id, "有效测试回答").unwrap();
        store::retry_pending(&c, &s.id, &api).unwrap();
    }
    assert!(store::begin_answer(&c, &s.id, "第21轮").is_err());
    assert_eq!(store::session(&c, &s.id).unwrap().turns.len(), 40);
}
#[test]
#[ignore = "Uses OS credential store; writes and deletes an isolated dummy test credential"]
fn os_keyring_roundtrip() {
    let account = format!(
        "test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let entry = keyring::Entry::new("local.swe.interview-prep.keychain-test", &account).unwrap();
    entry
        .set_password("temporary-fixture-not-an-api-key")
        .unwrap();
    let value = entry.get_password().unwrap();
    entry.delete_credential().unwrap();
    assert_eq!(value, "temporary-fixture-not-an-api-key");
    assert!(matches!(entry.get_password(), Err(keyring::Error::NoEntry)));
}

#[test]
fn temporary_env_keys_are_private_atomic_and_provider_specific() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".env");
    credentials::write_env_key(&path, "deepseek", "fixture-deepseek-not-real-key").unwrap();
    credentials::write_env_key(&path, "claude", "fixture-claude-not-real-key").unwrap();
    assert_eq!(
        credentials::read_env_key(&path, "deepseek").unwrap(),
        "fixture-deepseek-not-real-key"
    );
    assert_eq!(
        credentials::read_env_key(&path, "claude").unwrap(),
        "fixture-claude-not-real-key"
    );
    credentials::write_env_key(&path, "deepseek", "fixture-updated-not-real-key").unwrap();
    assert_eq!(
        credentials::read_env_key(&path, "claude").unwrap(),
        "fixture-claude-not-real-key"
    );
    credentials::write_env_key(&path, "deepseek", "").unwrap();
    assert!(credentials::read_env_key(&path, "deepseek").is_err());
    assert_eq!(
        credentials::read_env_key(&path, "claude").unwrap(),
        "fixture-claude-not-real-key"
    );
    assert!(credentials::write_env_key(&path, "deepseek", "bad\nINJECTED=value").is_err());
    assert!(credentials::write_env_key(&path, "unknown", "fixture-not-real-key").is_err());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}
