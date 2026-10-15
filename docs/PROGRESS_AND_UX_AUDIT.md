# Progress and UI/UX audit

Date: 2026-10-15. Platform: macOS Apple Silicon. Scope: developer-language cleanup, current progress verification, and computer-use testing of the desktop app. P2 remains out of scope.

## Current progress

| Area                | Current state                                                                                                                         | Evidence from this audit                                                                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P0 local Judge      | Implemented; real C++20 compilation, Function/ACM modes, persistence, and resource limits                                             | Desktop sample runs passed 2/2 in each mode; Judge and content-adapter regressions passed, subject to the intermittent failure noted below                  |
| P1 Deep Dive        | Implemented with Claude/DeepSeek adapters, streaming, caching, drafts, reference previews, explicit assisted submission, and feedback | 12 LLM/protocol tests and 10 interview/storage tests passed; one live English reference preview succeeded in the desktop UI without adding a candidate turn |
| P3 Hot 100 coverage | All 100 problems bundled, with 100 English content blocks and 200 reference programs                                                  | Isolated full-library test passed both modes on 989 logical cases, totaling 1,978 case executions                                                           |
| P2 System Design    | Not started                                                                                                                           | No implementation added during this audit                                                                                                                   |

These statements describe the current workspace, including work that was already present before this audit. They do not mean all P3 quality work is complete or that every AI response is technically correct. DeepSeek's previously reported live-service limitation was not retested here.

## Language checks

- Developer documentation and source comments are English. The separately named `README.zh-CN.md` remains an intentional Chinese documentation edition.
- All 176 UI translation pairs have matching interpolation placeholders. English strings contain no Chinese text except the language-switch button's intentional Chinese label.
- All 100 problem English blocks are present, contain no Chinese, and have tag/constraint counts matching their Chinese counterparts.
- All ten topic seeds include English content.
- Chinese statements, runtime errors, and Chinese interview prompts are intentionally retained. Runtime errors have an English translation table in `src/errors.ts`.
- Corrected an outdated i18n comment that implied problem/topic content could not change language.
- Removed Markdown hard-break whitespace that failed the whitespace check in the requirements document.

## Computer-use checks

The actual native Tauri app was inspected and operated through the computer-use tool, including accessibility-tree observations and screenshots.

- Rebuilt and opened the current 100-problem app; the previously installed bundle still contained the older four-problem UI.
- Switched between Chinese and English and checked navigation, problem titles, statements, constraints, complexity notes, I/O protocols, and result labels.
- Inspected light and dark themes for readable text, contrast, visible selection, and accessible controls.
- Searched for Two Sum in English. An unmatched query produced a clear empty state and an accessible clear button; clearing restored the full list.
- Switched between Function and ACM modes, preserving separate existing drafts. Real sample runs returned Accepted / 2 of 2 in each mode.
- Entered an invalid compiler path and verified an actionable English error; restored `/usr/bin/clang++` afterward.
- Opened English Deep Dive topics, saved history, provider settings, and cached question content.
- Requested one real English reference preview. It appeared under the question with explicit copy and submit actions; the interview remained at zero answered rounds, so previewing did not silently submit an answer or request a follow-up.
- Copied that preview into an editable, automatically saved draft. The interview stayed at zero answered rounds. Cleared the test draft and verified the original empty state was restored without submission.
- Retested the final compiler fix: a failed check displayed only the actionable error, without the earlier success message. Restored the valid compiler and switched to Chinese; its success text updated to Chinese.
- Left the rebuilt application in Chinese/light mode with the valid compiler restored and without editing the user's coding solutions.

This was a focused smoke test, not exhaustive accessibility certification or verification of every possible viewport and assistive technology. Saved interviews keep their original language when the UI language changes.

## UX corrections

- Replaced the grammatically incorrect English count “1 problems” with the neutral “1 shown”.
- Clear a previous compiler-success message before a new detection attempt, preventing contradictory success/error messages.
- Store the detected compiler path rather than a pretranslated success sentence, so its status text follows subsequent UI language changes.
- Use the existing centralized translations and Astryx components; no separate design system or hard-coded UI colors were introduced.

## Test results and remaining concern

The frontend production build and debug macOS app bundle succeeded. Across the initial run and targeted reruns, 39 automated checks passed: 12 LLM tests, five content/I/O tests, 12 P0 tests, and ten P1 tests. The isolated OS-keychain roundtrip remains intentionally ignored; this audit did not write system credentials.

The first broad run, concurrent with builds, failed in the full-library test because main-process memory sampling became unavailable. It correctly stopped the process rather than silently bypassing enforcement. An isolated rerun of the same full-library test then passed all 100 problems in both modes in 86.10 seconds. The cause has not been established: do not describe the initial full-suite run as clean or the intermittent memory-monitor issue as fixed. Further stress/race investigation remains follow-up work; no budgets or memory checks were weakened.

Reproduce the checks with:

```sh
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --test p0 every_problem_passes_reference_solutions_in_both_modes -- --nocapture
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --test p1
npm run tauri build -- --debug --bundles app
```

Local audit logs are in `/tmp/intv-language-progress-tests.log`, `/tmp/intv-hot100-audit.log`, `/tmp/intv-language-interview-tests.log`, and `/tmp/intv-language-final-desktop.log`. They are temporary artifacts, not durable application data.
