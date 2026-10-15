use crate::harness::{self, Io, Ty};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::OnceLock};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub statement: String,
    pub constraints: Vec<String>,
    pub complexity: Complexity,
    pub limits: JudgeLimits,
    pub examples: Value,
    pub cpp: Cpp,
    pub io: Io,
    // Hidden tests stay in the backend; the problem list sent to the UI omits them.
    #[serde(skip_serializing)]
    pub tests: Vec<Test>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Complexity {
    pub time: String,
    pub space: String,
    pub official_requirement: String,
    pub notes: String,
    pub source_url: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeLimits {
    pub time_limit_ms: u64,
    pub memory_limit_mb: u64,
    pub output_limit_kb: usize,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpp {
    pub function_template: String,
    pub acm_template: String,
    pub acm_io_spec: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    pub id: String,
    pub logical_input: Value,
    pub expected: Value,
    pub is_sample: bool,
}
// build.rs embeds every content/leetcode/problems/*.json as (file name, contents).
include!(concat!(env!("OUT_DIR"), "/problem_files.rs"));
struct Library {
    problems: Vec<Problem>,
    by_id: HashMap<String, usize>,
}
fn library() -> &'static Library {
    static LIBRARY: OnceLock<Library> = OnceLock::new();
    LIBRARY.get_or_init(|| {
        let mut problems: Vec<Problem> = PROBLEM_FILES
            .iter()
            .map(|(file, text)| {
                let p: Problem = serde_json::from_str(text)
                    .unwrap_or_else(|e| panic!("bundled problem {file} is invalid JSON: {e}"));
                if let Err(e) = validate(&p) {
                    panic!("bundled problem {file} is invalid: {e}");
                }
                p
            })
            .collect();
        problems.sort_by_key(|p| {
            let n = p.id.trim_start_matches(|c: char| !c.is_ascii_digit());
            (n.parse::<u32>().unwrap_or(u32::MAX), p.id.clone())
        });
        let mut by_id = HashMap::new();
        for (i, p) in problems.iter().enumerate() {
            if by_id.insert(p.id.clone(), i).is_some() {
                panic!("duplicate problem id {}", p.id);
            }
        }
        Library { problems, by_id }
    })
}
/// Structural checks every bundled problem must pass before it is shown or judged.
pub fn validate(p: &Problem) -> Result<(), String> {
    p.io.validate()?;
    if p.tests.is_empty() || !p.tests.iter().any(|t| t.is_sample) {
        return Err("needs at least one sample test".into());
    }
    for (i, t) in p.tests.iter().enumerate() {
        if p.tests[..i].iter().any(|u| u.id == t.id) {
            return Err(format!("duplicate test id {}", t.id));
        }
        p.io.stdin_for(&t.logical_input)
            .map_err(|e| format!("test {}: logicalInput: {e}", t.id))?;
        p.io.check_expected(&t.expected)
            .map_err(|e| format!("test {}: expected: {e}", t.id))?;
    }
    Ok(())
}
/// All bundled problems, parsed and validated once per process.
pub fn problems() -> &'static [Problem] {
    &library().problems
}
pub fn find(id: &str) -> Result<&'static Problem, String> {
    let lib = library();
    lib.by_id
        .get(id)
        .map(|&i| &lib.problems[i])
        .ok_or_else(|| "未知题目".into())
}
pub fn problem(id: &str) -> Result<Problem, String> {
    find(id).cloned()
}
pub const HEADERS: &str = "#include <iostream>\n#include <vector>\n#include <string>\n#include <algorithm>\n#include <unordered_map>\n#include <unordered_set>\n#include <queue>\n#include <stack>\n#include <set>\n#include <map>\n#include <climits>\nusing namespace std;\n";
pub fn input(p: &Problem, t: &Test) -> String {
    p.io.stdin_for(&t.logical_input)
        .expect("tests are validated when the library loads")
}
pub fn source(p: &Problem, mode: &str, code: &str) -> Result<String, String> {
    if mode == "acm" {
        return Ok(code.into());
    }
    if mode != "function" {
        return Err("未知编程模式".into());
    }
    let mut prelude = String::from(HEADERS);
    if p.io.uses(Ty::List) {
        prelude += harness::LIST_STRUCT;
    }
    if p.io.uses(Ty::Tree) {
        prelude += harness::TREE_STRUCT;
    }
    Ok(format!(
        "{}\n#line 1 \"user.cpp\"\n{}\n#line 1 \"harness.cpp\"\n{}",
        prelude,
        code,
        p.io.cpp_main()
    ))
}
pub fn compare(p: &Problem, t: &Test, output: &str) -> bool {
    p.io.accepts(&t.logical_input, &t.expected, output)
}
