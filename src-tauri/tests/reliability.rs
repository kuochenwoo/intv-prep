//! Real compiler/process regressions for short-lived and concurrent judged programs.
#![cfg(target_os = "macos")]
use std::{path::Path, time::Duration};
use swe_interview_prep::code_runner::{execute, execute_with_memory};
fn compile(dir: &Path) -> std::path::PathBuf {
    let source = dir.join("probe.cpp");
    let binary = dir.join("probe");
    std::fs::write(&source,r#"#include <cstdio>
#include <cstdlib>
#include <unistd.h>
int main(int argc,char**argv){
 if(argc>1){size_t n=32*1024*1024;volatile char* p=new char[n];for(size_t i=0;i<n;i+=4096)p[i]=1;delete[] p;}
 puts("real execution");return 0;
}"#).unwrap();
    let r = execute(
        "/usr/bin/clang++",
        &[
            "-std=c++20",
            "-O2",
            source.to_str().unwrap(),
            "-o",
            binary.to_str().unwrap(),
        ],
        dir,
        String::new(),
        Duration::from_secs(30),
        65536,
    )
    .unwrap();
    assert_eq!(r.status, "ok", "{}", r.stderr);
    binary
}
#[test]
fn short_lived_processes_keep_terminal_rss_and_exit_status_under_concurrency() {
    let dir = tempfile::tempdir().unwrap();
    let binary = compile(dir.path());
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let binary = &binary;
            let dir = dir.path();
            scope.spawn(move || {
                for run in 0..100 {
                    let r = execute_with_memory(
                        binary.to_str().unwrap(),
                        &[],
                        dir,
                        String::new(),
                        Duration::from_secs(3),
                        4096,
                        Some(64 * 1024 * 1024),
                    )
                    .unwrap_or_else(|e| panic!("run {run}: {e}"));
                    assert_eq!(r.status, "ok", "run {run}");
                    assert_eq!(r.exit_code, Some(0));
                    assert_eq!(r.stdout.trim(), "real execution");
                    assert!(
                        r.peak_memory_bytes.unwrap_or(0) > 0,
                        "run {run}: no terminal RSS"
                    );
                }
            });
        }
    });
}
#[test]
fn terminal_rss_detects_a_brief_allocation_without_raising_local_budgets() {
    let dir = tempfile::tempdir().unwrap();
    let binary = compile(dir.path());
    for _ in 0..12 {
        let r = execute_with_memory(
            binary.to_str().unwrap(),
            &["burst"],
            dir.path(),
            String::new(),
            Duration::from_secs(3),
            4096,
            Some(16 * 1024 * 1024),
        )
        .unwrap();
        assert_eq!(r.status, "memory_limit");
        assert!(r.peak_memory_bytes.unwrap() > 16 * 1024 * 1024);
    }
}
#[test]
fn parent_heap_is_not_charged_to_the_executed_child() {
    let dir = tempfile::tempdir().unwrap();
    let binary = compile(dir.path());
    let mut parent = vec![0u8; 192 * 1024 * 1024];
    for i in (0..parent.len()).step_by(4096) {
        parent[i] = 1;
    }
    let r = execute_with_memory(
        binary.to_str().unwrap(),
        &[],
        dir.path(),
        String::new(),
        Duration::from_secs(3),
        4096,
        Some(64 * 1024 * 1024),
    )
    .unwrap();
    std::hint::black_box(&parent);
    assert_eq!(r.status, "ok");
    assert!(r.peak_memory_bytes.unwrap() < 64 * 1024 * 1024);
}
