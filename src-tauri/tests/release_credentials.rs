//! Exercise the actual non-debug storage selection without any real key or HTTP request.
#[cfg(not(debug_assertions))]
#[test]
fn release_credentials_require_initialization_and_survive_reopen_outside_checkout() {
    use swe_interview_prep::{credentials, db, transcription};
    assert!(credentials::env_path().is_err());
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("installed-application");
    credentials::initialize_storage(&config_dir).unwrap();
    credentials::initialize_storage(&config_dir).unwrap();
    assert!(credentials::initialize_storage(temp.path()).is_err());
    assert_eq!(credentials::env_path().unwrap(), config_dir.join(".env"));
    assert_eq!(credentials::key_status("claude"), "missing");
    for provider in ["claude", "deepseek", "openai"] {
        credentials::save_key(provider, &format!("fixture-installed-{provider}-key")).unwrap();
        assert_eq!(credentials::key_status(provider), "saved");
    }
    let database = temp.path().join("prep.sqlite3");
    let connection = db::open(&database).unwrap();
    let view = credentials::view(&connection).unwrap();
    assert_eq!(
        view.credential_path,
        config_dir.join(".env").to_string_lossy()
    );
    assert_eq!(view.key_status, "saved");
    assert_eq!(transcription::settings()["keyStatus"], "saved");
    drop(connection);
    let connection = db::open(&database).unwrap();
    assert_eq!(credentials::view(&connection).unwrap().key_status, "saved");
    credentials::delete_key("claude").unwrap();
    assert_eq!(credentials::key_status("claude"), "missing");
    assert_eq!(credentials::key_status("deepseek"), "saved");
    assert_eq!(credentials::key_status("openai"), "saved");
    let stored: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM settings WHERE value LIKE '%fixture-installed-%'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stored, 0);
}
