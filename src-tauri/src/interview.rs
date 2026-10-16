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
    #[serde(default)]
    pub question_form: QuestionForm,
    #[serde(default)]
    pub priority: TopicPriority,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<TopicEnglish>,
}
/// Opening style; older authored topics retain their scenario-based behavior.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionForm {
    Foundation,
    #[default]
    Scenario,
    Troubleshooting,
}
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicPriority {
    #[default]
    Core,
    Advanced,
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
    /// Interviewer-only answer propositions, excluding neutral subject names. Questions
    /// cached before this field existed have none.
    #[serde(default)]
    pub answer_terms: Vec<String>,
}
impl GeneratedQuestion {
    /// Strict generation-only requirements; legacy cache reads use validate().
    pub fn validate_fresh(&self) -> Result<(), String> {
        self.validate()?;
        if !self
            .evaluation_checklist
            .iter()
            .any(|c| !marked_optional(c))
        {
            return Err("生成题目字段为空、过长或缺少约束/检查点，请重试".into());
        }
        let terms: std::collections::HashSet<_> = self
            .answer_terms
            .iter()
            .map(|t| t.trim().to_lowercase())
            .filter(|t| t.chars().count() >= 4 || (t.chars().count() >= 2 && !t.is_ascii()))
            .collect();
        if !(2..=4).contains(&self.answer_terms.len()) || terms.len() != self.answer_terms.len() {
            return Err("新题缺少有效且不同的答案要点，请重试".into());
        }
        Ok(())
    }
    /// The first answer term that also appears in a visible field, if any.
    pub fn leaked_term(&self) -> Option<&str> {
        let visible = std::iter::once(&self.question)
            .chain(std::iter::once(&self.background))
            .chain(&self.constraints)
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join("\n");
        self.answer_terms
            .iter()
            .map(|t| t.trim())
            .filter(|t| {
                // Very short terms (e.g. "B") match unrelated text.
                let n = t.chars().count();
                n >= 4 || (n >= 2 && !t.is_ascii())
            })
            .find(|t| visible.contains(&t.to_lowercase()))
    }
    /// Moves checkpoints the model itself marked optional into the follow-up
    /// directions, so the opening is graded only on what it actually asks.
    pub fn move_optional_checkpoints(&mut self) {
        let (optional, required): (Vec<_>, Vec<_>) = self
            .evaluation_checklist
            .drain(..)
            .partition(|c| marked_optional(c));
        self.evaluation_checklist = required;
        for item in optional {
            if self.follow_up_directions.len() < 12 {
                self.follow_up_directions.push(item);
            }
        }
    }
}
/// Separate wire type prevents legacy defaults from weakening new generation.
#[derive(Serialize)]
#[serde(transparent)]
pub struct FreshQuestion(pub GeneratedQuestion);
impl<'de> Deserialize<'de> for FreshQuestion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut q = GeneratedQuestion::deserialize(deserializer)?;
        q.move_optional_checkpoints();
        Ok(Self(q))
    }
}
impl Validated for FreshQuestion {
    fn validate(&self) -> Result<(), String> {
        self.0.validate_fresh()
    }
}
fn marked_optional(s: &str) -> bool {
    let lower = s.to_lowercase();
    [
        "可选",
        "选做",
        "加分",
        "非必需",
        "非必须",
        "不要求",
        "不作要求",
        "不是必需",
        "optional",
        "not required",
        "bonus",
        "extra credit",
    ]
    .iter()
    .any(|m| lower.contains(m))
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
            && list(&self.answer_terms, 0)
        {
            if self.leaked_term().is_some() {
                return Err("生成题目的可见内容泄露了答案要点，请重试".into());
            }
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

#[cfg(test)]
mod calibration_tests {
    use super::*;
    fn question() -> GeneratedQuestion {
        GeneratedQuestion {
            question: "What does this code print, and why?".into(),
            background: "A browser runs one script with a promise and a timer.".into(),
            constraints: vec!["Chrome, no other tasks queued".into()],
            follow_up_directions: vec!["Change one callback".into()],
            evaluation_checklist: vec!["Explains why C prints before B".into()],
            answer_terms: vec!["microtask checkpoint".into(), "Run-to-completion".into()],
        }
    }
    #[test]
    fn visible_answer_terms_are_rejected_case_insensitively() {
        let q = question();
        assert!(q.leaked_term().is_none() && q.validate().is_ok());
        let mut leaked = question();
        leaked.background = "After run-to-completion the engine does more work.".into();
        assert_eq!(leaked.leaked_term(), Some("Run-to-completion"));
        assert!(leaked.validate().unwrap_err().contains("泄露"));
        let mut zh = question();
        zh.answer_terms = vec!["语句级快照".into()];
        zh.constraints = vec!["隔离级别为 READ COMMITTED，每条语句使用语句级快照".into()];
        assert!(zh.validate().is_err());
        // Single letters and short ASCII names appear in code and must not count.
        let mut short = question();
        short.answer_terms = vec!["B".into(), "C".into()];
        assert!(short.validate().is_ok());
    }
    #[test]
    fn fresh_generation_rejects_missing_empty_short_and_duplicate_terms_but_legacy_loads() {
        let mut q = question();
        q.answer_terms.clear();
        assert!(q.validate().is_ok());
        assert!(q.validate_fresh().is_err());
        q.answer_terms = vec!["B".into(), "C".into()];
        assert!(q.validate_fresh().is_err());
        q.answer_terms = vec!["checkpoint".into(), "CHECKPOINT".into()];
        assert!(q.validate_fresh().is_err());
        let mut old = serde_json::to_value(question()).unwrap();
        old.as_object_mut().unwrap().remove("answerTerms");
        let legacy: GeneratedQuestion = serde_json::from_value(old.clone()).unwrap();
        legacy.validate().unwrap();
        let fresh: FreshQuestion = serde_json::from_value(old).unwrap();
        assert!(fresh.validate().is_err());
        question().validate_fresh().unwrap();
    }
    #[test]
    fn optional_checkpoints_move_to_follow_up_directions() {
        let mut q = question();
        q.evaluation_checklist
            .push("可选：比较 REPEATABLE READ（不要求）".into());
        q.evaluation_checklist
            .push("Optional: mention xmin/xmax".into());
        q.move_optional_checkpoints();
        assert_eq!(
            q.evaluation_checklist,
            vec!["Explains why C prints before B"]
        );
        assert_eq!(q.follow_up_directions.len(), 3);
        let mut all_optional = question();
        all_optional.evaluation_checklist = vec!["Bonus: discuss TTL".into()];
        assert!(all_optional.validate().is_ok()); // Legacy caches keep their original content.
        assert!(all_optional.validate_fresh().is_err());
    }
}
