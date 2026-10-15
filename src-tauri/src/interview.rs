use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Topic {
    pub id: String,
    pub category: String,
    pub title: String,
    pub focus: Vec<String>,
    pub scenario_constraints: Vec<String>,
    pub level: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<TopicEnglish>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopicEnglish {
    pub title: String,
    pub focus: Vec<String>,
    pub scenario_constraints: Vec<String>,
}
/// Language of the interview: the questions, follow-ups, answers and feedback.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    Zh,
    En,
}
impl Topic {
    /// The topic as the model sees it, in the interview language.
    pub fn localized(&self, lang: Lang) -> Topic {
        let mut t = self.clone();
        if let (Lang::En, Some(en)) = (lang, t.en.take()) {
            t.title = en.title;
            t.focus = en.focus;
            t.scenario_constraints = en.scenario_constraints;
        }
        t.en = None;
        t
    }
}
pub fn topics() -> Vec<Topic> {
    serde_json::from_str(include_str!("../../content/deep-dive/topics.json"))
        .expect("bundled topics")
}
pub fn topic(id: &str) -> Result<Topic, String> {
    topics()
        .into_iter()
        .find(|t| t.id == id)
        .ok_or_else(|| "未知 Deep Dive 主题".into())
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeneratedQuestion {
    pub question: String,
    pub background: String,
    pub constraints: Vec<String>,
    pub follow_up_directions: Vec<String>,
    pub evaluation_checklist: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FollowUp {
    pub observation: String,
    pub follow_up_question: String,
    pub should_end: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReferenceAnswer {
    pub answer: String,
    pub key_points: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceView {
    pub turn_index: usize,
    #[serde(flatten)]
    pub content: ReferenceAnswer,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rating {
    Good,
    Partial,
    Missing,
    Incorrect,
    Uncertain,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Dimension {
    pub name: String,
    pub rating: Rating,
    pub evidence: String,
    pub improvement: String,
}
pub const DIMENSIONS: [&str; 5] = [
    "principles",
    "selection_rationale",
    "tradeoffs",
    "failure_and_correctness",
    "communication",
];
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Feedback {
    pub summary: String,
    pub dimensions: Vec<Dimension>,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub next_practice_topics: Vec<String>,
}
fn text(s: &str) -> bool {
    !s.trim().is_empty() && s.chars().count() <= 6000
}
fn list(v: &[String], min: usize) -> bool {
    v.len() >= min && v.len() <= 12 && v.iter().all(|s| text(s))
}
pub trait Validated {
    fn validate(&self) -> Result<(), String>;
}
impl Validated for GeneratedQuestion {
    fn validate(&self) -> Result<(), String> {
        if text(&self.question)
            && text(&self.background)
            && list(&self.constraints, 1)
            && list(&self.follow_up_directions, 1)
            && list(&self.evaluation_checklist, 1)
        {
            Ok(())
        } else {
            Err("生成题目字段为空、过长或缺少约束/检查点，请重试".into())
        }
    }
}
impl Validated for FollowUp {
    fn validate(&self) -> Result<(), String> {
        if !text(&self.observation) {
            return Err("追问观察为空或超过6000字符，请重试".into());
        }
        if !text(&self.follow_up_question) {
            return Err("追问问题为空或超过6000字符，请重试".into());
        }
        Ok(())
    }
}
impl Validated for ReferenceAnswer {
    fn validate(&self) -> Result<(), String> {
        if text(&self.answer) && list(&self.key_points, 1) {
            Ok(())
        } else {
            Err("参考答案为空、过长或缺少要点，请重试".into())
        }
    }
}
impl Validated for Feedback {
    fn validate(&self) -> Result<(), String> {
        let names: std::collections::HashSet<_> =
            self.dimensions.iter().map(|d| d.name.as_str()).collect();
        if self.dimensions.len() != 5
            || names.len() != 5
            || !DIMENSIONS.iter().all(|n| names.contains(n))
        {
            return Err(format!(
                "反馈必须包含五个不同评估维度（收到 {} 项、{} 个不同名称），请重试",
                self.dimensions.len(),
                names.len()
            ));
        }
        if !text(&self.summary) {
            return Err("反馈总结为空或过长，请重试".into());
        }
        if self
            .dimensions
            .iter()
            .any(|d| !text(&d.evidence) || !text(&d.improvement))
        {
            return Err("反馈维度的依据或改进建议为空或过长，请重试".into());
        }
        if !list(&self.strengths, 0) || !list(&self.gaps, 0) {
            return Err("反馈优点或不足列表无效，请重试".into());
        }
        if !list(&self.next_practice_topics, 1) {
            return Err("反馈必须提供至少一个后续练习方向，且不超过12项，请重试".into());
        }
        Ok(())
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionView {
    pub id: String,
    pub seed_id: String,
    pub provider: String,
    pub model: String,
    pub question: String,
    pub background: String,
    pub constraints: Vec<String>,
    pub created_at: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Turn {
    #[serde(default)]
    pub ai_assisted: bool,
    pub role: String,
    pub text: String,
    pub timestamp: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionView {
    pub id: String,
    pub question: QuestionView,
    pub provider: String,
    pub model: String,
    pub status: String,
    pub draft: String,
    pub last_error: Option<String>,
    pub should_end: bool,
    pub turns: Vec<Turn>,
    pub feedback: Option<Feedback>,
    pub references: Vec<ReferenceView>,
    pub created_at: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub id: String,
    pub seed_id: String,
    pub title: String,
    pub status: String,
    pub provider: String,
    pub model: String,
    pub created_at: String,
}
pub trait LlmProvider {
    fn generate_question(&self, topic: &Topic) -> Result<GeneratedQuestion, String>;
    fn next_turn(&self, question: &GeneratedQuestion, turns: &[Turn]) -> Result<FollowUp, String>;
    fn reference_answer(
        &self,
        question: &GeneratedQuestion,
        target_question: &str,
        turns: &[Turn],
    ) -> Result<ReferenceAnswer, String>;
    fn summarize(&self, question: &GeneratedQuestion, turns: &[Turn]) -> Result<Feedback, String>;
}
