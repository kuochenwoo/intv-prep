#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use interview::LlmProvider;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};
use swe_interview_prep::{code_runner, content, credentials, db, interview, interview_store, llm};
use swe_interview_prep::{system_design, system_design_store};
use tauri::{AppHandle, Emitter, Manager, State};
#[derive(Clone)]
struct AppState {
    path: PathBuf,
    // One migrated connection shared by short commands. Long-running AI tasks open
    // their own cheap connection so a slow API call never blocks drafts or settings.
    db: Arc<Mutex<rusqlite::Connection>>,
    judge: Arc<Mutex<()>>,
    interview: Arc<Mutex<()>>,
    transcription: Arc<Mutex<()>>,
}
impl AppState {
    fn db(&self) -> MutexGuard<'_, rusqlite::Connection> {
        // A panic mid-command does not corrupt SQLite; keep using the connection.
        self.db.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn own_connection(&self) -> Result<rusqlite::Connection, String> {
        db::connect(&self.path)
    }
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AppUpdateInfo {
    version: String,
    enabled: bool,
    auto_check_allowed: bool,
}

fn update_install_supported() -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64", not(debug_assertions)))
        && std::env::current_exe().is_ok_and(|path| {
            !path.starts_with("/Volumes")
                && path.ancestors().any(|parent| parent.extension().is_some_and(|ext| ext == "app"))
        })
}

#[tauri::command]
fn app_update_info(app: AppHandle) -> AppUpdateInfo {
    let enabled = update_install_supported();
    AppUpdateInfo {
        version: app.package_info().version.to_string(),
        enabled,
        // Local QA profiles must not contact the production feed automatically.
        auto_check_allowed: enabled && app.config().identifier == "local.swe.interview-prep",
    }
}

#[tauri::command(async)]
fn prepare_app_update(state: State<AppState>) -> Result<(), String> {
    if !update_install_supported() {
        return Err("当前运行环境不支持应用内更新，请安装正式应用包".into());
    }
    // The frontend blocks new actions during the download/install transaction.
    // Refuse to interrupt an existing judge, model request or transcription.
    let busy = "请等待判题、面试请求或语音转写完成后再更新";
    let _judge = state.judge.try_lock().map_err(|_| busy)?;
    let _interview = state.interview.try_lock().map_err(|_| busy)?;
    let _transcription = state.transcription.try_lock().map_err(|_| busy)?;
    state.db().execute_batch("PRAGMA wal_checkpoint(FULL)")
        .map_err(|_| "无法保存更新前的数据，请稍后重试".to_string())?;
    Ok(())
}

#[tauri::command]
fn list_problems() -> &'static [content::Problem] {
    content::problems()
}
#[tauri::command(async)]
fn open_problem_source(problem_id: String) -> Result<(), String> {
    // Only open bundled, known problem URLs; never run a user-provided shell command.
    let url = &content::find(&problem_id)?.complexity.source_url;
    let launcher = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "linux") {
        "xdg-open"
    } else {
        return Err("此平台尚未支持打开官方链接".into());
    };
    let status = std::process::Command::new(launcher)
        .arg(url)
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("无法打开官方题面".into())
    }
}
#[tauri::command]
fn load_progress(state: State<AppState>) -> Result<Vec<db::Progress>, String> {
    db::load(&state.db())
}
#[tauri::command]
fn save_draft(
    state: State<AppState>,
    problem_id: String,
    mode: String,
    code: String,
) -> Result<(), String> {
    db::save(&state.db(), &problem_id, &mode, &code)
}
#[tauri::command]
fn clear_solutions(state: State<AppState>, problem_id: Option<String>) -> Result<(), String> {
    // Never clear while a judge run is about to record its result.
    let _guard = state.judge.try_lock().map_err(|_| "已有判题任务正在运行")?;
    db::clear(&state.db(), problem_id.as_deref())
}
#[tauri::command]
fn get_compiler(state: State<AppState>) -> Result<String, String> {
    db::compiler(&state.db())
}
#[tauri::command(async)]
fn set_compiler(state: State<AppState>, path: String) -> Result<String, String> {
    let detected = code_runner::detect_compiler(&path)?;
    db::set_compiler(&state.db(), &detected)?;
    Ok(detected)
}
#[tauri::command(async)]
fn run_judge(
    state: State<AppState>,
    problem_id: String,
    mode: String,
    code: String,
    samples: bool,
) -> Result<code_runner::JudgeResult, String> {
    let _guard = state.judge.try_lock().map_err(|_| "已有判题任务正在运行")?;
    // Hold the shared connection only around the writes, not while compiling and running.
    let compiler = {
        let c = state.db();
        db::save(&c, &problem_id, &mode, &code)?;
        db::compiler(&c)?
    };
    let result = code_runner::judge(
        &content::problem(&problem_id)?,
        &mode,
        &code,
        samples,
        &compiler,
    )?;
    if !samples {
        let c = state.db();
        db::record_submission(&c, &problem_id, &mode, &result)?;
    }
    Ok(result)
}
// The provider owns a runtime; create and use it on blocking workers, outside Tauri async tasks.
async fn background<T: Send + 'static>(
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|_| "后台面试任务意外终止，请重试")?
}
fn provider(
    app: &AppHandle,
    c: &rusqlite::Connection,
    session_id: Option<&str>,
    lang: interview::Lang,
    path: PathBuf,
    operation: &str,
) -> Result<llm::InterviewProvider, String> {
    let mut config = credentials::settings(c)?;
    if let Some(id) = session_id {
        let s = interview_store::session(c, id)?;
        config.provider = s.provider;
        config.model = s.model;
    }
    let app = app.clone();
    let last = Mutex::new(std::time::Instant::now());
    let show_stream = operation != "generation";
    // Only one interview request runs at a time, so one event name is enough.
    // Throttled: the UI only needs a few repaints per second of the growing text.
    Ok(
        llm::InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?
            .lang(lang)
            .record_usage(swe_interview_prep::usage::Context {
                path,
                module: "p1".into(),
                session_id: session_id.map(str::to_owned),
                operation: operation.into(),
            })
            .on_text(move |text| {
                // Do not expose opening-question drafts before validation/review accepts them.
                if !show_stream {
                    return;
                }
                let mut last = last.lock().unwrap_or_else(|e| e.into_inner());
                if text.is_empty() || last.elapsed() >= std::time::Duration::from_millis(80) {
                    *last = std::time::Instant::now();
                    let _ = app.emit("llm-stream", text);
                }
            }),
    )
}
#[tauri::command(async)]
fn interview_overview(state: State<AppState>) -> Result<serde_json::Value, String> {
    let c = state.db();
    Ok(
        serde_json::json!({"topics":interview::topics(),"questions":interview_store::questions(&c)?,"sessions":interview_store::sessions(&c)?,"settings":credentials::view(&c)?}),
    )
}
#[tauri::command(async)]
fn save_llm_settings(
    state: State<AppState>,
    config: credentials::LlmSettings,
    api_key: Option<String>,
) -> Result<credentials::SettingsView, String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    config.validate()?;
    if let Some(key) = api_key.filter(|s| !s.trim().is_empty()) {
        credentials::save_key(&config.provider, &key)?;
    }
    let c = state.db();
    credentials::save_settings(&c, &config)?;
    credentials::view(&c)
}
#[tauri::command(async)]
fn delete_llm_key(state: State<AppState>, provider: String) -> Result<(), String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    credentials::delete_key(&provider)
}
#[tauri::command(async)]
async fn generate_interview_question(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    topic_id: String,
) -> Result<interview::QuestionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let config = credentials::settings(&c)?;
        let after = swe_interview_prep::usage::latest_id(&c)?;
        let q = provider(
            &app,
            &c,
            None,
            lang.unwrap_or_default(),
            state.path.clone(),
            "generation",
        )?
        .generate_question(&interview::topic(&topic_id)?)?;
        let cached = interview_store::cache_question_lang(
            &c,
            &topic_id,
            &config,
            &q,
            lang.unwrap_or_default(),
        )?;
        swe_interview_prep::usage::link_question(&c, "p1", &cached.id, after)?;
        Ok(cached)
    })
    .await
}
#[tauri::command]
fn start_interview(
    state: State<AppState>,
    question_id: String,
) -> Result<interview::SessionView, String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    let c = state.db();
    let session = interview_store::start(&c, &question_id, &credentials::settings(&c)?)?;
    swe_interview_prep::usage::link_session(&c, "p1", &question_id, &session.id)?;
    Ok(session)
}
#[tauri::command]
fn get_interview_session(
    state: State<AppState>,
    session_id: String,
) -> Result<interview::SessionView, String> {
    interview_store::session(&state.db(), &session_id)
}
#[tauri::command]
fn save_interview_draft(
    state: State<AppState>,
    session_id: String,
    draft: String,
) -> Result<(), String> {
    interview_store::save_draft(&state.db(), &session_id, &draft)
}
#[tauri::command(async)]
async fn submit_interview_answer(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    session_id: String,
    answer: String,
) -> Result<interview::SessionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let api = provider(
            &app,
            &c,
            Some(&session_id),
            lang.unwrap_or_default(),
            state.path.clone(),
            "follow_up",
        )?;
        interview_store::begin_answer(&c, &session_id, &answer)?;
        interview_store::retry_pending(&c, &session_id, &api)
    })
    .await
}
#[tauri::command(async)]
async fn finish_interview(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    session_id: String,
) -> Result<interview::SessionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let api = provider(
            &app,
            &c,
            Some(&session_id),
            lang.unwrap_or_default(),
            state.path.clone(),
            "feedback",
        )?;
        interview_store::begin_feedback(&c, &session_id)?;
        interview_store::retry_pending(&c, &session_id, &api)
    })
    .await
}
#[tauri::command(async)]
async fn retry_interview(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    session_id: String,
) -> Result<interview::SessionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        interview_store::retry_pending(
            &c,
            &session_id,
            &provider(
                &app,
                &c,
                Some(&session_id),
                lang.unwrap_or_default(),
                state.path.clone(),
                "retry",
            )?,
        )
    })
    .await
}
#[tauri::command(async)]
async fn submit_interview_reference(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    session_id: String,
    turn_index: usize,
) -> Result<interview::SessionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        interview_store::submit_reference(
            &c,
            &session_id,
            turn_index,
            &provider(
                &app,
                &c,
                Some(&session_id),
                lang.unwrap_or_default(),
                state.path.clone(),
                "reference_submit",
            )?,
        )
    })
    .await
}
// One AI reference answer for the current question, shown beside the session.
// Unlike submit_interview_reference it is not submitted and asks for no follow-up.
#[tauri::command(async)]
async fn show_interview_reference(
    app: AppHandle,
    lang: Option<interview::Lang>,
    state: State<'_, AppState>,
    session_id: String,
    turn_index: usize,
) -> Result<interview::SessionView, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        interview_store::generate_reference(
            &c,
            &session_id,
            turn_index,
            &provider(
                &app,
                &c,
                Some(&session_id),
                lang.unwrap_or_default(),
                state.path.clone(),
                "reference",
            )?,
        )?;
        interview_store::session(&c, &session_id)
    })
    .await
}
#[tauri::command]
fn delete_interview_session(state: State<AppState>, session_id: String) -> Result<(), String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    interview_store::delete_session(&state.db(), &session_id)
}
#[tauri::command]
fn clear_interview_history(state: State<AppState>) -> Result<(), String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    interview_store::clear_history(&state.db())
}
#[tauri::command]
fn sd_delete_session(state: State<AppState>, session_id: String) -> Result<(), String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    system_design_store::delete_session(&state.db(), &session_id)
}
#[tauri::command]
fn sd_clear_history(state: State<AppState>) -> Result<(), String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    system_design_store::clear_history(&state.db())
}
#[tauri::command]
fn sd_overview(state: State<AppState>) -> Result<serde_json::Value, String> {
    let c = state.db();
    Ok(
        serde_json::json!({"scenarios":system_design::scenarios(),"questions":system_design_store::questions(&c)?,"sessions":system_design_store::sessions(&c)?,"settings":credentials::view(&c)?}),
    )
}
#[tauri::command]
fn sd_start(
    state: State<AppState>,
    question_id: String,
) -> Result<system_design::DesignSession, String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    let c = state.db();
    let session = system_design_store::start(&c, &question_id, &credentials::settings(&c)?)?;
    swe_interview_prep::usage::link_session(&c, "p2", &question_id, &session.id)?;
    Ok(session)
}
#[tauri::command]
fn sd_session(
    state: State<AppState>,
    session_id: String,
) -> Result<system_design::DesignSession, String> {
    system_design_store::session(&state.db(), &session_id)
}
#[tauri::command]
fn sd_save_workspace(
    state: State<AppState>,
    session_id: String,
    workspace: system_design::Workspace,
) -> Result<(), String> {
    system_design_store::save_workspace(&state.db(), &session_id, workspace)
}
#[tauri::command]
fn sd_change_stage(
    state: State<AppState>,
    session_id: String,
    stage: system_design::Stage,
) -> Result<system_design::DesignSession, String> {
    let _guard = state
        .interview
        .try_lock()
        .map_err(|_| "请等待当前面试请求完成")?;
    system_design_store::change_stage(&state.db(), &session_id, stage)
}
fn sd_provider(
    app: &AppHandle,
    config: &credentials::LlmSettings,
    lang: interview::Lang,
    path: PathBuf,
    session_id: Option<&str>,
    operation: &str,
) -> Result<llm::InterviewProvider, String> {
    let app = app.clone();
    let last = Mutex::new(std::time::Instant::now());
    let show_stream = operation != "generation";
    Ok(
        llm::InterviewProvider::new(config, credentials::read_key(&config.provider)?)?
            .lang(lang)
            .record_usage(swe_interview_prep::usage::Context {
                path,
                module: "p2".into(),
                session_id: session_id.map(str::to_owned),
                operation: operation.into(),
            })
            .on_text(move |text| {
                // Do not expose opening-question drafts before validation/review accepts them.
                if !show_stream {
                    return;
                }
                let mut last = last.lock().unwrap_or_else(|e| e.into_inner());
                if text.is_empty() || last.elapsed() >= std::time::Duration::from_millis(80) {
                    *last = std::time::Instant::now();
                    let _ = app.emit("sd-stream", text);
                }
            }),
    )
}
#[tauri::command(async)]
async fn sd_generate(
    app: AppHandle,
    state: State<'_, AppState>,
    scenario_id: String,
    lang: interview::Lang,
) -> Result<system_design::CachedQuestion, String> {
    let state = state.inner().clone();
    background(move || {
        use system_design::DesignProvider;
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let config = credentials::settings(&c)?;
        let after = swe_interview_prep::usage::latest_id(&c)?;
        let q = sd_provider(&app, &config, lang, state.path.clone(), None, "generation")?
            .generate_design(&system_design::scenario(&scenario_id)?, lang)?;
        let cached = system_design_store::cache_question(&c, &scenario_id, lang, &config, q)?;
        swe_interview_prep::usage::link_question(&c, "p2", &cached.id, after)?;
        Ok(cached)
    })
    .await
}
#[tauri::command(async)]
async fn sd_request(
    app: AppHandle,
    state: State<'_, AppState>,
    session_id: String,
    action: String,
    answer: Option<String>,
) -> Result<system_design::DesignSession, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let s = system_design_store::session(&c, &session_id)?;
        let mut config = credentials::settings(&c)?;
        config.provider = s.provider;
        config.model = s.model;
        let api = sd_provider(
            &app,
            &config,
            s.question.lang,
            state.path.clone(),
            Some(&session_id),
            &action,
        )?;
        match action.as_str() {
            "answer" => system_design_store::begin_answer(
                &c,
                &session_id,
                answer.as_deref().unwrap_or_default(),
            )?,
            "feedback" => system_design_store::begin_feedback(&c, &session_id)?,
            "retry" => {}
            _ => return Err("未知系统设计操作".into()),
        }
        system_design_store::retry_pending(&c, &session_id, &api)
    })
    .await
}
#[tauri::command]
async fn sd_reference(
    app: AppHandle,
    state: State<'_, AppState>,
    session_id: String,
    submit_turn: Option<usize>,
) -> Result<system_design::DesignSession, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .interview
            .try_lock()
            .map_err(|_| "已有面试请求正在运行")?;
        let c = state.own_connection()?;
        let s = system_design_store::session(&c, &session_id)?;
        let mut config = credentials::settings(&c)?;
        config.provider = s.provider;
        config.model = s.model;
        let api = sd_provider(
            &app,
            &config,
            s.question.lang,
            state.path.clone(),
            Some(&session_id),
            if submit_turn.is_some() {
                "reference_submit"
            } else {
                "reference"
            },
        )?;
        match submit_turn {
            Some(turn) => system_design_store::submit_reference(&c, &session_id, turn, &api),
            None => system_design_store::generate_reference(&c, &session_id, &api),
        }
    })
    .await
}

#[tauri::command]
fn transcription_settings() -> serde_json::Value {
    swe_interview_prep::transcription::settings()
}
#[tauri::command]
fn save_transcription_key(api_key: String) -> Result<(), String> {
    credentials::save_key("openai", &api_key)
}
#[tauri::command]
async fn transcribe_audio(
    state: State<'_, AppState>,
    module: String,
    session_id: String,
    audio: Vec<u8>,
    mime: String,
    lang: interview::Lang,
) -> Result<swe_interview_prep::transcription::Transcript, String> {
    let state = state.inner().clone();
    background(move || {
        let _guard = state
            .transcription
            .try_lock()
            .map_err(|_| "已有语音转写请求正在运行")?;
        swe_interview_prep::transcription::transcribe(
            &state.path,
            &module,
            &session_id,
            &audio,
            &mime,
            lang,
        )
    })
    .await
}

#[tauri::command]
fn coding_timing(
    state: State<AppState>,
    problem_id: String,
    action: String,
    minutes: Option<i64>,
) -> Result<Option<swe_interview_prep::session_tools::MockClock>, String> {
    swe_interview_prep::coding_timer::timing(
        &state.db(),
        &problem_id,
        &action,
        minutes,
        swe_interview_prep::session_tools::now_ms(),
    )
}

#[tauri::command]
fn session_timing(
    state: State<AppState>,
    module: String,
    session_id: String,
    action: String,
    minutes: Option<i64>,
) -> Result<Option<swe_interview_prep::session_tools::MockClock>, String> {
    swe_interview_prep::session_tools::timing(&state.db(), &module, &session_id, &action, minutes)
}
#[tauri::command]
fn export_interview(
    state: State<AppState>,
    module: String,
    session_id: String,
    format: String,
    lang: String,
) -> Result<swe_interview_prep::session_tools::Export, String> {
    swe_interview_prep::session_tools::export(&state.db(), &module, &session_id, &format, &lang)
}
#[tauri::command]
fn api_usage(
    state: State<AppState>,
    module: Option<String>,
    session_id: Option<String>,
) -> Result<serde_json::Value, String> {
    swe_interview_prep::usage::overview(&state.db(), module.as_deref(), session_id.as_deref())
}
#[tauri::command]
fn review_overview(state: State<AppState>) -> Result<swe_interview_prep::review::Overview, String> {
    let now = swe_interview_prep::session_tools::now_ms();
    swe_interview_prep::review::overview(&state.db(), now)
}
#[tauri::command]
fn review_dismiss(state: State<AppState>, concept: String) -> Result<(), String> {
    swe_interview_prep::review::dismiss(&state.db(), &concept)
}
#[tauri::command]
fn save_api_prices(
    state: State<AppState>,
    provider: String,
    model: String,
    rates: swe_interview_prep::usage::Rates,
) -> Result<(), String> {
    swe_interview_prep::usage::save_rates(&state.db(), &provider, &model, &rates)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            credentials::initialize_storage(&app.path().app_config_dir()?)
                .map_err(std::io::Error::other)?;
            let path = app.path().app_data_dir()?.join("prep.sqlite3");
            let connection = db::open(&path).map_err(std::io::Error::other)?;
            app.manage(AppState {
                path,
                db: Arc::new(Mutex::new(connection)),
                judge: Arc::new(Mutex::new(())),
                interview: Arc::new(Mutex::new(())),
                transcription: Arc::new(Mutex::new(())),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_update_info,
            prepare_app_update,
            review_overview,
            review_dismiss,
            session_timing,
            coding_timing,
            transcription_settings,
            save_transcription_key,
            transcribe_audio,
            export_interview,
            api_usage,
            save_api_prices,
            sd_overview,
            sd_start,
            sd_session,
            sd_save_workspace,
            sd_change_stage,
            sd_generate,
            sd_request,
            sd_reference,
            sd_delete_session,
            sd_clear_history,
            interview_overview,
            save_llm_settings,
            delete_llm_key,
            generate_interview_question,
            start_interview,
            get_interview_session,
            save_interview_draft,
            submit_interview_answer,
            finish_interview,
            retry_interview,
            submit_interview_reference,
            show_interview_reference,
            delete_interview_session,
            clear_interview_history,
            list_problems,
            open_problem_source,
            load_progress,
            save_draft,
            clear_solutions,
            get_compiler,
            set_compiler,
            run_judge
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 失败");
}
