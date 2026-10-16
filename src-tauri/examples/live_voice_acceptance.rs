//! Explicit live acceptance: synthetic audio only, OpenAI STT and DeepSeek text only.
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    db,
    interview::{self, GeneratedQuestion, Lang, LlmProvider},
    interview_store as p1, llm,
    system_design::{DesignQuestion, Stage},
    system_design_store as p2, transcription, usage,
};

fn api(
    path: &Path,
    module: &str,
    session: Option<&str>,
    operation: &str,
    lang: Lang,
    cap: Arc<AtomicUsize>,
) -> Result<llm::InterviewProvider, String> {
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 90,
    };
    Ok(
        llm::InterviewProvider::new(&config, credentials::read_key("deepseek")?)?
            .lang(lang)
            .record_usage(usage::Context {
                path: path.into(),
                module: module.into(),
                session_id: session.map(str::to_owned),
                operation: operation.into(),
            })
            .before_request(move || loop {
                let n = cap.load(Ordering::SeqCst);
                if n >= 8 {
                    return Err("DeepSeek acceptance cap reached (8 HTTP attempts)".into());
                }
                if cap
                    .compare_exchange(n, n + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    return Ok(());
                }
            }),
    )
}
fn mixed_voice(audio: &Path, path: &Path) -> Result<(), String> {
    let c = db::open(path)?;
    let before = usage::overview(&c, None, None)?;
    let attempts = before["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["provider"] == "openai")
        .count();
    if attempts >= 4 {
        return Err("OpenAI voice cap reached".into());
    }
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 90,
    };
    let question = p1::questions(&c)?
        .into_iter()
        .next()
        .ok_or("No authored test question")?;
    let session = p1::start(&c, &question.id, &config)?;
    let result = transcription::transcribe(
        path,
        "p1",
        &session.id,
        &std::fs::read(audio).map_err(|e| e.to_string())?,
        "audio/mp4",
        Lang::Zh,
    );
    let mut report: Value = serde_json::from_slice(
        &std::fs::read("review/voice-live-acceptance.json").map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    report["extraVoiceChecks"] = json!([{"case":"mixed_chinese_english_voice","passed":result.is_ok(),"transcript":result.as_ref().ok().map(|v|&v.text),"error":result.as_ref().err()}]);
    report["usage"] = usage::overview(&c, None, None)?;
    report["openaiHttpAttempts"] = json!(attempts + 1);
    std::fs::write(
        "review/voice-live-acceptance.json",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "Mixed-language OpenAI voice check: {}",
        if result.is_ok() { "passed" } else { "failed" }
    );
    result.map(|_| ())
}
fn main() -> Result<(), String> {
    if std::env::args().nth(1).as_deref() == Some("--mixed") {
        return mixed_voice(
            Path::new(&std::env::args().nth(2).ok_or("Audio path required")?),
            Path::new(
                &std::env::args()
                    .nth(3)
                    .ok_or("Acceptance database required")?,
            ),
        );
    }
    let audio_dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("Synthetic audio directory required")?,
    );
    let path = PathBuf::from(
        std::env::args()
            .nth(2)
            .ok_or("Fresh acceptance SQLite path required")?,
    );
    if path.exists() {
        return Err("Use a fresh isolated acceptance database".into());
    }
    let c = db::open(&path)?;
    let cap = Arc::new(AtomicUsize::new(0));
    let config = LlmSettings {
        provider: "deepseek".into(),
        model: "deepseek-flash".into(),
        timeout_seconds: 90,
    };
    let mut results: Vec<Value> = vec![];
    let generated = api(&path, "p1", None, "generation", Lang::En, cap.clone())?
        .generate_question(&interview::topic("javascript-event-loop")?);
    results.push(json!({"case":"p1_deepseek_generation","passed":generated.is_ok(),"error":generated.as_ref().err()}));
    // A failure uses an explicitly authored fixture so voice validation can still proceed.
    let q = generated.unwrap_or(GeneratedQuestion {
        question:
            "Explain the ordering of synchronous code, Promise callbacks and timers in a browser."
                .into(),
        background: "Authored fallback, not successful model generation.".into(),
        constraints: vec!["One browser window".into()],
        follow_up_directions: vec!["Trace the queues".into()],
        evaluation_checklist: vec!["Explain the microtask checkpoint".into()],
        answer_terms: vec![],
    });
    let q = p1::cache_question(&c, "javascript-event-loop", &config, &q)?;
    let session = p1::start(&c, &q.id, &config)?;
    let p1_session_id = session.id.clone();
    let voice = transcription::transcribe(
        &path,
        "p1",
        &session.id,
        &std::fs::read(audio_dir.join("en.m4a")).map_err(|e| e.to_string())?,
        "audio/mp4",
        Lang::En,
    );
    results.push(json!({"case":"openai_english_voice","passed":voice.is_ok(),"transcript":voice.as_ref().ok().map(|v|&v.text),"error":voice.as_ref().err()}));
    if let Ok(voice) = voice {
        let draft = format!("Typed draft retained.\n\n{}", voice.text);
        p1::save_draft(&c, &session.id, &draft)?;
        results.push(json!({"case":"p1_draft_not_submitted_by_transcription","passed":p1::session(&c,&session.id)?.turns.is_empty() && p1::session(&c,&session.id)?.draft==draft}));
        p1::begin_answer(&c, &session.id, &draft)?;
        let follow = p1::retry_pending(
            &c,
            &session.id,
            &api(
                &path,
                "p1",
                Some(&session.id),
                "follow_up",
                Lang::En,
                cap.clone(),
            )?,
        );
        results.push(json!({"case":"p1_deepseek_follow_up","passed":follow.is_ok(),"error":follow.as_ref().err()}));
        if follow.is_ok() {
            p1::begin_feedback(&c, &session.id)?;
            let feedback = p1::retry_pending(
                &c,
                &session.id,
                &api(
                    &path,
                    "p1",
                    Some(&session.id),
                    "feedback",
                    Lang::En,
                    cap.clone(),
                )?,
            );
            results.push(json!({"case":"p1_deepseek_feedback","passed":feedback.as_ref().is_ok_and(|s|s.feedback.is_some()),"error":feedback.as_ref().err()}));
        }
    }
    let q = p2::cache_question(
        &c,
        "file-sync",
        Lang::Zh,
        &config,
        DesignQuestion {
            question:
                "设计一个支持离线修改和冲突处理的文件同步服务。请解释文件版本如何保存和合并。"
                    .into(),
            background: "Authored acceptance scenario; not AI-generated content.".into(),
            constraints: vec!["多端可能同时离线修改同一文件".into()],
            follow_up_directions: vec!["分析版本冲突".into()],
            evaluation_checklist: vec!["避免静默覆盖".into()],
        },
    )?;
    let session = p2::start(&c, &q.id, &config)?;
    let session = p2::change_stage(&c, &session.id, Stage::HighLevel)?;
    let voice = transcription::transcribe(
        &path,
        "p2",
        &session.id,
        &std::fs::read(audio_dir.join("zh.m4a")).map_err(|e| e.to_string())?,
        "audio/mp4",
        Lang::Zh,
    );
    results.push(json!({"case":"openai_chinese_voice","passed":voice.is_ok(),"transcript":voice.as_ref().ok().map(|v|&v.text),"error":voice.as_ref().err()}));
    if let Ok(voice) = voice {
        let draft = format!("保留原有草稿。\n\n{}", voice.text);
        let mut workspace = session.workspace.clone();
        workspace.drafts.insert(Stage::HighLevel, draft.clone());
        p2::save_workspace(&c, &session.id, workspace)?;
        results.push(json!({"case":"p2_draft_not_submitted_by_transcription","passed":p2::session(&c,&session.id)?.turns.iter().all(|t|t.role!="candidate") && p2::session(&c,&session.id)?.workspace.drafts.get(&Stage::HighLevel)==Some(&draft)}));
        p2::begin_answer(&c, &session.id, &draft)?;
        let follow = p2::retry_pending(
            &c,
            &session.id,
            &api(
                &path,
                "p2",
                Some(&session.id),
                "follow_up",
                Lang::Zh,
                cap.clone(),
            )?,
        );
        results.push(json!({"case":"p2_deepseek_follow_up","passed":follow.is_ok(),"error":follow.as_ref().err()}));
        if follow.is_ok() {
            p2::begin_feedback(&c, &session.id)?;
            let feedback = p2::retry_pending(
                &c,
                &session.id,
                &api(
                    &path,
                    "p2",
                    Some(&session.id),
                    "feedback",
                    Lang::Zh,
                    cap.clone(),
                )?,
            );
            results.push(json!({"case":"p2_deepseek_feedback","passed":feedback.as_ref().is_ok_and(|s|s.feedback.is_some()),"error":feedback.as_ref().err()}));
        }
    }
    let accounting = usage::overview(&c, None, None)?;
    let rows = accounting["records"].as_array().unwrap();
    let voice_count = rows.iter().filter(|r| r["provider"] == "openai").count();
    let isolation = rows.iter().all(|r| {
        if r["provider"] == "openai" {
            r["model"] == transcription::MODEL && r["operation"] == "transcription"
        } else {
            r["provider"] == "deepseek"
        }
    });
    results.push(json!({"case":"provider_key_isolation","passed":isolation && voice_count<=4 && cap.load(Ordering::SeqCst)<=8}));
    let passed = results.iter().all(|r| r["passed"] == true) && results.len() == 10;
    let report = json!({"status":if passed{"passed"}else{"failed_or_incomplete"},"audio":"Locally synthesized authored samples; no personal audio","openaiHttpAttempts":voice_count,"deepseekHttpAttempts":cap.load(Ordering::SeqCst),"results":results,"usage":accounting,"priceNote":"Costs are unknown without configured transcription audio/text rates; not invoices","p1Session":p1::session(&c,&p1_session_id).ok(),"p2Session":p2::session(&c,&session.id).ok()});
    std::fs::write(
        "review/voice-live-acceptance.json",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!("Voice/text acceptance: {}; OpenAI {} attempts, DeepSeek {} attempts. Report: review/voice-live-acceptance.json",report["status"],voice_count,cap.load(Ordering::SeqCst));
    if passed {
        Ok(())
    } else {
        Err("Acceptance contains failures or incomplete steps; see report".into())
    }
}
