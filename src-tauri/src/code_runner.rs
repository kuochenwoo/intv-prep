use crate::content::{self, Problem};
use serde::Serialize;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessResult {
    pub status: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
}
fn kill_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    let _ = child.kill();
}
fn capture(mut pipe: impl Read, count: Arc<AtomicUsize>, cap: usize) -> String {
    let mut kept = Vec::new();
    let mut buf = [0u8; 4096];
    while let Ok(n) = pipe.read(&mut buf) {
        if n == 0 {
            break;
        }
        let old = count.fetch_add(n, Ordering::SeqCst);
        let room = cap.saturating_sub(old);
        kept.extend_from_slice(&buf[..n.min(room)]);
        if old + n > cap {
            break;
        }
    }
    String::from_utf8_lossy(&kept).into_owned()
}
pub fn execute(
    program: &str,
    args: &[&str],
    cwd: &std::path::Path,
    input: String,
    timeout: Duration,
    cap: usize,
) -> Result<ProcessResult, String> {
    let mut cmd = Command::new(program);
    cmd.args(args)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    unsafe {
        cmd.pre_exec(|| {
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            for (resource, limit) in [
                (libc::RLIMIT_CORE, 0),
                (libc::RLIMIT_FSIZE, 32 * 1024 * 1024),
                (libc::RLIMIT_NOFILE, 128),
                (libc::RLIMIT_CPU, 30),
            ] {
                let r = libc::rlimit {
                    rlim_cur: limit,
                    rlim_max: limit,
                };
                if libc::setrlimit(resource, &r) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
    let start = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("无法执行 {program}: {e}"))?;
    let mut stdin = child.stdin.take().unwrap();
    let writer = thread::spawn(move || {
        let _ = stdin.write_all(input.as_bytes());
    });
    let count = Arc::new(AtomicUsize::new(0));
    let out = child.stdout.take().unwrap();
    let err = child.stderr.take().unwrap();
    let c = count.clone();
    let stdout = thread::spawn(move || capture(out, c, cap));
    let c = count.clone();
    let stderr = thread::spawn(move || capture(err, c, cap));
    let status = loop {
        if count.load(Ordering::SeqCst) > cap {
            break "output_limit";
        }
        if start.elapsed() > timeout {
            break "timeout";
        }
        match child.try_wait() {
            Ok(Some(s)) => break if s.success() { "ok" } else { "runtime_error" },
            Ok(None) => thread::sleep(Duration::from_millis(5)),
            Err(_) => break "runtime_error",
        }
    };
    kill_group(&mut child);
    let exit_code = child.wait().ok().and_then(|s| s.code());
    let _ = writer.join();
    let out = stdout.join().unwrap_or_default();
    let err = stderr.join().unwrap_or_default();
    let status = if count.load(Ordering::SeqCst) > cap {
        "output_limit"
    } else {
        status
    };
    Ok(ProcessResult {
        status: status.into(),
        exit_code,
        stdout: out,
        stderr: err,
        duration_ms: start.elapsed().as_millis(),
    })
}
pub fn detect_compiler(requested: &str) -> Result<String, String> {
    let candidates: Vec<&str> = if requested.trim().is_empty() {
        vec!["clang++", "g++"]
    } else {
        vec![requested.trim()]
    };
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for name in candidates {
        // Resolve before changing the process working directory; preserve full paths with spaces.
        let resolved = if name.contains('/') {
            // Keep the clang++ symlink name: canonicalizing to clang changes linker behavior.
            std::env::current_dir()
                .ok()
                .map(|cwd| cwd.join(name))
                .filter(|p| p.is_file())
        } else {
            std::env::var_os("PATH").and_then(|paths| {
                std::env::split_paths(&paths)
                    .map(|p| p.join(name))
                    .find(|p| p.is_file())
            })
        };
        if let Some(path) = resolved {
            let path = path.to_string_lossy().into_owned();
            if let Ok(r) = execute(
                &path,
                &["--version"],
                temp.path(),
                String::new(),
                Duration::from_secs(3),
                65536,
            ) {
                if r.status == "ok" {
                    return Ok(path);
                }
            }
        }
    }
    Err("未找到可用的 clang++ / g++，请指定编译器可执行文件的完整路径（不含参数）".into())
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseResult {
    pub id: String,
    pub status: String,
    pub exit_code: Option<i32>,
    pub input: String,
    pub expected: serde_json::Value,
    pub actual: String,
    pub stderr: String,
    pub duration_ms: u128,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeResult {
    pub status: String,
    pub passed: usize,
    pub total: usize,
    pub compiler: String,
    pub diagnostics: String,
    pub cases: Vec<CaseResult>,
}
pub fn judge(
    p: &Problem,
    mode: &str,
    code: &str,
    samples: bool,
    compiler: &str,
) -> Result<JudgeResult, String> {
    if code.len() > 256 * 1024 {
        return Err("代码长度超过 256 KiB".into());
    }
    let source = content::source(p, mode, code)?;
    let compiler = detect_compiler(compiler)?;
    let tmp = tempfile::tempdir().map_err(|e| e.to_string())?;
    std::fs::write(tmp.path().join("solution.cpp"), source).map_err(|e| e.to_string())?;
    let tests: Vec<_> = p.tests.iter().filter(|t| !samples || t.is_sample).collect();
    let mut result = JudgeResult {
        status: "accepted".into(),
        passed: 0,
        total: tests.len(),
        compiler: compiler.clone(),
        diagnostics: String::new(),
        cases: vec![],
    };
    let compile = execute(
        &compiler,
        &["-std=c++20", "-O0", "solution.cpp", "-o", "solution"],
        tmp.path(),
        String::new(),
        Duration::from_secs(30),
        256 * 1024,
    )?;
    if compile.status != "ok" {
        result.status = if compile.status == "runtime_error" {
            "compile_error".into()
        } else {
            format!("compile_{}", compile.status)
        };
        result.diagnostics = format!("{}{}", compile.stdout, compile.stderr);
        return Ok(result);
    }
    let executable = tmp.path().join("solution").to_string_lossy().into_owned();
    for t in tests {
        let input = content::input(p, t);
        let r = execute(
            &executable,
            &[],
            tmp.path(),
            input.clone(),
            Duration::from_millis(1000),
            64 * 1024,
        )?;
        let status = if r.status != "ok" {
            r.status.clone()
        } else if content::compare(p, t, &r.stdout) {
            "accepted".into()
        } else {
            "wrong_answer".into()
        };
        if status == "accepted" {
            result.passed += 1;
        } else if result.status == "accepted" {
            result.status = status.clone();
        }
        let stop = status == "timeout" || status == "output_limit";
        result.cases.push(CaseResult {
            id: t.id.clone(),
            status,
            exit_code: r.exit_code,
            input,
            expected: t.expected.clone(),
            actual: r.stdout,
            stderr: r.stderr,
            duration_ms: r.duration_ms,
        });
        if stop {
            break;
        }
    }
    Ok(result)
}
