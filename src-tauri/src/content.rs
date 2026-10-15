use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub statement: String,
    pub constraints: Vec<String>,
    pub examples: Value,
    pub cpp: Cpp,
    pub tests: Vec<Test>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpp {
    pub function_template: String,
    pub acm_template: String,
    pub function_harness_id: String,
    pub acm_io_spec: String,
    pub comparator_id: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    pub id: String,
    pub logical_input: Value,
    pub expected: Value,
    pub is_sample: bool,
}
pub fn problems() -> Vec<Problem> {
    [
        include_str!("../../content/leetcode/problems/lc-1.json"),
        include_str!("../../content/leetcode/problems/lc-3.json"),
        include_str!("../../content/leetcode/problems/lc-206.json"),
        include_str!("../../content/leetcode/problems/lc-283.json"),
    ]
    .iter()
    .map(|s| serde_json::from_str(s).expect("bundled problem JSON"))
    .collect()
}
pub fn problem(id: &str) -> Result<Problem, String> {
    problems()
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| "未知题目".into())
}
pub const HEADERS: &str = "#include <iostream>\n#include <vector>\n#include <string>\n#include <algorithm>\n#include <unordered_map>\n#include <unordered_set>\n#include <queue>\n#include <stack>\n#include <set>\n#include <map>\n#include <climits>\nusing namespace std;\n";
pub const LIST: &str = "struct ListNode { int val; ListNode* next; ListNode(int x=0, ListNode* n=nullptr): val(x), next(n) {} };\n";
pub fn input(p: &Problem, t: &Test) -> String {
    let v = &t.logical_input;
    match p.cpp.function_harness_id.as_str() {
        "substring" => format!("{}\n", v.as_str().unwrap()),
        "two_sum" => format!(
            "{} {}\n{}\n",
            v["nums"].as_array().unwrap().len(),
            v["target"],
            numbers(&v["nums"])
        ),
        _ => format!("{}\n{}\n", v.as_array().unwrap().len(), numbers(v)),
    }
}
fn numbers(v: &Value) -> String {
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn source(p: &Problem, mode: &str, code: &str) -> Result<String, String> {
    if mode == "acm" {
        return Ok(code.into());
    }
    if mode != "function" {
        return Err("未知编程模式".into());
    }
    let harness = match p.cpp.function_harness_id.as_str() {
        "substring" => include_str!("../../content/leetcode/adapters/substring.cpp"),
        "two_sum" => include_str!("../../content/leetcode/adapters/two_sum.cpp"),
        "move_zeroes" => include_str!("../../content/leetcode/adapters/move_zeroes.cpp"),
        "reverse_list" => include_str!("../../content/leetcode/adapters/reverse_list.cpp"),
        _ => return Err("未实现的适配器".into()),
    };
    Ok(format!(
        "{}{}\n#line 1 \"user.cpp\"\n{}\n#line 1 \"harness.cpp\"\n{}",
        HEADERS,
        if p.cpp.function_harness_id == "reverse_list" {
            LIST
        } else {
            ""
        },
        code,
        harness
    ))
}
pub fn compare(p: &Problem, t: &Test, output: &str) -> bool {
    let parsed: Result<Vec<i64>, _> = output.split_whitespace().map(str::parse).collect();
    let Ok(values) = parsed else {
        return false;
    };
    if p.cpp.comparator_id == "valid_pair" {
        if values.len() != 2 || values[0] == values[1] {
            return false;
        }
        let nums = t.logical_input["nums"].as_array().unwrap();
        let (a, b) = (values[0], values[1]);
        a >= 0
            && b >= 0
            && (a as usize) < nums.len()
            && (b as usize) < nums.len()
            && nums[a as usize].as_i64().unwrap() + nums[b as usize].as_i64().unwrap()
                == t.logical_input["target"].as_i64().unwrap()
    } else {
        let expected: Vec<i64> = if let Some(a) = t.expected.as_array() {
            a.iter().map(|x| x.as_i64().unwrap()).collect()
        } else {
            vec![t.expected.as_i64().unwrap()]
        };
        values == expected
    }
}
