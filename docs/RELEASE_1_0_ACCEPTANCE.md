# 1.0.0 release acceptance — 2026-10-17

## Scope

Restrained question-card typography and surfaces, canonical stable-version release support, and final offline/native verification. Original interview content, judging budgets, provider choices and production application identity remain unchanged. Distribution stays ad-hoc signed, macOS 13+ arm64, and not Apple-notarized.

## Automated checks

- 128 Rust library/integration tests passed, including real compilation/execution of all 100 LeetCode reference solutions in Function and ACM modes, resource-limit regressions, storage recovery, request handling and assistance guards.
- The separate optimized Release credential test passed with synthetic keys and an isolated temporary directory.
- 16 frontend helper/Markdown/navigation tests and 13 release-tool tests passed. New release checks cover stable/prerelease classification, trusted merged-PR gating, published-release kind matching, preview-to-stable updates and refusing prerelease promotion after a stable feed.
- The updater test used a newly generated disposable signing identity and real signature verification, rejecting altered archive bytes and a mismatched version.
- Production frontend build, version consistency, workflow actionlint, repository/private-history audit and whitespace checks passed. Vite retains its existing large-chunk warning.

## Native Release UI checks

A real optimized app was built with a separate QA identifier, `local.swe.interview-prep.release10-qa`, and verified with `codesign --verify --deep --strict`. Its Info.plist and Settings reported 1.0.0. Personal application data and provider settings were not used or changed.

- Cold start loaded all 100 LC problems. The revised title/body hierarchy, restrained complexity panel and selection states rendered correctly.
- A hash-map Two Sum solution was typed through computer use, saved, compiled and accepted 6/6 in Function mode. An independently typed full program was compiled and accepted 6/6 in ACM mode, including the 10000-element case. The QA database retained completed results for both modes.
- P1 showed the default Core filter (146/180 topics), cached question cards and readable fenced SQL. The history drawer opened and closed with Escape. Switching to English/dark mode translated controls while keeping the saved Chinese question intact.
- A 41-character offline draft was entered through the UI, displayed as saved and confirmed persisted in the QA database. The microphone remained an idle compact icon; it was not activated.
- Export opened its explicit confirmation dialog and Cancel returned to the interview without exporting.
- P2 loaded all 26 scenarios, a saved question, session-language label, explicit stage controls and a collapsible Markdown/Mermaid workspace. Finish remained disabled for the empty-answer session.
- Settings displayed the runtime QA credential path, missing-key warnings, separate interview/voice settings, update settings and version 1.0.0.

P1/P2 reading fixtures were explicitly labeled offline typography samples and written only to the isolated QA database. They were not represented as newly generated model answers or live interview acceptance. The QA usage ledger contained zero API attempts. No paid text or voice request was made.

## Publication gates

The GitHub PR must pass CI before merge. Trusted successful main CI must identify the merged version increase before creating v1.0.0. The build then verifies actual app architecture/signing, updater signatures and uploaded asset digests before publishing a stable Latest release and advancing the existing signed compatibility feed. Do not treat local checks alone as proof that GitHub publication completed.
