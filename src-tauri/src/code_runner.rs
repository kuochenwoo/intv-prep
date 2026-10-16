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
    pub peak_memory_bytes: Option<u64>,
}

// Sample the executable's resident footprint, including the runtime and harness.
// This is not auxiliary-space analysis or an OS memory sandbox.
#[cfg(target_os = "macos")]
fn resident_bytes(pid: u32) -> Option<u64> {
    let mut info = std::mem::MaybeUninit::<libc::proc_taskinfo>::zeroed();
    let size = std::mem::size_of::<libc::proc_taskinfo>() as i32;
    let read = unsafe {
        libc::proc_pidinfo(
            pid as i32,
            libc::PROC_PIDTASKINFO,
            0,
            info.as_mut_ptr().cast(),
            size,
        )
    };
    if read == size {
        Some(unsafe { info.assume_init() }.pti_resident_size)
    } else {
        // The task-info endpoint can be unavailable during exec/exit transitions.
        // Ask the independent resource-usage endpoint before reporting monitoring loss.
        let mut usage = std::mem::MaybeUninit::<libc::rusage_info_v2>::zeroed();
        let result = unsafe {
            libc::proc_pid_rusage(pid as i32, libc::RUSAGE_INFO_V2, usage.as_mut_ptr().cast())
        };
        (result == 0).then(|| unsafe { usage.assume_init() }.ri_resident_size)
    }
}
#[cfg(target_os = "linux")]
fn resident_bytes(pid: u32) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    // VmHWM also retains peaks that occurred between samples while alive.
    status
        .lines()
        .filter(|s| s.starts_with("VmRSS:") || s.starts_with("VmHWM:"))
        .filter_map(|s| s.split_whitespace().nth(1)?.parse::<u64>().ok())
        .max()
        .map(|kb| kb * 1024)
}
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn resident_bytes(_pid: u32) -> Option<u64> {
    None
}
// On macOS, collect terminal RSS and exit status together. A reaped task no
// longer supports proc_pidinfo, and a fast executable can finish between samples.
// Linux retains its existing /proc sampler; terminal RSS semantics and units are
// platform-specific and have not been validated for that platform here.
#[derive(Default)]
struct ProcessWait {
    status: Option<std::process::ExitStatus>,
    peak: Option<u64>,
}
impl ProcessWait {
    fn poll(
        &mut self,
        child: &mut std::process::Child,
        blocking: bool,
    ) -> std::io::Result<Option<std::process::ExitStatus>> {
        if self.status.is_some() {
            return Ok(self.status);
        }
        #[cfg(target_os = "macos")]
        {
            use std::os::unix::process::ExitStatusExt;
            loop {
                let mut status = 0;
                let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
                let result = unsafe {
                    libc::wait4(
                        child.id() as i32,
                        &mut status,
                        if blocking { 0 } else { libc::WNOHANG },
                        usage.as_mut_ptr(),
                    )
                };
                if result == 0 {
                    return Ok(None);
                }
                if result < 0 {
                    let error = std::io::Error::last_os_error();
                    if error.kind() == std::io::ErrorKind::Interrupted {
                        continue;
                    }
                    return Err(error);
                }
                let bytes = unsafe { usage.assume_init() }.ru_maxrss;
                self.peak = u64::try_from(bytes).ok().filter(|n| *n > 0);
                self.status = Some(std::process::ExitStatus::from_raw(status));
                return Ok(self.status);
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.status = if blocking {
                Some(child.wait()?)
            } else {
                child.try_wait()?
            };
            Ok(self.status)
        }
    }
}
fn kill_group(child: &mut std::process::Child, exited: bool) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    if !exited {
        let _ = child.kill();
    }
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
    execute_with_memory(program, args, cwd, input, timeout, cap, None)
}
pub fn execute_with_memory(
    program: &str,
    args: &[&str],
    cwd: &std::path::Path,
    input: String,
    timeout: Duration,
    cap: usize,
    memory_limit_bytes: Option<u64>,
) -> Result<ProcessResult, String> {
    execute_monitored(
        program,
        args,
        cwd,
        input,
        timeout,
        cap,
        memory_limit_bytes,
        resident_bytes,
    )
}
fn execute_monitored(
    program: &str,
    args: &[&str],
    cwd: &std::path::Path,
    input: String,
    timeout: Duration,
    cap: usize,
    memory_limit_bytes: Option<u64>,
    mut sample: impl FnMut(u32) -> Option<u64>,
) -> Result<ProcessResult, String> {
    if memory_limit_bytes.is_some() && !cfg!(any(target_os = "macos", target_os = "linux")) {
        return Err("此平台尚未支持内存监测，无法按题目限制判题".into());
    }
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
    let mut peak_memory_bytes: Option<u64> = None;
    let mut memory_read_failures = 0;
    let mut reaper = ProcessWait::default();
    let status = loop {
        // Check for exit first: disappearing live task information is normal after exit.
        match reaper.poll(&mut child, false) {
            Ok(Some(s)) => break if s.success() { "ok" } else { "runtime_error" },
            Ok(None) => {}
            Err(_) => break "memory_monitor_error",
        }
        if let Some(limit) = memory_limit_bytes {
            match sample(child.id()) {
                Some(bytes) => {
                    memory_read_failures = 0;
                    peak_memory_bytes = Some(peak_memory_bytes.unwrap_or(0).max(bytes));
                    if bytes > limit {
                        break "memory_limit";
                    }
                }
                None => {
                    // An already-exited process may no longer expose memory data.
                    match reaper.poll(&mut child, false) {
                        Ok(Some(s)) => break if s.success() { "ok" } else { "runtime_error" },
                        Ok(None) => {
                            // On macOS task info can disappear just before the
                            // exit status becomes waitable. Allow that brief race.
                            memory_read_failures += 1;
                            if memory_read_failures >= 10 {
                                break "memory_monitor_error";
                            }
                        }
                        Err(_) => break "memory_monitor_error",
                    }
                }
            }
        }
        if count.load(Ordering::SeqCst) > cap {
            break "output_limit";
        }
        if start.elapsed() > timeout {
            break "timeout";
        }
        match reaper.poll(&mut child, false) {
            Ok(Some(s)) => break if s.success() { "ok" } else { "runtime_error" },
            Ok(None) => thread::sleep(Duration::from_millis(5)),
            Err(_) => break "runtime_error",
        }
    };
    kill_group(&mut child, reaper.status.is_some());
    let exit_code = reaper
        .poll(&mut child, true)
        .ok()
        .flatten()
        .and_then(|s| s.code());
    if let Some(bytes) = reaper.peak {
        peak_memory_bytes = Some(peak_memory_bytes.unwrap_or(0).max(bytes));
    }
    let _ = writer.join();
    let out = stdout.join().unwrap_or_default();
    let err = stderr.join().unwrap_or_default();
    if status == "memory_monitor_error" {
        return Err("无法读取运行进程的内存用量；已终止进程，未绕过内存限制".into());
    }
    let status = if count.load(Ordering::SeqCst) > cap {
        "output_limit"
    } else if memory_limit_bytes
        .zip(peak_memory_bytes)
        .is_some_and(|(limit, peak)| peak > limit)
        && ["ok", "runtime_error"].contains(&status)
    {
        "memory_limit"
    } else {
        status
    };
    Ok(ProcessResult {
        status: status.into(),
        exit_code,
        stdout: out,
        stderr: err,
        duration_ms: start.elapsed().as_millis(),
        peak_memory_bytes,
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
    pub peak_memory_bytes: Option<u64>,
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
    pub limits: content::JudgeLimits,
}
pub fn judge(
    p: &Problem,
    mode: &str,
    code: &str,
    samples: bool,
    compiler: &str,
) -> Result<JudgeResult, String> {
    let limits = &p.limits;
    if limits.time_limit_ms == 0
        || limits.time_limit_ms > 60000
        || limits.memory_limit_mb == 0
        || limits.memory_limit_mb > 4096
        || limits.output_limit_kb == 0
        || limits.output_limit_kb > 1024
    {
        return Err("无效的题目运行限制".into());
    }
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
        limits: limits.clone(),
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
        let r = execute_with_memory(
            &executable,
            &[],
            tmp.path(),
            input.clone(),
            Duration::from_millis(limits.time_limit_ms),
            limits.output_limit_kb * 1024,
            Some(limits.memory_limit_mb * 1024 * 1024),
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
        let stop = ["timeout", "output_limit", "memory_limit"].contains(&status.as_str());
        // A passed hidden test keeps only its summary: its full input and answer stay in
        // the backend and are not stored with every submission. Failing cases keep a
        // bounded excerpt so the result stays small.
        let detail = t.is_sample || status != "accepted";
        result.cases.push(CaseResult {
            id: t.id.clone(),
            status,
            exit_code: r.exit_code,
            input: if detail { excerpt(input) } else { String::new() },
            expected: if detail {
                bounded(&t.expected)
            } else {
                serde_json::Value::Null
            },
            actual: if detail { excerpt(r.stdout) } else { String::new() },
            stderr: excerpt(r.stderr),
            duration_ms: r.duration_ms,
            peak_memory_bytes: r.peak_memory_bytes,
        });
        if stop {
            break;
        }
    }
    Ok(result)
}

/// Longest case text kept in a result; larger inputs are cut with a marker.
const EXCERPT_BYTES: usize = 4096;

pub(crate) fn excerpt(text: String) -> String {
    if text.len() <= EXCERPT_BYTES {
        return text;
    }
    let mut end = EXCERPT_BYTES;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n… ({} bytes total)", &text[..end], text.len())
}

/// Expected values are shown as JSON; a huge one becomes a cut string instead.
pub(crate) fn bounded(value: &serde_json::Value) -> serde_json::Value {
    let text = value.to_string();
    if text.len() <= EXCERPT_BYTES {
        value.clone()
    } else {
        serde_json::Value::String(excerpt(text))
    }
}

#[cfg(test)]
mod excerpt_tests {
    use super::*;
    #[test]
    fn long_case_text_is_cut_on_a_char_boundary() {
        assert_eq!(excerpt("abc".into()), "abc");
        let long = "数".repeat(EXCERPT_BYTES);
        let cut = excerpt(long.clone());
        assert!(cut.len() < EXCERPT_BYTES + 40);
        assert!(cut.ends_with(&format!("({} bytes total)", long.len())));
        let big = serde_json::json!(vec![1; 5000]);
        assert!(bounded(&big).is_string());
        assert_eq!(bounded(&serde_json::json!([1, 2])), serde_json::json!([1, 2]));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod monitor_failure_tests {
    use super::*;
    #[test]
    fn unavailable_live_samples_stop_a_real_process_instead_of_bypassing_limits() {
        // Only OS sample failure is injected. The child really runs and must be killed;
        // no compilation, execution result, RSS value or judge success is fabricated.
        let dir = tempfile::tempdir().unwrap();
        let mut pid = 0;
        let result = execute_monitored(
            "/bin/sleep",
            &["5"],
            dir.path(),
            String::new(),
            Duration::from_secs(2),
            4096,
            Some(64 * 1024 * 1024),
            |child| {
                pid = child;
                None
            },
        );
        assert!(result.unwrap_err().contains("未绕过内存限制"));
        assert!(pid > 0);
        assert_eq!(
            unsafe { libc::kill(pid as i32, 0) },
            -1,
            "child must be reaped"
        );
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH)
        );
    }
}
