// Development tool for adding problems without rebuilding the app:
//   INTV_CONTENT_DIR=content/leetcode cargo run --no-default-features --example verify_problem -- content/leetcode/problems/lc-42.json ...
// For each problem file it validates the JSON, checks that the Function-mode
// template compiles against the generated harness, and judges the reference
// solutions in src-tauri/tests/solutions/<id>-{function,acm}.cpp on every test.
use swe_interview_prep::{code_runner::judge, content};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failed = false;
    for path in std::env::args().skip(1) {
        let report = verify(root, &path);
        match report {
            Ok(summary) => println!("OK   {path}: {summary}"),
            Err(e) => {
                failed = true;
                println!("FAIL {path}: {e}");
            }
        }
    }
    std::process::exit(failed as i32);
}

fn verify(root: &std::path::Path, path: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let p = content::parse(&text)?;
    content::validate(&p)?;
    let template = judge(&p, "function", &p.cpp.function_template, true, "")?;
    if template.status.starts_with("compile") {
        return Err(format!(
            "function template does not compile:\n{}",
            template.diagnostics
        ));
    }
    let acm_template = judge(&p, "acm", &p.cpp.acm_template, true, "")?;
    if acm_template.status.starts_with("compile") {
        return Err(format!(
            "ACM template does not compile:\n{}",
            acm_template.diagnostics
        ));
    }
    let mut summary = vec![format!("{} tests", p.tests.len())];
    for mode in ["function", "acm"] {
        let file = root
            .join("tests/solutions")
            .join(format!("{}-{mode}.cpp", p.id));
        let code =
            std::fs::read_to_string(&file).map_err(|_| format!("missing {}", file.display()))?;
        if code.contains("bits/stdc++") {
            return Err(format!(
                "{mode}: <bits/stdc++.h> is unavailable on macOS clang"
            ));
        }
        let r = judge(&p, mode, &code, false, "")?;
        if r.status != "accepted" || r.passed != p.tests.len() {
            let case = r.cases.iter().find(|c| c.status != "accepted");
            return Err(format!(
                "{mode}: {} ({}/{})\n{}\n{}",
                r.status,
                r.passed,
                p.tests.len(),
                r.diagnostics.chars().take(2000).collect::<String>(),
                case.map(|c| format!(
                    "case {}: input {:.300}\nexpected {:.300}\nactual {:.300}\nstderr {:.300}",
                    c.id, c.input, c.expected, c.actual, c.stderr
                ))
                .unwrap_or_default()
            ));
        }
        let slowest = r.cases.iter().max_by_key(|c| c.duration_ms).unwrap();
        let memory = r.cases.iter().filter_map(|c| c.peak_memory_bytes).max();
        summary.push(format!(
            "{mode} max {} ms ({}) peak {:.1} MiB",
            slowest.duration_ms,
            slowest.id,
            memory.unwrap_or(0) as f64 / 1048576.0
        ));
    }
    Ok(summary.join("; "))
}
