# Review tab

The Review tab (between System Design and Settings) answers three questions: what did I do, what should I fix, and when should I revisit it. It is built from local data only, so it works offline and costs nothing. The tab label shows how many reviews are due today.

## Data

`src-tauri/src/review.rs` builds one `review_overview` response:

- **Activity**: every full LeetCode submission (`judge_runs`, logged by `run_judge` when `samples` is false), every P1 session with at least one answer (timed at its last answer), and every P2 session with an answer (timed at session start, because design turns carry no timestamps).
- **Timed practice**: P1/P2 mock clocks and persisted LeetCode countdowns, attributed to the day the timer started. LC clocks are shared across Function/ACM modes for the same problem; pause excludes idle time, expiry caps elapsed time at the selected budget, and reset archives the elapsed attempt before returning to an unstarted countdown. Switching the duration or selecting Off also archives a started attempt. Navigation, relaunch and sleep preserve a running wall-clock deadline; pause before leaving to exclude that time. Earlier LC timer time cannot be backfilled.
- **Costs**: all `api_usage` rows, preserving `usd: null` for unknown estimates. The day card shows Unknown when all requests have unknown cost, and a known subtotal plus an explicit unknown-request count for mixed data. A reported zero remains zero.
- **Weak points**, one per concept:
  - `lc:<problem>`: every non-accepted submission is weak, and an accepted one is good. Compilation errors and compilation timeouts are ignored.
  - `p1:<topic>`: a finished session is weak when any assessed dimension is `partial` or `incorrect`. It is good when at least one dimension is `good` and none is weak. `missing` and `uncertain` never create a weakness or advance review progress; an entirely unassessed session contributes no outcome. Sessions whose answers were all AI-assisted are ignored.
  - `p2:<dimension>`: `good` is good and `partial`/`incorrect` is weak. `missing` and `uncertain` are ignored, since they usually mean the stage was not discussed.

P1 evidence uses the exact `evidence` text of each weak feedback dimension and links to its original session. It never substitutes the latest candidate answer or an improvement suggestion for the evidence of an earlier mistake. P2 evidence retains the feedback evidence/advice, and LC evidence uses actual judge results. Evidence appears newest first. AI-assisted context is explicitly labeled and never counted as independent evidence.

The frontend groups by the user's local calendar day.

P1/P2 feedback displays a bilingual "Added to Review" status once saved feedback is available, including reopened histories. This confirms the local record is available in Review; it does not claim that every session creates a weak point. Unassessed/AI-only evidence policies still apply.

## Schedule

`schedule()` walks a concept's outcomes in time order:

- A weak outcome sets the stage to 0. The next review is due one day later.
- A good outcome at least 20 hours after the previous counted outcome advances the stage. Reviews are then due 3 days and 7 days later.
- At stage 3 the point is resolved and no longer scheduled.

Status is `open` at stage 0, `improving` at stages 1–2, and `resolved` at stage 3.

"Not a weak point" dismisses a concept until new weak evidence arrives.

## Limits

- Judge history starts with this version; earlier submissions only exist as the latest result per problem and are not backfilled.
- P2 activity uses the session start time.
- Weak points are grouped by problem, topic and design dimension, not by finer concepts inside an answer. Doing that would need an AI call per session.

## Review navigation and regression checks

Explicit review navigation takes precedence over default session restoration on first mount. Both P1 and P2 check a navigation generation before fetching/applying a session, invalidate older requests, and ignore results after unmount. Navigation waits for queued draft saves and reports save failures rather than silently discarding a draft. Loading the overview can still refresh the topic lists and settings without selecting an unrelated recent session.

Run `npm run test:review-ui` for asynchronous navigation and unknown/zero/mixed cost-summary regressions. Run `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib review::` for scheduling, P1 unassessed/AI-only handling, exact evidence provenance and nullable cost persistence. These tests use authored local histories and deferred UI requests; they do not pretend to compile, judge or call an AI provider successfully.

### Fix acceptance (2026-10-16)

- The four reported defects are fixed: P1 missing/uncertain handling, exact feedback evidence, first-mount/stale navigation, and unknown-cost rendering.
- Rust regression suite: 90 passed, one intentionally ignored Keychain round-trip. This includes P0 real C++ reference judging in both modes and the P1/P2/session-tool suites. Review-specific coverage includes 11 tests, with the four new authored-history/cost regressions.
- Frontend async-navigation and cost-summary tests: four passed (`npm run test:review-ui`).
- Native UI acceptance used a separate `local.swe.interview-prep.review-qa` profile containing only authored fixtures. The P1 and P2 pages were first opened from older Review evidence while newer active sessions existed; both kept the requested finished session. The P1 evidence card showed the earlier incorrect claim rather than the later correct answer. Unknown-only costs displayed Unknown, and mixed zero/nonzero/unknown costs displayed a known $0.25 subtotal plus an unknown indicator/count, in Chinese and English.
- No provider request was made and no personal interview history, key or configured provider/model was changed for acceptance. No historical feedback was rewritten; review results are derived from the preserved histories.

The QA profile and fixture usage rows are testing data, not real provider usage or proof of model output quality. `missing` remains insufficient independent evidence, including both unasked dimensions and an explicit unknown answer; it does not independently establish a technical mistake.

The final TypeScript/Vite build and normal native debug `.app` build passed. Vite retains the existing large-chunk advisory. The runnable bundle is `src-tauri/target/review-fixed/SWE Interview Prep.app` (Git-ignored); source changes remain uncommitted. Validation details are in `review/review-fixes-acceptance.json`. Machine-local logs are `/tmp/intv-review-fixed-regression.log`, `/tmp/intv-review-ui-tests-final.log`, `/tmp/intv-review-fixed-build-final.log` and `/tmp/intv-review-fixed-desktop-final.log`.

## LC timing follow-up

Run `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib coding_timer::` for restart persistence, pause/resume, per-problem isolation, archived resets, capped expiry, input validation and combined LC/P1 timing with start-day attribution. Timers default to Off, offer 10/20/30 minutes and only start explicitly. Stored durations are countdown usage, not an estimate of focused work, and do not establish solution complexity.

Follow-up acceptance (2026-10-16): 23 targeted tests passed (three coding-timer, 11 Review, five session-tool and four frontend regressions). TypeScript/Vite and native debug app builds passed. Computer-use checks in the isolated QA profile verified explicit start, problem switching, paused restart recovery, reset history, Review minutes and bilingual feedback/timer-only messages. No paid API request or personal-history change was made. The runnable bundle is `src-tauri/target/review-followups/SWE Interview Prep.app`; see `review/review-followups-acceptance.json`.
