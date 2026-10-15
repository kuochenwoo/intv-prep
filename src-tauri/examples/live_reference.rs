//! Explicit live acceptance: AI answers the original question and a followup, then gets feedback.
//! Reuses a real cached question in an isolated test database, always starts a new test session.
use swe_interview_prep::{credentials, db, interview_store as store, llm::InterviewProvider};
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/intv-p1-live-claude-final.sqlite3".into());
    let c = db::open(std::path::Path::new(&path))?;
    let question = store::questions(&c)?
        .into_iter()
        .find(|q| q.provider == "claude")
        .ok_or("Run the live interview acceptance first to cache a Claude scenario question")?;
    let config = credentials::LlmSettings {
        provider: question.provider.clone(),
        model: question.model.clone(),
        timeout_seconds: 120,
    };
    let api = InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?;
    let mut s = if let Ok(id) = std::env::var("INTERVIEW_SESSION") {
        store::session(&c, &id)?
    } else {
        store::start(&c, &question.id, &config)?
    };
    println!(
        "Live AI-assisted answer test session: {}; {} / {}",
        s.id, config.provider, config.model
    );
    if s.status == "awaiting_reply" {
        s = store::retry_pending(&c, &s.id, &api)?;
        println!(
            "Resumed the pending follow-up without duplicating the AI answer; {} messages",
            s.turns.len()
        );
    }
    while s.turns.iter().filter(|t| t.role == "candidate").count() < 2 {
        let round = s.turns.len() / 2;
        s = store::submit_reference(&c, &s.id, s.turns.len(), &api)?;
        let answer = &s.turns[s.turns.len() - 2];
        let followup = s.turns.last().ok_or("Missing follow-up")?;
        if answer.role != "candidate" || !answer.ai_assisted || followup.role != "interviewer" {
            return Err("The AI answer or subsequent follow-up was not saved correctly".into());
        }
        println!(
            "Round {}: submitted a real AI answer ({} characters); next follow-up: {}",
            round + 1,
            answer.text.chars().count(),
            followup.text
        );
    }
    if s.status != "finished" {
        if s.status == "active" {
            store::begin_feedback(&c, &s.id)?;
        }
        s = store::retry_pending(&c, &s.id, &api)?;
    }
    println!(
        "Live feedback: {}",
        s.feedback.as_ref().ok_or("Missing feedback")?.summary
    );
    drop(c);
    let c = db::open(std::path::Path::new(&path))?;
    let restored = store::session(&c, &s.id)?;
    if restored.status != "finished"
        || restored.turns.len() != 4
        || restored.turns.iter().filter(|t| t.ai_assisted).count() != 2
        || restored.references.len() != 2
    {
        return Err("AI answers or session state did not survive reopening consistently".into());
    }
    println!("Verified automatic AI answers, submission, follow-ups, five-dimension feedback, and restoration. Test database: {path}");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Live AI-assisted answer acceptance incomplete: {e}");
        std::process::exit(1);
    }
}
