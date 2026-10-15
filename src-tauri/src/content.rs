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
    // The cached library drops them too and `problem()` re-parses on demand.
    #[serde(default, skip_serializing)]
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
// build.rs embeds content/leetcode/problems/*.json and harnesses/*.cpp as (file name, contents).
include!(concat!(env!("OUT_DIR"), "/problem_files.rs"));
struct Library {
    // Metadata only: max-size tests are large, so they are parsed per judge run.
    problems: Vec<Problem>,
    by_id: HashMap<String, (usize, &'static str)>,
}
pub fn parse(text: &str) -> Result<Problem, String> {
    let p: Problem = serde_json::from_str(text).map_err(|e| format!("invalid JSON: {e}"))?;
    p.io.validate()?;
    if let Some(h) = &p.io.harness {
        harness_source(h)?;
    }
    Ok(p)
}
fn library() -> &'static Library {
    static LIBRARY: OnceLock<Library> = OnceLock::new();
    LIBRARY.get_or_init(|| {
        let mut problems: Vec<(Problem, &'static str)> = PROBLEM_FILES
            .iter()
            .map(|(file, text)| {
                let mut p = parse(text)
                    .unwrap_or_else(|e| panic!("bundled problem {file} is invalid: {e}"));
                p.tests = vec![];
                (p, *text)
            })
            .collect();
        problems.sort_by_key(|(p, _)| {
            let n = p.id.trim_start_matches(|c: char| !c.is_ascii_digit());
            (n.parse::<u32>().unwrap_or(u32::MAX), p.id.clone())
        });
        let mut by_id = HashMap::new();
        for (i, (p, text)) in problems.iter().enumerate() {
            if by_id.insert(p.id.clone(), (i, *text)).is_some() {
                panic!("duplicate problem id {}", p.id);
            }
        }
        Library {
            problems: problems.into_iter().map(|(p, _)| p).collect(),
            by_id,
        }
    })
}
/// Custom harness text: bundled files, or (development only, for verifying new
/// problems before a rebuild) `$INTV_CONTENT_DIR/harnesses/<name>`.
pub fn harness_source(name: &str) -> Result<String, String> {
    if let Some((_, text)) = HARNESS_FILES.iter().find(|(file, _)| *file == name) {
        return Ok(text.to_string());
    }
    if let Some(dir) = std::env::var_os("INTV_CONTENT_DIR") {
        let path = std::path::Path::new(&dir).join("harnesses").join(name);
        if let Ok(text) = std::fs::read_to_string(path) {
            return Ok(text);
        }
    }
    Err(format!("missing harness {name}"))
}
/// Structural checks every problem must pass before it is judged.
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
/// All bundled problems without tests, parsed once per process, by problem number.
pub fn problems() -> &'static [Problem] {
    &library().problems
}
/// Cached metadata (no tests) for a bundled problem.
pub fn find(id: &str) -> Result<&'static Problem, String> {
    let lib = library();
    lib.by_id
        .get(id)
        .map(|&(i, _)| &lib.problems[i])
        .ok_or_else(|| "未知题目".into())
}
/// A bundled problem with its full tests, parsed from the embedded JSON.
pub fn problem(id: &str) -> Result<Problem, String> {
    let (_, text) = library().by_id.get(id).ok_or("未知题目")?;
    parse(text)
}
pub const HEADERS: &str = "#include <iostream>\n#include <vector>\n#include <string>\n#include <algorithm>\n#include <unordered_map>\n#include <unordered_set>\n#include <queue>\n#include <stack>\n#include <set>\n#include <map>\n#include <climits>\n#include <deque>\n#include <list>\n#include <numeric>\n#include <functional>\n#include <cmath>\n#include <sstream>\n#include <utility>\n#include <tuple>\n#include <array>\n#include <bitset>\n#include <cstring>\n#include <cstdint>\n#include <limits>\nusing namespace std;\n";
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
    if let Some(extra) = &p.io.prelude {
        prelude += extra;
        prelude.push('\n');
    }
    let main = match &p.io.harness {
        Some(name) => p.io.custom_main(&harness_source(name)?),
        None => p.io.cpp_main(),
    };
    Ok(format!(
        "{}\n#line 1 \"user.cpp\"\n{}\n#line 1 \"harness.cpp\"\n{}",
        prelude, code, main
    ))
}
pub fn compare(p: &Problem, t: &Test, output: &str) -> bool {
    p.io.accepts(&t.logical_input, &t.expected, output)
}
