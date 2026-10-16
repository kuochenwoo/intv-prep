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
        answer_terms: vec![],
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
    reference_turns: Cell<usize>,
}
impl FixtureProvider {
    fn new() -> Self {
        Self {
            fail: Cell::new(false),
            generated: Cell::new(0),
            turns: Cell::new(0),
            summaries: Cell::new(0),
            reference_turns: Cell::new(0),
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
    fn reference_answer(
        &self,
        _: &GeneratedQuestion,
        target: &str,
        turns: &[Turn],
    ) -> Result<ReferenceAnswer, String> {
        self.reference_turns.set(turns.len());
        if self.fail.get() {
            return Err("测试参考请求失败".into());
        }
        if turns.is_empty() {
            assert_eq!(target, question().question);
        } else {
            assert_eq!(target, turns.last().unwrap().text);
        }
        assert!(!turns.iter().any(|t| t.text.contains("UNSUBMITTED_DRAFT")));
        Ok(ReferenceAnswer {
            answer: format!("测试参考，仅针对：{target}"),
            key_points: vec!["测试要点".into()],
        })
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
fn expanded_topics_have_bilingual_content_and_reviewed_sources() {
    let seeds = topics();
    let reviews: serde_json::Value =
        serde_json::from_str(include_str!("../../content/deep-dive/review.json")).unwrap();
    let reviews = reviews.as_array().unwrap();
    let mut reviewed = std::collections::HashSet::new();
    for review in reviews {
        let id = review["topicId"].as_str().unwrap();
        assert!(reviewed.insert(id), "duplicate review: {id}");
        assert!(
            seeds.iter().any(|seed| seed.id == id),
            "orphan review: {id}"
        );
        assert!(
            !review["reviewNote"].as_str().unwrap().trim().is_empty(),
            "{id}"
        );
        let sources = review["sources"].as_array().unwrap();
        assert!(!sources.is_empty(), "{id}");
        assert!(
            sources
                .iter()
                .all(|url| url.as_str().unwrap().starts_with("https://")),
            "{id}"
        );
    }
    assert_eq!(reviewed.len(), seeds.len());
    let mut titles = std::collections::HashSet::new();
    for seed in &seeds {
        assert!(titles.insert(&seed.title), "duplicate title: {}", seed.id);
        assert_eq!(seed.level, "mid-to-senior");
        assert!(seed.en.is_some(), "missing English content: {}", seed.id);
        for lang in [Lang::Zh, Lang::En] {
            let localized = seed.localized(lang);
            assert!(!localized.title.trim().is_empty(), "{}", seed.id);
            assert!(
                localized
                    .focus
                    .iter()
                    .chain(&localized.scenario_constraints)
                    .all(|value| !value.trim().is_empty()),
                "{}",
                seed.id
            );
        }
        assert!(
            reviewed.contains(seed.id.as_str()),
            "missing source record: {}",
            seed.id
        );
        let review = reviews
            .iter()
            .find(|review| review["topicId"] == seed.id)
            .unwrap();
        assert_eq!(
            serde_json::to_value(&seed.sources).unwrap(),
            review["sources"]
        );
        assert!(!serde_json::to_string(seed.en.as_ref().unwrap())
            .unwrap()
            .chars()
            .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)));
        assert!(seed.sources.iter().all(|url| url.starts_with("https://")));
    }
    for form in [
        QuestionForm::Foundation,
        QuestionForm::Scenario,
        QuestionForm::Troubleshooting,
    ] {
        assert!(seeds
            .iter()
            .any(|seed| seed.question_form == form && seed.priority == TopicPriority::Core));
    }
    assert!(seeds
        .iter()
        .any(|seed| seed.priority == TopicPriority::Advanced));
    for (category, minimum) in [
        ("Database", 7),
        ("Networking", 5),
        ("Operating Systems", 4),
        ("Concurrency", 6),
        ("Runtime", 4),
        ("Redis", 3),
        ("Security", 7),
        ("Distributed Systems", 8),
        ("Data Structures", 5),
        ("Engineering", 6),
        ("Business Scenarios", 7),
        ("Search and Storage", 4),
    ] {
        assert!(
            seeds
                .iter()
                .filter(|seed| seed.category == category)
                .count()
                >= minimum,
            "missing fundamentals coverage: {category}"
        );
    }
}
#[test]
fn legacy_topic_metadata_defaults_and_localization_preserve_authoring_contracts() {
    let seed = topic("mysql-index-structure").unwrap();
    assert_eq!(seed.question_form, QuestionForm::Foundation);
    let english = seed.localized(Lang::En);
    assert_eq!(english.question_form, seed.question_form);
    assert_eq!(english.priority, seed.priority);
    assert_eq!(english.sources, seed.sources);
    assert_ne!(english.title, seed.title);
    let mut legacy = serde_json::to_value(&seed).unwrap();
    for field in ["questionForm", "priority", "sources"] {
        legacy.as_object_mut().unwrap().remove(field);
    }
    let legacy: Topic = serde_json::from_value(legacy).unwrap();
    assert_eq!(legacy.question_form, QuestionForm::Scenario);
    assert_eq!(legacy.priority, TopicPriority::Core);
    assert!(legacy.sources.is_empty());
    for id in [
        "volatile-counter",
        "safe-publication",
        "condition-wait",
        "thread-pool-starvation",
        "cas-aba",
        "mutable-hash-key",
        "resource-cleanup",
    ] {
        let ported = topic(id).unwrap();
        assert!(ported
            .scenario_constraints
            .iter()
            .all(|line| !line.contains("Java 21")));
    }
}

#[test]
fn cache_incident_seed_keeps_diagnoses_private_and_asks_for_evidence() {
    let seed = topic("redis-cache-failures").unwrap();
    assert_eq!(seed.question_form, QuestionForm::Troubleshooting);
    assert_eq!(seed.title, "缓存未命中与数据库压力");
    assert_eq!(
        seed.localized(Lang::En).title,
        "Cache Misses and Database Pressure"
    );
    assert!(seed.focus.iter().any(|text| text.contains("穿透")));
    assert!(seed
        .scenario_constraints
        .iter()
        .any(|text| text.contains("证据")));
    for label in ["穿透", "击穿", "雪崩"] {
        assert!(!seed.title.contains(label));
    }
}

#[test]
fn foundation_subject_names_do_not_replace_private_answer_propositions() {
    let mut q = GeneratedQuestion {
        question: "为什么数据库常用 B+ 树索引？".into(),
        background: "讨论磁盘上的数据库索引。".into(),
        constraints: vec!["比较一次等值查询的访问路径。".into()],
        follow_up_directions: vec!["范围查询的访问方式。".into()],
        evaluation_checklist: vec!["解释一次查询需要访问的页数。".into()],
        answer_terms: vec!["较高扇出降低树高".into(), "减少查询所需页访问".into()],
    };
    q.validate_fresh().unwrap();
    q.background.push_str("较高扇出降低树高");
    assert!(q.validate_fresh().is_err());
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

#[test]
fn references_are_per_question_cached_and_do_not_change_answers_or_session_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("prep.sqlite3");
    let c = db::open(&path).unwrap();
    let config = LlmSettings::default();
    let api = FixtureProvider::new();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    store::save_draft(&c, &s.id, "UNSUBMITTED_DRAFT").unwrap();
    api.fail.set(true);
    assert!(store::generate_reference(&c, &s.id, 0, &api).is_err());
    assert!(store::session(&c, &s.id).unwrap().references.is_empty());
    api.fail.set(false);
    let original = store::generate_reference(&c, &s.id, 0, &api).unwrap();
    assert_eq!(api.reference_turns.get(), 0);
    let unchanged = store::session(&c, &s.id).unwrap();
    assert_eq!(unchanged.status, "active");
    assert_eq!(unchanged.draft, "UNSUBMITTED_DRAFT");
    assert!(unchanged.turns.is_empty());
    for _ in 0..2 {
        store::begin_answer(&c, &s.id, "我使用 Redis").unwrap();
        store::retry_pending(&c, &s.id, &api).unwrap();
    }
    let followup = store::generate_reference(&c, &s.id, 2, &api).unwrap();
    assert_eq!(api.reference_turns.get(), 2); // No later answer/turn leaks into an old reference.
    assert_ne!(original.answer, followup.answer);
    assert!(store::generate_reference(&c, &s.id, 1, &api).is_err()); // Candidate message.
    assert!(store::generate_reference(&c, &s.id, 999, &api).is_err());
    store::begin_answer(&c, &s.id, "继续回答").unwrap();
    assert!(store::generate_reference(&c, &s.id, 4, &api).is_err()); // Pending request.
    store::retry_pending(&c, &s.id, &api).unwrap();
    store::begin_feedback(&c, &s.id).unwrap();
    store::retry_pending(&c, &s.id, &api).unwrap();
    drop(c);
    let c = db::open(&path).unwrap();
    api.fail.set(true);
    assert_eq!(
        store::generate_reference(&c, &s.id, 2, &api)
            .unwrap()
            .answer,
        followup.answer
    );
    assert!(store::reference(&c, &s.id, 0).unwrap().is_some());
    let restored = store::session(&c, &s.id).unwrap();
    assert_eq!(restored.references.len(), 2);
    assert_eq!(restored.turns.len(), 6);
    assert!(!restored.turns.iter().any(|t| t.text.contains("测试参考")));
    assert_eq!(restored.status, "finished");
    store::delete_session(&c, &s.id).unwrap();
    let count: i64 = c
        .query_row("SELECT count(*) FROM interview_references", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn ai_reference_submits_current_question_then_continues_interview() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("prep.sqlite3")).unwrap();
    let config = LlmSettings::default();
    let api = FixtureProvider::new();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    store::save_draft(&c, &s.id, "UNSUBMITTED_DRAFT").unwrap();
    assert!(store::submit_reference(&c, &s.id, 0, &api).is_err());
    assert_eq!(
        store::session(&c, &s.id).unwrap().draft,
        "UNSUBMITTED_DRAFT"
    );
    store::save_draft(&c, &s.id, "").unwrap();
    api.fail.set(true);
    assert!(store::submit_reference(&c, &s.id, 0, &api).is_err());
    let failed = store::session(&c, &s.id).unwrap();
    assert_eq!(failed.status, "active");
    assert!(failed.turns.is_empty());
    api.fail.set(false);
    let first = store::submit_reference(&c, &s.id, 0, &api).unwrap();
    assert_eq!(first.status, "active");
    assert_eq!(first.turns.len(), 2);
    assert_eq!(first.turns[0].role, "candidate");
    assert!(first.turns[0].ai_assisted);
    assert_eq!(first.turns[0].text, first.references[0].content.answer);
    assert_eq!(first.turns[1].role, "interviewer");
    assert!(!first.turns[1].ai_assisted);
    assert!(store::submit_reference(&c, &s.id, 0, &api).is_err()); // Stale click.
    let second = store::submit_reference(&c, &s.id, 2, &api).unwrap();
    assert_eq!(second.turns.len(), 4);
    assert!(second.turns[2].ai_assisted);
    assert_eq!(api.reference_turns.get(), 2);
    assert!(second.turns[2].text.contains(&first.turns[1].text));
    store::begin_answer(&c, &s.id, "我再自行补充方案").unwrap();
    let third = store::retry_pending(&c, &s.id, &api).unwrap();
    assert!(!third.turns[4].ai_assisted);
    store::begin_feedback(&c, &s.id).unwrap();
    let finished = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(finished.status, "finished");
    assert_eq!(finished.turns.iter().filter(|t| t.ai_assisted).count(), 2);
    assert!(store::submit_reference(&c, &s.id, 6, &api).is_err());
}

#[test]
fn ai_answer_is_saved_before_failed_followup_and_retry_never_duplicates_it() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("prep.sqlite3");
    let c = db::open(&path).unwrap();
    let config = LlmSettings::default();
    let api = FixtureProvider::new();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    // Existing generation cache allows the next request to fail specifically at followup.
    let cached = store::generate_reference(&c, &s.id, 0, &api).unwrap();
    api.fail.set(true);
    assert!(store::submit_reference(&c, &s.id, 0, &api).is_err());
    drop(c);
    let c = db::open(&path).unwrap();
    let pending = store::session(&c, &s.id).unwrap();
    assert_eq!(pending.status, "awaiting_reply");
    assert_eq!(pending.turns.len(), 1);
    assert!(pending.turns[0].ai_assisted);
    assert_eq!(pending.turns[0].text, cached.answer);
    assert!(store::submit_reference(&c, &s.id, 0, &api).is_err());
    api.fail.set(false);
    let next = store::retry_pending(&c, &s.id, &api).unwrap();
    assert_eq!(next.turns.len(), 2);
    assert_eq!(next.references.len(), 1);
    assert!(next.draft.is_empty());
    assert_eq!(next.turns.iter().filter(|t| t.ai_assisted).count(), 1);
}

#[test]
fn ai_provenance_migration_preserves_legacy_turns_and_is_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("prep.sqlite3");
    let c = db::open(&path).unwrap();
    let config = LlmSettings::default();
    let q = store::cache_question(&c, "cache-selection", &config, &question()).unwrap();
    let s = store::start(&c, &q.id, &config).unwrap();
    store::begin_answer(&c, &s.id, "旧版本的手写回答").unwrap();
    c.execute("ALTER TABLE interview_turns DROP COLUMN ai_assisted", [])
        .unwrap();
    drop(c);
    for _ in 0..2 {
        let c = db::open(&path).unwrap();
        let restored = store::session(&c, &s.id).unwrap();
        assert_eq!(restored.status, "awaiting_reply");
        assert_eq!(restored.turns[0].text, "旧版本的手写回答");
        assert!(!restored.turns[0].ai_assisted);
    }
    let legacy: Turn = serde_json::from_value(
        serde_json::json!({"role":"candidate","text":"旧回答","timestamp":"test"}),
    )
    .unwrap();
    assert!(!legacy.ai_assisted);
}
