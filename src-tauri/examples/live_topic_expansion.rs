//! Explicit live acceptance for expanded P1 topics. Reads existing provider keys only.
//! Uses scripted practice answers and writes an isolated report, not application history.
use serde_json::json;
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    interview::{self, Lang, LlmProvider, Turn},
    llm::InterviewProvider,
};

fn main() -> Result<(), String> {
    let provider = std::env::var("INTERVIEW_PROVIDER").unwrap_or_else(|_| "claude".into());
    let model = std::env::var("INTERVIEW_MODEL").unwrap_or_else(|_| {
        if provider == "deepseek" {
            "deepseek-flash"
        } else {
            "claude-haiku-5-5"
        }
        .into()
    });
    let config = LlmSettings {
        provider,
        model,
        timeout_seconds: 120,
    };
    let mut report = Vec::new();
    for (id, lang) in [
        ("mvcc-snapshots", Lang::Zh),
        ("javascript-event-loop", Lang::En),
    ] {
        let api =
            InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(lang);
        let question = api.generate_question(&interview::topic(id)?)?;
        let reference = api.reference_answer(&question, &question.question, &[])?;
        let turns = vec![Turn {
            role: "candidate".into(),
            text: reference.answer.clone(),
            ai_assisted: true,
            timestamp: String::new(),
        }];
        let follow_up = api.next_turn(&question, &turns)?;
        let feedback = api.summarize(&question, &turns)?;
        report.push(json!({"topicId":id,"lang":lang,"provider":config.provider,
            "model":config.model,"question":question,"reference":reference,
            "turns":turns,"followUp":follow_up,"feedback":feedback}));
        std::fs::write(
            "/tmp/intv-p1-expansion-live.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        println!("Completed real generation, reference, follow-up and feedback: {id} {lang:?}");
    }
    Ok(())
}
