# Reliability follow-up — 2026-10-15

## Outcome

The final complete backend run passed **55 tests**, with one intentionally ignored OS Keychain round-trip. It includes all 100 LC reference solutions in Function and ACM modes, real compiler/resource regressions, P1/P2 storage recovery, local HTTP/streaming fixtures, and the new memory-monitor tests. No problem limits were raised and no successful compilation, execution, judging or provider response was fabricated.

DeepSeek completed real P1 and P2 interviews and real authentication-failure recovery during this run. This is a point-in-time acceptance result, not a guarantee against future provider overload, 503s or network outages. The app's configured provider/model and stored keys were not changed by these probes.

## Judge investigation and changes

The historical intermittent `memory_monitor_error` did not reproduce in the initial full-library baseline (72.36 seconds). Its original cause therefore remains unconfirmed. The review did identify an exit-accounting gap: live task information can disappear at exit, and a process can finish between RSS samples.

On macOS the runner now collects exit status and the kernel's terminal RSS high-water mark together with `wait4`, retaining that result rather than reaping the child twice. It polls exit before requesting live task information, and falls back from `proc_pidinfo` to `proc_pid_rusage` if the live task-info call is unavailable. Live samples still run at the existing interval. Persistent monitoring failure still kills the process group and returns an error; the change does not silently disable resource checks.

Terminal RSS is merged with live RSS observations and checked against the same problem budget, including a brief allocation followed immediately by exit. On Darwin the kernel populates `ru_maxrss` from `resident_size_max`; see [Apple's XNU resource-accounting implementation](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_resource.c). Linux retains its existing `/proc` monitoring path; this run does not certify native Linux behavior. RSS remains a process resource metric rather than algorithmic auxiliary space, an aggregate process-tree memory sandbox, or a Big-O proof.

New regressions compile and run real C++ programs:

- Four workers each execute 100 fast-exiting programs, requiring correct exit codes, actual stdout, and a nonzero terminal RSS observation.
- Twelve brief 32 MiB allocations must exceed the unchanged 16 MiB test budget.
- A touched 192 MiB parent heap must not be charged to a small child with a 64 MiB budget.
- A real sleeping child is stopped and reaped when live samples are deliberately unavailable. Only the failure of the OS sampler is injected; no successful process or judge outcome is mocked.

The common full-library test now records the problem ID, mode and judge error instead of losing that context to a worker `unwrap` panic. If the historical failure recurs, these diagnostics narrow the investigation. UI memory text now says “observed peak” and explains the macOS terminal measurement.

The final rebuilt macOS app was checked through native computer use. Running the existing Two Sum Function-mode draft with `/usr/bin/clang++` returned Accepted, 2/2 samples, and an observed peak of 1.7 MiB for each case. The dark English layout kept the problem, editor, run controls and result readable; no draft or appearance preference was changed. Frontend and native app builds both passed. The full-library regression executed 989 logical cases in each mode (1,978 actual case executions).

## Real DeepSeek acceptance

All tests used the existing provider-specific `.env` key through the normal HTTPS adapter. The file is still Git-ignored with mode `0600`. No Keychain migration or credential write was performed.

1. A bounded metadata-only connectivity probe returned HTTP 200, a real completion ID and numeric usage (7 prompt / 2 completion / 9 total tokens).
2. P1 generated and cached a question, completed two actual streamed follow-ups, produced five-dimension feedback, reopened its database, and reused the cached question offline.
3. P2 generated a question, discussed Requirements, Estimation, Architecture, Deep Dive and Trade-offs, produced eight-dimension feedback, reopened its database, and reused the question offline. Model stage suggestions remained advisory.
4. A recovery probe sent an intentionally invalid synthetic credential to the real endpoint and required HTTP 401/403. It reopened the database with the candidate answer and pending state intact, retried with the existing real key, verified exactly one candidate submission, and completed real five-dimension feedback. Stored credentials were never replaced.

Early probes in this session failed at connection establishment before an HTTP response. Later calls succeeded without changing the TLS configuration, disabling certificate validation or modifying proxy settings; the early failure's cause is unconfirmed. A temporary automatic-approval service quota failure also prevented one diagnostic action; it was not executed, and approval later succeeded normally. No approval restriction was bypassed.

## Reproduce

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features
# Real API usage, isolated test databases; reads the existing .env key:
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-flash \
  cargo run --manifest-path src-tauri/Cargo.toml --no-default-features \
  --example live_deep_dive -- /tmp/intv-reliability-deepseek.sqlite3
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-flash \
  INTERVIEW_DB_PATH=/tmp/intv-reliability-deepseek-p2.sqlite3 \
  cargo run --manifest-path src-tauri/Cargo.toml --no-default-features \
  --example live_system_design
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features \
  --example live_deepseek_recovery
```

The recovery probe requires the first P1 probe's actual cached question. The P2 live example defaults to Claude unless the provider/model environment variables above are supplied. These examples use scripted practice answers, not evidence of a user's ability.

Evidence logs for this run are `/tmp/intv-reliability-all-tests.log`, `/tmp/intv-reliability-deepseek-latest.log`, `/tmp/intv-reliability-deepseek-live.log`, `/tmp/intv-reliability-deepseek-p2.log` and `/tmp/intv-reliability-deepseek-recovery.log`. Frontend/native build logs are `/tmp/intv-reliability-frontend.log` and `/tmp/intv-reliability-desktop.log`.
