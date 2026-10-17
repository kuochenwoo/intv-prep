use std::time::{Duration, Instant};
use swe_interview_prep::{
    code_runner::{self, execute, judge},
    content::{self, problem},
    db,
};
// Reference solutions live in tests/solutions/<id>-function.cpp and <id>-acm.cpp.
fn solution(id: &str, mode: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/solutions")
        .join(format!("{id}-{mode}.cpp"));
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("missing {}", path.display()))
}
#[test]
fn every_problem_passes_reference_solutions_in_both_modes() {
    let ids: Vec<String> = content::problems().iter().map(|p| p.id.clone()).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    // Half the cores: per-test time limits are wall-clock, so avoid oversubscribing.
    let workers = std::thread::available_parallelism().map_or(2, |n| (n.get() / 2).clamp(1, 4));
    let failures = std::sync::Mutex::new(vec![]);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(id) = ids.get(i) else { break };
                let p = problem(id).unwrap();
                for mode in ["function", "acm"] {
                    // Independently written full programs verify the documented IO protocol.
                    let r = match judge(&p, mode, &solution(id, mode), false, "") {
                        Ok(result) => result,
                        Err(error) => {
                            failures
                                .lock()
                                .unwrap()
                                .push(format!("{id} {mode}: judge error: {error}"));
                            continue;
                        }
                    };
                    if r.status != "accepted" || r.passed != p.tests.len() || p.tests.is_empty() {
                        let case = r.cases.iter().find(|c| c.status != "accepted");
                        failures.lock().unwrap().push(format!(
                            "{id} {mode}: {} {}/{} {} {:?}",
                            r.status,
                            r.passed,
                            p.tests.len(),
                            r.diagnostics.chars().take(400).collect::<String>(),
                            case.map(|c| format!(
                                "case {} got {:.200} stderr {:.200}",
                                c.id, c.actual, c.stderr
                            ))
                        ));
                    }
                }
            });
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
#[test]
fn malformed_wrong_crashed_hung_and_noisy_programs_are_classified() {
    let p = problem("lc-3").unwrap();
    for (code, expected) in [
        ("not C++", "compile_error"),
        ("int main(){return 0;}", "wrong_answer"),
        ("int main(){return 42;}", "runtime_error"),
        (
            "int main(){volatile int x=0;while(true){x=x+1;}}",
            "timeout",
        ),
        (
            "#include <iostream>\nint main(){while(true)std::cout<<\"noise noise noise\";}",
            "output_limit",
        ),
        (
            "#include <cstdlib>\nint main(){std::abort();}",
            "runtime_error",
        ),
    ] {
        let r = judge(&p, "acm", code, true, "").unwrap();
        assert_eq!(r.status, expected, "{}: {}", code, r.diagnostics);
    }
    let r = judge(
        &p,
        "function",
        "class Solution{public:int lengthOfLongestSubstring(string){while(true){}}};",
        true,
        "",
    )
    .unwrap();
    assert_eq!(r.status, "timeout");
    assert!(judge(&p, "bad", "", false, "").is_err());
    assert!(code_runner::detect_compiler("/definitely/missing/compiler").is_err());
}
#[test]
fn semantic_comparator_accepts_alternatives_and_rejects_garbage() {
    let p = problem("lc-1").unwrap();
    let t = &p.tests[3];
    for s in ["0 3", "3 0\n", "1 2"] {
        assert!(content::compare(&p, t, s));
    }
    for s in ["0 0", "-1 4", "0 99", "0 3 extra", "0 3 1", "", "1.0 2"] {
        assert!(!content::compare(&p, t, s));
    }
    let p = problem("lc-206").unwrap();
    assert!(content::compare(&p, &p.tests[1], "\n  \t"));
    assert!(!content::compare(&p, &p.tests[1], "0"));
}
#[test]
fn persisted_drafts_modes_status_results_and_compiler_survive_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data/prep.sqlite3");
    {
        let c = db::open(&path).unwrap();
        db::save(&c, "lc-1", "function", "function draft").unwrap();
        db::save(&c, "lc-1", "acm", "ACM draft").unwrap();
        db::record(&c, "lc-1", "function", "{\"status\":\"accepted\"}", true).unwrap();
        db::set_compiler(&c, "/path with spaces/clang++").unwrap();
    }
    let c = db::open(&path).unwrap();
    db::save(&c, "lc-1", "function", "edited draft").unwrap();
    db::record(
        &c,
        "lc-1",
        "function",
        "{\"status\":\"wrong_answer\"}",
        false,
    )
    .unwrap();
    let rows = db::load(&c).unwrap();
    assert_eq!(rows.len(), 2);
    let f = rows.iter().find(|r| r.mode == "function").unwrap();
    assert_eq!(f.code, "edited draft");
    assert_eq!(f.status, "completed");
    assert!(f.last_result.as_ref().unwrap().contains("wrong_answer"));
    let a = rows.iter().find(|r| r.mode == "acm").unwrap();
    assert_eq!(a.code, "ACM draft");
    assert_eq!(a.status, "in_progress");
    assert!(a.last_result.is_none());
    assert_eq!(db::compiler(&c).unwrap(), "/path with spaces/clang++");
    assert!(db::save(&c, "unknown", "acm", "").is_err());
}
#[test]
#[cfg(unix)]
fn process_groups_kill_descendants_and_close_inherited_pipes() {
    let dir = tempfile::tempdir().unwrap();
    let start = Instant::now();
    let r = execute(
        "/bin/sh",
        &["-c", "sleep 20 & exit 0"],
        dir.path(),
        String::new(),
        Duration::from_millis(200),
        4096,
    )
    .unwrap();
    assert_eq!(r.status, "ok");
    assert!(start.elapsed() < Duration::from_secs(2));
    let r = execute(
        "/bin/sh",
        &["-c", "(sleep 0.5; touch escaped) & wait"],
        dir.path(),
        String::new(),
        Duration::from_millis(100),
        4096,
    )
    .unwrap();
    assert_eq!(r.status, "timeout");
    std::thread::sleep(Duration::from_millis(650));
    assert!(!dir.path().join("escaped").exists());
    let r = execute(
        "/bin/sh",
        &["-c", "while true; do echo error >&2; done"],
        dir.path(),
        String::new(),
        Duration::from_secs(1),
        4096,
    )
    .unwrap();
    assert_eq!(r.status, "output_limit");
    assert!(r.stderr.len() <= 4096);
}
#[test]
fn compiler_path_with_spaces_and_cpp20_are_supported() {
    let dir = tempfile::tempdir().unwrap();
    let parent = dir.path().join("compiler with spaces");
    std::fs::create_dir(&parent).unwrap();
    let path = parent.join("clang++");
    #[cfg(unix)]
    std::os::unix::fs::symlink(code_runner::detect_compiler("").unwrap(), &path).unwrap();
    let p = problem("lc-3").unwrap();
    let r=judge(&p,"acm","#include <span>\n#include <iostream>\nint main(){int a[]={0}; std::span<int> s(a); std::cout<<s[0];}",true,path.to_str().unwrap()).unwrap();
    assert_eq!(r.status, "wrong_answer");
    assert!(r.diagnostics.is_empty());
}

#[test]
fn io_protocols_and_linked_list_cycles_are_checked() {
    for (id, expected) in [
        ("lc-1", "4 9\n2 7 11 15\n"),
        ("lc-3", "\n"),
        ("lc-206", "5\n1 2 3 4 5\n"),
        ("lc-283", "5\n0 1 0 3 12\n"),
    ] {
        let p = problem(id).unwrap();
        assert_eq!(content::input(&p, &p.tests[0]), expected);
    }
    let p = problem("lc-206").unwrap();
    let r = judge(
        &p,
        "function",
        "class Solution{public:ListNode* reverseList(ListNode* h){if(h)h->next=h;return h;}};",
        true,
        "",
    )
    .unwrap();
    assert_eq!(r.status, "runtime_error");
    assert_eq!(r.cases[0].exit_code, Some(2));
    assert!(r.cases[0].stderr.contains("cyclic"));
}

#[test]
fn judge_cleans_up_temporary_sources_and_executables() {
    let p = problem("lc-3").unwrap();
    let r = judge(&p, "acm", "#include <filesystem>\n#include <iostream>\nint main(){std::cout << std::filesystem::current_path().string();}", true, "").unwrap();
    assert_eq!(r.status, "wrong_answer");
    for case in r.cases {
        assert!(case.actual.starts_with('/'));
        assert!(!std::path::Path::new(&case.actual).exists());
    }
}

#[test]
fn two_sum_worst_case_and_complexity_profile() {
    let p = problem("lc-1").unwrap();
    let worst = p.tests.iter().find(|t| t.id == "worst-case-10000").unwrap();
    assert!(!worst.is_sample);
    let nums = worst.logical_input["nums"].as_array().unwrap();
    assert_eq!(nums.len(), 10000);
    assert_eq!(worst.expected, serde_json::json!([9998, 9999]));
    assert!(content::compare(&p, worst, "9998 9999"));
    // Ascending small nonnegative numbers cannot reach the target, even paired
    // with either tail value; only the two tail values sum to 300000.
    assert!(nums[..9998]
        .iter()
        .enumerate()
        .all(|(i, v)| v.as_i64() == Some(i as i64)));
    assert_eq!(
        &nums[9998..],
        &[serde_json::json!(100000), serde_json::json!(200000)]
    );
    assert_eq!(worst.logical_input["target"], 300000);

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("profile.cpp"),
        include_str!("benchmarks/two_sum.cpp"),
    )
    .unwrap();
    let compiler = code_runner::detect_compiler("").unwrap();
    let compiled = execute(
        &compiler,
        &["-std=c++20", "-O0", "profile.cpp", "-o", "profile"],
        dir.path(),
        String::new(),
        Duration::from_secs(30),
        65536,
    )
    .unwrap();
    assert_eq!(compiled.status, "ok", "{}", compiled.stderr);
    let profile = execute(
        dir.path().join("profile").to_str().unwrap(),
        &[],
        dir.path(),
        String::new(),
        Duration::from_secs(10),
        65536,
    )
    .unwrap();
    assert_eq!(profile.status, "ok", "{}", profile.stderr);
    println!("{}", profile.stdout);
    let rows: Vec<_> = profile.stdout.lines().skip(1).collect();
    assert_eq!(rows.len(), 8);
    for (i, row) in rows.iter().enumerate() {
        let fields: Vec<_> = row.split(',').collect();
        assert_eq!(fields.len(), 5);
        let n: u64 = fields[1].parse().unwrap();
        assert_eq!(n, [1000, 2000, 4000, 10000][i / 2]);
        let operations: u64 = fields[2].parse().unwrap();
        let entries: u64 = fields[3].parse().unwrap();
        let _elapsed_ns: u64 = fields[4].parse().unwrap();
        if i % 2 == 0 {
            assert_eq!(fields[0], "double_for");
            assert_eq!(operations, n * (n - 1) / 2);
            assert_eq!(entries, 0); // Scalar local variables only; input excluded.
        } else {
            assert_eq!(fields[0], "hash");
            assert_eq!(operations, n);
            assert_eq!(entries, n - 1);
        }
    }
    // Keep a plain double-for program exercised by the actual Judge too.
    // Whether 50M checks exceed 1s depends on the machine; don't fake a TLE.
    let mut only_worst = p.clone();
    only_worst.tests = vec![worst.clone()];
    let r = judge(
        &only_worst,
        "acm",
        include_str!("solutions/lc-1-brute-acm.cpp"),
        false,
        "",
    )
    .unwrap();
    assert!(
        ["accepted", "timeout"].contains(&r.status.as_str()),
        "{}",
        r.diagnostics
    );
    println!(
        "double-for Judge: {} ({} ms)",
        r.status, r.cases[0].duration_ms
    );
}

#[test]
fn per_problem_resource_limits_are_executed_in_both_modes() {
    let allocating = r#"
        #include <thread>
        #include <chrono>
        int main() {
            volatile char* memory = new char[192 * 1024 * 1024];
            for (int i = 0; i < 192 * 1024 * 1024; i += 4096) memory[i] = 1;
            std::this_thread::sleep_for(std::chrono::milliseconds(300));
            delete[] memory;
        }
    "#;
    let allocating_function = r#"
        #include <thread>
        #include <chrono>
        class Solution { public: int lengthOfLongestSubstring(string) {
            volatile char* memory = new char[192 * 1024 * 1024];
            for (int i = 0; i < 192 * 1024 * 1024; i += 4096) memory[i] = 1;
            std::this_thread::sleep_for(std::chrono::milliseconds(300));
            delete[] memory;
            return 0;
        }};
    "#;
    for (mode, code) in [("acm", allocating), ("function", allocating_function)] {
        let mut p = problem("lc-3").unwrap();
        p.limits.memory_limit_mb = 32;
        let r = judge(&p, mode, code, true, "").unwrap();
        assert_eq!(r.status, "memory_limit", "{mode}");
        assert_eq!(r.cases.len(), 1);
        assert!(r.cases[0].peak_memory_bytes.unwrap() > 32 * 1024 * 1024);
        assert_eq!(r.limits.memory_limit_mb, 32);
    }
    let mut p = problem("lc-3").unwrap();
    p.limits.time_limit_ms = 50;
    let r = judge(&p, "acm", "#include <thread>\n#include <chrono>\nint main(){std::this_thread::sleep_for(std::chrono::seconds(2));}", true, "").unwrap();
    assert_eq!(r.status, "timeout");
    assert_eq!(r.limits.time_limit_ms, 50);
    p.limits.output_limit_kb = 1;
    p.limits.time_limit_ms = 1000;
    let r = judge(
        &p,
        "acm",
        "#include <iostream>\nint main(){for(int i=0;i<2000;++i)std::cout << 'x';}",
        true,
        "",
    )
    .unwrap();
    assert_eq!(r.status, "output_limit");
    assert!(r.cases[0].actual.len() <= 1024);
    p.limits.memory_limit_mb = 0;
    assert!(judge(&p, "acm", "int main(){}", true, "").is_err());
}

#[test]
fn every_problem_has_sourced_complexity_policy_and_maximum_size_test() {
    for meta in content::problems() {
        let p = problem(&meta.id).unwrap();
        content::validate(&p).unwrap_or_else(|e| panic!("{}: {e}", p.id));
        assert!(!p.complexity.time.is_empty(), "{}", p.id);
        assert!(!p.complexity.space.is_empty(), "{}", p.id);
        assert!(!p.complexity.official_requirement.is_empty(), "{}", p.id);
        assert!(!p.complexity.notes.is_empty(), "{}", p.id);
        let en =
            p.en.as_ref()
                .unwrap_or_else(|| panic!("{} needs an en block", p.id));
        assert_eq!(en.tags.len(), p.tags.len(), "{}", p.id);
        assert_eq!(en.constraints.len(), p.constraints.len(), "{}", p.id);
        let english = [
            &en.title,
            &en.statement,
            &en.acm_io_spec,
            &en.complexity.time,
            &en.complexity.space,
            &en.complexity.official_requirement,
            &en.complexity.notes,
        ];
        for text in english.into_iter().chain(&en.tags).chain(&en.constraints) {
            assert!(!text.trim().is_empty(), "{}", p.id);
            assert!(
                !text.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)),
                "{}: {text}",
                p.id
            );
        }
        assert!(
            p.complexity
                .source_url
                .starts_with("https://leetcode.com/problems/"),
            "{}",
            p.id
        );
        // Budgets are chosen per problem from input scale and measured runs.
        assert!((500..=5000).contains(&p.limits.time_limit_ms), "{}", p.id);
        assert!((64..=512).contains(&p.limits.memory_limit_mb), "{}", p.id);
        assert!((16..=1024).contains(&p.limits.output_limit_kb), "{}", p.id);
        assert!(
            p.tests.iter().any(|t| !t.is_sample
                && (t.id.starts_with("max-size-") || t.id.starts_with("worst-case-"))),
            "{} needs a max-size or worst-case test",
            p.id
        );
    }
}

#[test]
fn background_connections_share_migrated_data_and_enforce_foreign_keys() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested/prep.sqlite3");
    let c = db::open(&path).unwrap();
    db::save(&c, "lc-1", "acm", "shared draft").unwrap();
    // Background tasks use connect(): no schema work, same data, WAL allows both.
    let other = db::connect(&path).unwrap();
    assert_eq!(db::load(&other).unwrap()[0].code, "shared draft");
    let fk: i32 = other
        .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
        .unwrap();
    assert_eq!(fk, 1);
}

#[test]
fn judge_result_and_review_run_commit_or_rollback_together() {
    let dir = tempfile::tempdir().unwrap();
    let c = db::open(&dir.path().join("atomic.sqlite3")).unwrap();
    db::save(&c, "lc-1", "function", &solution("lc-1", "function")).unwrap();
    // Real compilation and execution; a trigger then forces only persistence to fail.
    let p = problem("lc-1").unwrap();
    let result = judge(&p, "function", &solution("lc-1", "function"), false, "").unwrap();
    assert_eq!(result.status, "accepted");
    c.execute_batch("CREATE TRIGGER fail_result BEFORE UPDATE ON coding_progress BEGIN SELECT RAISE(FAIL,'fixture write failure'); END;").unwrap();
    assert!(db::record_submission(&c, "lc-1", "function", &result).is_err());
    assert_eq!(
        c.query_row("SELECT count(*) FROM judge_runs", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        0
    );
    assert!(db::load(&c).unwrap()[0].last_result.is_none());
    c.execute_batch("DROP TRIGGER fail_result").unwrap();
    db::record_submission(&c, "lc-1", "function", &result).unwrap();
    assert_eq!(
        c.query_row("SELECT count(*) FROM judge_runs", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        1
    );
    assert_eq!(db::load(&c).unwrap()[0].status, "completed");
}
