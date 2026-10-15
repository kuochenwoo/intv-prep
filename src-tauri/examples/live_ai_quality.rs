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
        if std::env::var_os("INTERVIEW_QUALITY_FINAL").is_some() {
            "review/calibration-closeout-live.json"
        } else if std::env::var_os("INTERVIEW_QUALITY_FOCUS").is_some() {
            "/tmp/intv-ai-quality-focused-live.json"
        } else {
            "/tmp/intv-ai-quality-live.json"
        },
        serde_json::to_string_pretty(report).unwrap(),
    )
    .map_err(|e| e.to_string())
}
fn save_recheck(report: &[Value]) -> Result<(), String> {
    std::fs::write(
        "review/calibration-closeout-audit-recheck.json",
        serde_json::to_string_pretty(report).unwrap(),
    )
    .map_err(|e| e.to_string())
}
static HTTP_ATTEMPTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn provider(config: &LlmSettings, lang: Lang) -> Result<InterviewProvider, String> {
    use std::io::Write;
    Ok(
        InterviewProvider::new(config, credentials::read_key(&config.provider)?)?
            .lang(lang)
            .before_request(|| {
                if std::env::var_os("INTERVIEW_QUALITY_FINAL").is_some() {
                    // Reserve the worst-case 12 attempts used by the earlier failed operation.
                    let n = HTTP_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if n >= 147 {
                        return Err("Approved evaluation HTTP-attempt budget exhausted".into());
                    }
                    std::fs::write(
                        "review/calibration-closeout-http-attempts.txt",
                        (n + 1).to_string(),
                    )
                    .map_err(|e| e.to_string())?;
                }
                Ok(())
            })
            .on_text(|partial| {
                if std::env::var_os("INTERVIEW_QUALITY_FOLLOWUP_RECHECK").is_some() {
                    std::fs::write("review/calibration-closeout-epoll-partial.txt",partial).expect("evaluation partial trace");
                }
            })
            .on_review(|proposal, verdict| {
                if std::env::var_os("INTERVIEW_QUALITY_FINAL").is_some() {
                    let entry =
                        json!({"policy":swe_interview_prep::quality::POLICY_VERSION,"proposal":proposal,"verdict":verdict});
                    let mut file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("review/calibration-closeout-reviews.jsonl")
                        .expect("evaluation trace file");
                    writeln!(file, "{}", entry).expect("evaluation trace write");
                }
            }),
    )
}
fn design_session(case: &Value, config: &LlmSettings) -> DesignSession {
    let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
    DesignSession {
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
    }
}
fn main() -> Result<(), String> {
    let provider_name = std::env::var("INTERVIEW_PROVIDER").unwrap_or_else(|_| "claude".into());
    let model = std::env::var("INTERVIEW_MODEL").unwrap_or_else(|_| {
        if provider_name == "deepseek" {
            "deepseek-flash"
        } else {
            "claude-haiku-5-5"
        }
        .into()
    });
    let config = LlmSettings {
        provider: provider_name,
        model,
        timeout_seconds: 120,
    };
    let cases: Value =
        serde_json::from_str(include_str!("../../content/evals/interview-quality.json")).unwrap();
    let focused = std::env::var_os("INTERVIEW_QUALITY_FOCUS").is_some();
    let closeout = std::env::var_os("INTERVIEW_QUALITY_FINAL").is_some();
    if closeout {
        std::fs::create_dir_all("review").map_err(|e| e.to_string())?;
    }
    let final_cases: Value = serde_json::from_str(include_str!(
        "../../content/evals/calibration-closeout.json"
    ))
    .unwrap();
    if closeout && std::env::var_os("INTERVIEW_QUALITY_FOLLOWUP_RECHECK").is_some() {
        let used: usize = std::fs::read_to_string("review/calibration-closeout-http-attempts.txt")
            .map_err(|e| e.to_string())?
            .trim()
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        HTTP_ATTEMPTS.store(used, std::sync::atomic::Ordering::SeqCst);
        let mut report: Vec<Value> = serde_json::from_str(
            &std::fs::read_to_string("review/calibration-closeout-live.json")
                .map_err(|e| e.to_string())?,
        )
        .unwrap();
        let entry = report
            .iter_mut()
            .find(|e| e["input"]["id"] == "p1-unknown-epoll-en")
            .unwrap();
        let case = entry["input"].clone();
        let question = serde_json::from_value(case["question"].clone()).unwrap();
        let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
        let follow = provider(&config, Lang::En)?.next_turn(&question, &turns)?;
        std::fs::write(
            "review/calibration-closeout-epoll-followup.json",
            serde_json::to_string_pretty(
                &json!({"before":entry["followUp"],"after":follow,"input":case}),
            )
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        entry["followUp"] = json!(follow);
        entry["followUpRechecked"] = json!(true);
        save(&report)?;
        println!("Completed real epoll prerequisite follow-up recheck");
        return Ok(());
    }
    if closeout && std::env::var_os("INTERVIEW_QUALITY_REPAIR_FAILED").is_some() {
        let used: usize = std::fs::read_to_string("review/calibration-closeout-http-attempts.txt")
            .map_err(|e| e.to_string())?
            .trim()
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        HTTP_ATTEMPTS.store(used, std::sync::atomic::Ordering::SeqCst);
        let checks: Vec<Value> = serde_json::from_str(
            &std::fs::read_to_string("review/calibration-closeout-audit-recheck.json")
                .map_err(|e| e.to_string())?,
        )
        .unwrap();
        let mut report: Vec<Value> = serde_json::from_str(
            &std::fs::read_to_string("review/calibration-closeout-live.json")
                .map_err(|e| e.to_string())?,
        )
        .unwrap();
        std::fs::write(
            "review/calibration-closeout-before-grounded-repairs.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        for check in checks
            .iter()
            .skip(1)
            .filter(|c| c["expectedAccepted"] == true && c["output"]["accepted"] == false)
        {
            let entry = &check["input"];
            let case = &entry["input"];
            let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
            let api = provider(&config, lang)?;
            let feedback = if entry["kind"] == "p1" {
                let question = serde_json::from_value(case["question"].clone()).unwrap();
                let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
                let mut input =
                    swe_interview_prep::llm::fundamentals_feedback_context(&question, &turns);
                input["previousProposal"] = entry["feedback"].clone();
                input["qualityReviewIssues"] = check["output"]["issues"].clone();
                let f: Feedback = api.calibrated_with(
                    lang,
                    [
                        swe_interview_prep::llm::FEEDBACK_ZH,
                        swe_interview_prep::llm::FEEDBACK_EN,
                    ],
                    input,
                    swe_interview_prep::llm::feedback_schema(),
                    false,
                    "Repair only the flagged feedback evidence; preserve correct fields",
                    |f| swe_interview_prep::llm::normalize_feedback(f, &turns, lang),
                )?;
                json!(f)
            } else {
                let session = design_session(case, &config);
                let mut input = system_design::feedback_context(&session);
                input["previousProposal"] = entry["feedback"].clone();
                input["qualityReviewIssues"] = check["output"]["issues"].clone();
                let raw: system_design::RawDesignFeedback = api.calibrated_with(
                    lang,
                    system_design::FEEDBACK,
                    input,
                    system_design::feedback_schema(),
                    true,
                    "Repair only flagged rating/evidence contradictions; no unasked deductions",
                    |mut r: system_design::RawDesignFeedback| {
                        r.summary = system_design::design_summary(&r.assessment, &session);
                        r
                    },
                )?;
                json!(system_design::checked_feedback(raw, &session))
            };
            let target = report
                .iter_mut()
                .find(|e| e["kind"] == entry["kind"] && e["input"]["id"] == case["id"])
                .unwrap();
            target["feedback"] = feedback;
            target["groundedRepairFromFinalReview"] = json!(true);
            save(&report)?;
            println!("Completed grounded real feedback repair: {}", case["id"]);
        }
        return Ok(());
    }
    if closeout && std::env::var_os("INTERVIEW_QUALITY_AUDIT_ONLY").is_some() {
        let used: usize = std::fs::read_to_string("review/calibration-closeout-http-attempts.txt")
            .map_err(|e| e.to_string())?
            .trim()
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        HTTP_ATTEMPTS.store(used, std::sync::atomic::Ordering::SeqCst);
        let mut prior: Value = serde_json::from_str(
            &std::fs::read_to_string("review/calibration-closeout-live.json")
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        std::fs::write(
            "review/calibration-closeout-before-final-projection.json",
            serde_json::to_string_pretty(&prior).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        // Apply only the production summary projection; retain real model ratings/evidence.
        for entry in prior.as_array_mut().unwrap().iter_mut() {
            let case = entry["input"].clone();
            if entry["kind"] == "p1" {
                let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
                let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
                let mut f: Feedback = serde_json::from_value(entry["feedback"].clone()).unwrap();
                f.summary = swe_interview_prep::quality::summary(
                    lang,
                    &f.dimensions
                        .iter()
                        .map(|d| (d.name.as_str(), &d.rating))
                        .collect::<Vec<_>>(),
                    turns
                        .iter()
                        .filter(|t| t.role == "candidate" && !t.ai_assisted)
                        .count(),
                    turns
                        .iter()
                        .filter(|t| t.role == "candidate" && t.ai_assisted)
                        .count(),
                );
                entry["feedback"] = json!(f);
                entry["localSummaryProjection"] =
                    json!(swe_interview_prep::quality::POLICY_VERSION);
            } else if entry["kind"] == "p2" {
                let f = serde_json::from_value(entry["feedback"].clone()).unwrap();
                entry["feedback"] = json!(system_design::with_stage_coverage(
                    f,
                    &design_session(&case, &config)
                ));
                entry["localSummaryProjection"] =
                    json!(swe_interview_prep::quality::POLICY_VERSION);
            }
        }
        std::fs::write(
            "review/calibration-closeout-live.json",
            serde_json::to_string_pretty(&prior).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let only_failed = std::env::var_os("INTERVIEW_QUALITY_RECHECK_FAILED").is_some();
        let mut results = if only_failed {
            serde_json::from_str::<Vec<Value>>(
                &std::fs::read_to_string("review/calibration-closeout-audit-recheck.json")
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?
        } else {
            vec![
                json!({"provider":config.provider,"model":config.model,"kind":"final-policy-recheck","httpAttemptsBefore":used}),
            ]
        };
        let failed_ids: Vec<Value> = results
            .iter()
            .filter(|c| c["expectedAccepted"] == true && c["output"]["accepted"] == false)
            .map(|c| c["input"]["input"]["id"].clone())
            .collect();
        if only_failed {
            std::fs::write(
                "review/calibration-closeout-recheck-before-grounded-repairs.json",
                serde_json::to_string_pretty(&results).unwrap(),
            )
            .map_err(|e| e.to_string())?;
        }
        for entry in prior
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| matches!(e["kind"].as_str(), Some("generation" | "p1" | "p2")))
        {
            if only_failed && !failed_ids.contains(&entry["input"]["id"]) {
                continue;
            }
            let lang: Lang = serde_json::from_value(entry["input"]["lang"].clone()).unwrap();
            let api = provider(&config, lang)?;
            let case = &entry["input"];
            let (purpose, input, proposal) = if entry["kind"] == "generation" {
                (
                    "P1 opening: only visible fields can leak",
                    json!({"topic":interview::topic(case["seedId"].as_str().unwrap())?.localized(lang)}),
                    entry["output"].clone(),
                )
            } else if entry["kind"] == "p1" {
                let question = serde_json::from_value(case["question"].clone()).unwrap();
                let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
                ("P1 feedback: independent evidence, actual asked scope and consistent assessment",swe_interview_prep::llm::fundamentals_feedback_context(&question,&turns),entry["feedback"].clone())
            } else {
                (
                    "P2 feedback: independent evidence in any stage and relevant gaps",
                    system_design::feedback_context(&design_session(case, &config)),
                    entry["feedback"].clone(),
                )
            };
            let verdict = api.review_quality(lang, purpose, input, proposal)?;
            results.retain(|c| {
                !(c["expectedAccepted"] == true
                    && c["input"]["kind"] == entry["kind"]
                    && c["input"]["input"] == entry["input"])
            });
            results.push(json!({"input":entry,"output":verdict,"expectedAccepted":true}));
            save_recheck(&results)?;
        }
        for case in final_cases["reviewProbes"].as_array().unwrap() {
            if only_failed {
                continue;
            }
            let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
            let api = provider(&config, lang)?;
            let verdict = api.review_quality(
                lang,
                case["purpose"].as_str().unwrap(),
                case["input"].clone(),
                case["proposal"].clone(),
            )?;
            results.push(json!({"input":case,"output":verdict,"expectedAccepted":false}));
            save_recheck(&results)?;
        }
        std::fs::write(
            "review/calibration-closeout-audit-recheck.json",
            serde_json::to_string_pretty(&results).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        println!("Completed final-policy recheck on accepted outputs and adversarial probes");
        return Ok(());
    }
    if closeout {
        let used = match std::fs::read_to_string("review/calibration-closeout-http-attempts.txt") {
            Ok(s) => s.trim().parse::<usize>().map_err(|e| e.to_string())?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
            Err(e) => return Err(e.to_string()),
        };
        HTTP_ATTEMPTS.store(used, std::sync::atomic::Ordering::SeqCst);
    }
    let planned = if closeout {
        17
    } else if focused {
        8
    } else {
        16
    };
    let mut report = vec![json!({"provider":config.provider,"model":config.model,
        "description":cases["description"],"logicalRequestsPlanned":planned,"focused":focused,"closeout":closeout,"semanticReviewEnabled":true,
        "providerOperationsNormally":if closeout {31} else if focused {14} else {26},
        "note":"Business operations include semantic review and at most one repair; transport/schema success is not human semantic acceptance. Content decoding has separate bounded retries."})];
    if closeout && std::env::var_os("INTERVIEW_QUALITY_RESUME").is_some() {
        report = serde_json::from_str(
            &std::fs::read_to_string("review/calibration-closeout-live.json")
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        report[0]["resumedWithSeparateReviewTasks"] = json!(true);
    }
    let refreshing = closeout && std::env::var_os("INTERVIEW_QUALITY_FEEDBACK_REFRESH").is_some();
    let prior_outputs = if refreshing { report.clone() } else { vec![] };
    if refreshing {
        std::fs::write(
            "review/calibration-closeout-before-canonical-summary.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        report.retain(|e| !matches!(e["kind"].as_str(), Some("p1" | "p2" | "review-probe")));
        report[0]["summaryPolicy"] = json!("derived-from-ratings-and-local-provenance");
    }
    save(&report)?;
    for case in cases["generations"].as_array().unwrap() {
        if report
            .iter()
            .any(|e| e["kind"] == "generation" && e["input"] == *case)
        {
            continue;
        }
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api = provider(&config, lang)?;
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
        if report
            .iter()
            .any(|e| e["kind"] == "p1" && e["input"]["id"] == case["id"])
        {
            continue;
        }
        if focused
            && !matches!(
                case["id"].as_str(),
                Some("p1-correct-event-en" | "p1-partial-mvcc-zh")
            )
        {
            continue;
        }
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api = provider(&config, lang)?;
        let question: GeneratedQuestion = serde_json::from_value(case["question"].clone()).unwrap();
        let turns: Vec<Turn> = serde_json::from_value(case["turns"].clone()).unwrap();
        let follow = if let Some(prior) = prior_outputs
            .iter()
            .find(|e| e["kind"] == "p1" && e["input"]["id"] == case["id"])
        {
            serde_json::from_value(prior["followUp"].clone()).map_err(|e| e.to_string())?
        } else if case["feedbackOnly"] == true {
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
    let mut design_cases = cases["p2Cases"].as_array().unwrap().clone();
    if closeout {
        design_cases.push(final_cases["crossStageCase"].clone());
    }
    for case in &design_cases {
        if report
            .iter()
            .any(|e| e["kind"] == "p2" && e["input"]["id"] == case["id"])
        {
            continue;
        }
        let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
        let api = provider(&config, lang)?;
        let session = design_session(case, &config);
        let follow = if let Some(prior) = prior_outputs
            .iter()
            .find(|e| e["kind"] == "p2" && e["input"]["id"] == case["id"])
        {
            serde_json::from_value(prior["followUp"].clone()).map_err(|e| e.to_string())?
        } else if focused || case["id"] == "p2-cross-stage-evidence-en" {
            None
        } else {
            Some(api.design_follow_up(&session)?)
        };
        let feedback = api.design_feedback(&session)?;
        report.push(json!({"kind":"p2","input":case,"followUp":follow,"feedback":feedback}));
        save(&report)?;
        println!("Completed real P2 case: {}", case["id"]);
    }
    if closeout && std::env::var_os("INTERVIEW_QUALITY_SKIP_PROBES").is_none() {
        for case in final_cases["reviewProbes"].as_array().unwrap() {
            let lang: Lang = serde_json::from_value(case["lang"].clone()).unwrap();
            let api = InterviewProvider::new(&config, credentials::read_key(&config.provider)?)?
                .lang(lang);
            let verdict = api.review_quality(
                lang,
                case["purpose"].as_str().unwrap(),
                case["input"].clone(),
                case["proposal"].clone(),
            )?;
            report.retain(|e| !(e["kind"] == "review-probe" && e["input"]["id"] == case["id"]));
            report.push(json!({"kind":"review-probe","input":case,"output":verdict}));
            save(&report)?;
            println!("Completed real review probe: {}", case["id"]);
        }
    }
    println!("Completed {planned} logical real requests; semantic review remains manual. Focused batch: {focused}");
    Ok(())
}
