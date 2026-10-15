//! Explicit quality evaluation using scripted inputs and real provider requests.
//! The report records outputs for human semantic review, not automatic quality certification.
use serde_json::{json, Value};
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    interview::{self, Feedback, GeneratedQuestion, Lang, LlmProvider, Rating, Turn},
    llm::InterviewProvider,
    system_design::{self, DesignProvider, DesignSession, Workspace},
};

fn save(report: &[Value]) -> Result<(), String> {
    std::fs::write(
        "/tmp/intv-ai-quality-live.json",
        serde_json::to_string_pretty(report).unwrap(),
    )
    .map_err(|e| e.to_string())
}
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
    let cases: Value =
        serde_json::from_str(include_str!("../../content/evals/interview-quality.json")).unwrap();
    let mut report = vec![json!({"provider":config.provider,"model":config.model,
        "description":cases["description"],"logicalRequestsPlanned":16})];
    for case in cases["generations"].as_array().unwrap() {
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api =
            InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(lang);
        let question =
            api.generate_question(&interview::topic(case["seedId"].as_str().unwrap())?)?;
        report.push(json!({"kind":"generation","input":case,"output":question,
            "review":"Check visible fields for answer leakage and private checkpoints for unasked scope."}));
        save(&report)?;
        println!(
            "Completed real generation: {} {}",
            case["seedId"], case["lang"]
        );
    }
    for case in cases["p1Cases"].as_array().unwrap() {
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api =
            InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(lang);
        let question: GeneratedQuestion = serde_json::from_value(case["question"].clone()).unwrap();
        let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
        let follow = if case["feedbackOnly"] == true {
            None
        } else {
            Some(api.next_turn(&question, &turns)?)
        };
        let feedback: Feedback = api.summarize(&question, &turns)?;
        // This assertion verifies the application's provenance guard, not model semantic quality.
        if turns.iter().all(|t| t.role != "candidate" || t.ai_assisted)
            && !feedback
                .dimensions
                .iter()
                .all(|d| matches!(d.rating, Rating::Missing))
        {
            return Err("AI-only provenance guard failed".into());
        }
        report.push(json!({"kind":"p1","input":case,"followUp":follow,"feedback":feedback}));
        save(&report)?;
        println!("Completed real P1 case: {}", case["id"]);
    }
    for case in cases["p2Cases"].as_array().unwrap() {
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api =
            InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?.lang(lang);
        let session = DesignSession {
            id: "scripted-quality-probe".into(),
            created_at: String::new(),
            question: system_design::CachedQuestion {
                id: "scripted-question".into(),
                seed_id: case["seedId"].as_str().unwrap().into(),
                lang,
                provider: config.provider.clone(),
                model: config.model.clone(),
                created_at: String::new(),
                content: serde_json::from_value(case["question"].clone()).unwrap(),
            },
            provider: config.provider.clone(),
            model: config.model.clone(),
            stage: serde_json::from_value(case["stage"].clone()).unwrap(),
            status: "active".into(),
            workspace: Workspace {
                drafts: Default::default(),
                notes: case["notes"].as_str().unwrap().into(),
                diagram: case["diagram"].as_str().unwrap().into(),
            },
            turns: serde_json::from_value(case["turns"].clone()).unwrap(),
            suggested_stage: None,
            last_error: None,
            feedback: None,
            references: vec![],
        };
        let follow = api.design_follow_up(&session)?;
        let feedback = api.design_feedback(&session)?;
        report.push(json!({"kind":"p2","input":case,"followUp":follow,"feedback":feedback}));
        save(&report)?;
        println!("Completed real P2 case: {}", case["id"]);
    }
    println!("Completed 16 logical real requests; semantic review remains manual. Report /tmp/intv-ai-quality-live.json");
    Ok(())
}
