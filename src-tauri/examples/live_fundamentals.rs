//! Explicit live prompt-quality probe. Calls Claude, prints generated content, never credentials.
//! Uses no application database and makes no credential writes.
use serde_json::json;
use swe_interview_prep::{
    credentials::{self, LlmSettings},
    interview::{self, Lang, LlmProvider, Turn},
    llm::InterviewProvider,
};

fn main() -> Result<(), String> {
    let config = LlmSettings {
        provider: "claude".into(),
        model: "claude-haiku-5-5".into(),
        timeout_seconds: 120,
    };
    let mut report = Vec::new();
    for lang in [Lang::Zh, Lang::En] {
        let api = InterviewProvider::new(&config, credentials::read_key("claude")?)?.lang(lang);
        let question = api.generate_question(&interview::topic("cache-selection")?)?;
        println!("{}", json!({"lang":lang,"question":question}));
        let mut turns = vec![Turn {
            role: "candidate".into(),
            text: match lang {
                Lang::Zh => "我不太清楚，可以从读写的执行顺序开始问吗？",
                Lang::En => {
                    "I am not sure. Can we start with the order of the read and write operations?"
                }
            }
            .into(),
            ai_assisted: false,
            timestamp: String::new(),
        }];
        let basic = api.next_turn(&question, &turns)?;
        println!("{}", json!({"lang":lang,"prerequisiteFollowUp":basic}));
        turns.push(Turn {
            role: "interviewer".into(),
            text: basic.follow_up_question.clone(),
            ai_assisted: false,
            timestamp: String::new(),
        });
        turns.push(Turn { role:"candidate".into(), text:match lang {
            Lang::Zh => "读请求已经把 v1 放在自己的内存里，之后即使写请求提交 v2 并删除缓存，读请求也可能把手上的 v1 回填。删除缓存并不会取消这个正在进行的读请求。",
            Lang::En => "The reader already holds v1 in memory. Even after the writer commits v2 and deletes the cache, the reader can cache its v1. Deleting the cache does not cancel that in-flight reader.",
        }.into(), ai_assisted:false, timestamp:String::new() });
        let boundary = api.next_turn(&question, &turns)?;
        println!("{}", json!({"lang":lang,"boundaryFollowUp":boundary}));
        // Leave the final follow-up unanswered to check feedback does not invent evidence.
        turns.push(Turn {
            role: "interviewer".into(),
            text: boundary.follow_up_question.clone(),
            ai_assisted: false,
            timestamp: String::new(),
        });
        let feedback = api.summarize(&question, &turns)?;
        let sample = json!({"lang":lang,"question":question,"prerequisiteFollowUp":basic,"turns":turns,"boundaryFollowUp":boundary,"feedback":feedback});
        println!("{}", json!({"lang":lang,"feedback":feedback}));
        report.push(sample);
        std::fs::write(
            "/tmp/intv-p1-fundamentals-live.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
