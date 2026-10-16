# Preview acceptance evidence

Version: `0.1.0-preview.1`. Local checks performed on macOS 27.0.1 / arm64.

- Production frontend build and 11 frontend helper/Markdown tests passed.
- 114 offline Rust tests passed, including all 100 LeetCode reference solutions
  compiled and judged in both modes, resource regressions and local HTTP fixtures.
- One actual non-debug Release credential integration test passed with synthetic
  provider keys, requiring initialization and keeping keys out of SQLite.
- Four release-tool tests passed (version/tag, signing/identity, private paths,
  secret-pattern detection). Repository/history audit passed before upload.
- Optimized arm64 app and DMG built with ad-hoc signing. `codesign --verify --deep
  --strict` and architecture checks passed. Bundle scan found no credential pattern,
  credential/data file, or embedded checkout `.env` path.
- Native Release QA app used a separate identifier
  `local.swe.interview-prep.release-qa`, and was copied to `/tmp` outside the checkout.
  Cold start displayed missing keys despite the development checkout having its
  own configuration. Settings displayed the runtime app-config `.env` path.
  A disposable fixture was saved with mode 0600 and read after a process restart.
- Native Release QA actually compiled Two Sum with `/usr/bin/clang++` in Function
  and ACM modes. Both passed all six semantic cases, including 10,000 elements.
- Release packaging collected 543 dependency entries with no missing notice text,
  supplemental pinned upstream/SPDX license texts and unmodified MPL crate sources.

No paid model/transcription requests were made for release preparation. Personal
credentials/history were not migrated or modified. The UI test is a relocated
local bundle test, not a quarantined download on a clean Mac. Apple notarization,
Developer ID signing, Intel and additional macOS versions are not validated.
GitHub workflow runs provide the separate evidence for hosted CI and draft assets.
