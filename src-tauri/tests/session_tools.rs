//! Persistence/export tests use local authored fixtures; no model success is simulated.
use serde_json::{json, Value};
use swe_interview_prep::{
    credentials::LlmSettings,
    db,
    interview::{GeneratedQuestion, Lang},
    interview_store as p1, session_tools as tools,
    system_design::{DesignQuestion, Workspace},
    system_design_store as p2, usage,
};
fn config() -> LlmSettings {
    LlmSettings {
        provider: "claude".into(),
        model: "claude-haiku-5-5".into(),
        timeout_seconds: 120,
    }
}
fn p1_session(c: &rusqlite::Connection) -> String {
    let q = GeneratedQuestion {
        question: "Why is the second read different?".into(),
        background: "A transaction reads twice.".into(),
        constraints: vec!["READ COMMITTED".into()],
        follow_up_directions: vec!["PRIVATE_DIRECTION".into()],
        evaluation_checklist: vec!["PRIVATE_CHECKLIST".into()],
        answer_terms: vec![],
    };
    let q = p1::cache_question(c, "cache-selection", &config(), &q).unwrap();
    p1::start(c, &q.id, &config()).unwrap().id
}
#[test]
fn timer_is_optional_explicit_and_durable() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("test.sqlite");
    let c = db::open(&path).unwrap();
    let id = p1_session(&c);
    assert!(tools::clock(&c, "p1", &id).unwrap().is_none());
    assert!(tools::timing(&c, "p1", &id, "start", Some(45)).is_err());
    let timer = tools::timing(&c, "p1", &id, "start", Some(10))
        .unwrap()
        .unwrap();
    assert_eq!(timer.budget_ms, 600000);
    assert!(tools::timing(&c, "p1", &id, "start", Some(20)).is_err());
    tools::timing(&c, "p1", &id, "pause", None).unwrap();
    p1::save_draft(&c, &id, "preserved draft").unwrap();
    drop(c);
    let c = db::open(&path).unwrap();
    let paused = tools::clock(&c, "p1", &id).unwrap().unwrap();
    assert!(paused.deadline_ms.is_none());
    assert_eq!(p1::session(&c, &id).unwrap().draft, "preserved draft");
    tools::timing(&c, "p1", &id, "resume", None).unwrap();
    tools::stop(&c, "p1", &id).unwrap();
    assert!(tools::clock(&c, "p1", &id)
        .unwrap()
        .unwrap()
        .ended_at
        .is_some());
    assert!(tools::timing(&c, "p1", "missing", "start", Some(10)).is_err());
}
#[test]
fn p1_exports_exclude_private_metadata_settings_and_drafts() {
    let d = tempfile::tempdir().unwrap();
    let c = db::open(&d.path().join("test.sqlite")).unwrap();
    let id = p1_session(&c);
    p1::save_draft(&c, &id, "UNSUBMITTED_DRAFT").unwrap();
    c.execute(
        "INSERT INTO settings VALUES('unrelated','SETTINGS_SENTINEL')",
        [],
    )
    .unwrap();
    let file = tools::export(&c, "p1", &id, "json", "en").unwrap();
    let value: Value = serde_json::from_str(&file.text).unwrap();
    assert_eq!(value["contentLanguage"], "en");
    assert_eq!(value["schemaVersion"], 1);
    for word in [
        "PRIVATE_CHECKLIST",
        "PRIVATE_DIRECTION",
        "UNSUBMITTED_DRAFT",
        "SETTINGS_SENTINEL",
        "apiKey",
    ] {
        assert!(!file.text.contains(word));
    }
    let md = tools::export(&c, "p1", &id, "md", "zh").unwrap();
    assert!(md.text.contains("面试记录"));
    assert!(md.text.contains("Why is the second read different?"));
    assert!(tools::export(&c, "p1", &id, "pdf", "en").is_err());
}
#[test]
fn p2_export_keeps_notes_diagram_language_and_assistance() {
    let d = tempfile::tempdir().unwrap();
    let c = db::open(&d.path().join("test.sqlite")).unwrap();
    let q = DesignQuestion {
        question: "Design short links".into(),
        background: "One million links daily".into(),
        constraints: vec!["One year retention".into()],
        follow_up_directions: vec!["PRIVATE_DIRECTION".into()],
        evaluation_checklist: vec!["PRIVATE_CHECKLIST".into()],
    };
    let q = p2::cache_question(&c, "url-shortener", Lang::En, &config(), q).unwrap();
    let s = p2::start(&c, &q.id, &config()).unwrap();
    let workspace = Workspace {
        notes: "Persist links before acknowledging".into(),
        diagram: "flowchart LR\nA-->B".into(),
        drafts: [(s.stage, "UNSUBMITTED_DRAFT".into())]
            .into_iter()
            .collect(),
    };
    p2::save_workspace(&c, &s.id, workspace).unwrap();
    // Preserve an assisted historical turn without pretending to perform an AI request.
    let mut saved = serde_json::to_value(p2::session(&c, &s.id).unwrap()).unwrap();
    saved["turns"].as_array_mut().unwrap().push(json!({"role":"candidate","text":"Authored assisted-history fixture","stage":"requirements","observation":null,"aiAssisted":true}));
    c.execute(
        "UPDATE sd_sessions SET content_json=?1 WHERE id=?2",
        rusqlite::params![saved.to_string(), s.id],
    )
    .unwrap();
    let json = tools::export(&c, "p2", &s.id, "json", "zh").unwrap();
    let v: Value = serde_json::from_str(&json.text).unwrap();
    assert_eq!(v["contentLanguage"], "en");
    assert_eq!(v["session"]["turns"][1]["aiAssisted"], true);
    assert_eq!(v["session"]["workspace"]["diagram"], "flowchart LR\nA-->B");
    assert!(!json.text.contains("UNSUBMITTED_DRAFT"));
    assert!(!json.text.contains("PRIVATE_"));
    let md = tools::export(&c, "p2", &s.id, "md", "en").unwrap();
    assert!(md.text.contains("not independent evidence"));
    assert!(md.text.contains("```mermaid"));
    assert!(md.text.contains("Persist links before acknowledging"));
}
#[test]
fn generation_usage_links_once_and_does_not_charge_cache_reuse() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("test.sqlite");
    let c = db::open(&path).unwrap();
    let ctx = usage::Context {
        path,
        module: "p1".into(),
        session_id: None,
        operation: "generation".into(),
    };
    let after = usage::latest_id(&c).unwrap();
    usage::begin(&ctx, "claude", "model", false).unwrap();
    usage::begin(&ctx, "claude", "model", true).unwrap();
    usage::link_question(&c, "p1", "question", after).unwrap();
    usage::link_session(&c, "p1", "question", "first").unwrap();
    usage::link_session(&c, "p1", "question", "reused").unwrap();
    assert_eq!(
        usage::overview(&c, Some("p1"), Some("first")).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        usage::overview(&c, Some("p1"), Some("reused")).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn feedback_stops_clock_atomically_and_failed_finish_keeps_it_running() {
    let d = tempfile::tempdir().unwrap();
    let c = db::open(&d.path().join("test.sqlite")).unwrap();
    let id = p1_session(&c);
    tools::timing(&c, "p1", &id, "start", Some(20)).unwrap();
    assert!(p1::begin_feedback(&c, &id).is_err());
    assert!(tools::clock(&c, "p1", &id)
        .unwrap()
        .unwrap()
        .ended_at
        .is_none());
    tools::timing(&c, "p1", &id, "reset", None).unwrap();
    assert!(tools::clock(&c, "p1", &id).unwrap().is_none());
    tools::timing(&c, "p1", &id, "start", Some(30)).unwrap();
    c.execute("INSERT INTO interview_turns(session_id,role,text,turn_index) VALUES(?1,'candidate','Explicit local history fixture',0)",[&id]).unwrap();
    p1::begin_feedback(&c, &id).unwrap();
    assert_eq!(p1::session(&c, &id).unwrap().status, "feedback_pending");
    assert!(tools::clock(&c, "p1", &id)
        .unwrap()
        .unwrap()
        .ended_at
        .is_some());
    assert!(tools::timing(&c, "p1", &id, "reset", None).is_err());
}
