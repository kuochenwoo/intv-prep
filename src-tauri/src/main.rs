#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{path::PathBuf, sync::Mutex};
use swe_interview_prep::{code_runner, content, db};
use tauri::{Manager, State};
struct AppState {
    path: PathBuf,
    judge: Mutex<()>,
}
#[tauri::command]
fn list_problems() -> Vec<content::Problem> {
    content::problems()
}
#[tauri::command]
fn load_progress(state: State<AppState>) -> Result<Vec<db::Progress>, String> {
    db::load(&db::open(&state.path)?)
}
#[tauri::command]
fn save_draft(
    state: State<AppState>,
    problem_id: String,
    mode: String,
    code: String,
) -> Result<(), String> {
    db::save(&db::open(&state.path)?, &problem_id, &mode, &code)
}
#[tauri::command]
fn get_compiler(state: State<AppState>) -> Result<String, String> {
    db::compiler(&db::open(&state.path)?)
}
#[tauri::command(async)]
fn set_compiler(state: State<AppState>, path: String) -> Result<String, String> {
    let detected = code_runner::detect_compiler(&path)?;
    db::set_compiler(&db::open(&state.path)?, &detected)?;
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
    let c = db::open(&state.path)?;
    db::save(&c, &problem_id, &mode, &code)?;
    let result = code_runner::judge(
        &content::problem(&problem_id)?,
        &mode,
        &code,
        samples,
        &db::compiler(&c)?,
    )?;
    if !samples {
        db::record(
            &c,
            &problem_id,
            &mode,
            &serde_json::to_string(&result).map_err(|e| e.to_string())?,
            result.status == "accepted",
        )?;
    }
    Ok(result)
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_data_dir()?.join("prep.sqlite3");
            db::open(&path).map_err(std::io::Error::other)?;
            app.manage(AppState {
                path,
                judge: Mutex::new(()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_problems,
            load_progress,
            save_draft,
            get_compiler,
            set_compiler,
            run_judge
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 失败");
}
