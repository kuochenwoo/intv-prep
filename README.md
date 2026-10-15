# SWE Interview Prep · P0 + P1

A local SWE interview-preparation desktop application. P0's dual-mode local Judge is complete. P1's Technical Deep Dive supports Claude / DeepSeek, caching, and resumable interviews. Claude Haiku 5.5 has passed real question generation, follow-ups, feedback, and failure recovery. DeepSeek's real multi-turn acceptance remains affected by HTTP 503 / keepalive timeouts. P2 System Design and P3's complete Hot 100 coverage are not implemented.

Project documentation and code comments are in English. The application UI, problem statements, and interview content remain in Chinese.

## Getting started

Development dependencies: Node.js 22.12+, npm, stable Rust, the platform-specific [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/), and a C++20-capable `clang++` or `g++`. On macOS, Xcode Command Line Tools provide the C++ compiler.

```sh
npm ci
npm run tauri dev
```

`npm run dev` is for frontend development only and does not provide the local Judge. Run the application in a Tauri window. There is no Node/Express application server.

A temporary Rust toolchain was installed on the development machine without changing shell configuration. Until Rust is installed normally, this machine can use:

```sh
export PATH="/tmp/intv-cargo/bin:$PATH"
export CARGO_HOME=/tmp/intv-cargo
export RUSTUP_HOME=/tmp/intv-rustup
npm run tauri dev
```

The operating system may clear `/tmp`. For ongoing development, install Rust according to the official prerequisites.

## Build and test

```sh
npm run build                         # TypeScript checks and Vite production build
npm test                              # Judge, SQLite, interview state, and HTTP recovery; no paid API calls
npm run tauri build -- --bundles app  # macOS release build: a locally unsigned .app
```

The macOS application is written to `src-tauri/target/release/bundle/macos/SWE Interview Prep.app`. Open it directly; it no longer needs Node, Rust, or the Vite server. A C++ compiler is still required. Use `npm run tauri build -- --debug --bundles app` for a debug bundle under `src-tauri/target/debug/bundle/macos/`.

Verified on macOS Apple Silicon. Process-group termination and resource limits use Unix APIs. Windows process-tree termination has not been implemented or verified; this version should not be considered Windows-ready.

Tests invoke the real local compiler, without mocking compilation, execution, or Accepted results. `src-tauri/tests/p0.rs` covers:

- All local tests for four problems in Function and ACM modes, C++20, and compiler paths containing spaces.
- Compilation errors, wrong answers, empty output, nonzero exits, crashes, infinite loops, and excessive stdout/stderr output.
- Multiple valid answers, invalid indices, malformed output, and empty linked lists.
- Per-problem time, memory, and output settings; real memory allocation and page touching in both modes, terminated as Memory Limit Exceeded; maximum-size inputs for all four problems.
- Background-child termination, inherited-pipe cleanup, and restoration of drafts, statuses, results, and settings after reopening SQLite.

## P0 verification (2026-10-15)

- TypeScript/Vite builds, the Tauri desktop build, and `.app` packaging succeeded on macOS Apple Silicon.
- Eleven automated integration tests passed. Four problems × two modes executed 50 semantic test cases through a real C++20 compiler.
- In the desktop window, empty implementations produced wrong answers. Two Sum submissions in Function and ACM modes initially showed 5/5. Drafts, completion states, compiler settings, and the latest submission results survived a restart.
- After the resource-limit update, a desktop Two Sum Function submission showed 6/6, the active 1000 ms / 128 MiB budget, and each test's measured peak RSS sample.
- Judge tests passed in an execution environment without network access. Bundled assets load locally through `tauri://localhost`. No machine-wide network settings were changed for a physical-disconnection test.

## Using the application

1. The application detects `clang++` / `g++`, or accepts the executable's full path through the compiler field and the “检测并保存” button. Do not include extra compiler flags in the path. Detection checks that the executable starts; real judging compiles with `-std=c++20 -O0` and displays compiler diagnostics.
2. Select a problem and read its statement, constraints, logical examples, and ACM I/O protocol. Search by title, ID, or tag; filter by difficulty and tag.
3. In Function mode, implement only `class Solution`; the application provides headers, `main()`, and serialization wrappers. Linked-list problems also provide `ListNode`. In ACM mode, write the full program and follow the problem's I/O protocol.
4. “运行样例” runs the first two tests marked as samples. “提交本地测试” runs every local test for the problem. Only a full passing submission marks that mode complete. Completion remains as a practice record after subsequent edits or failed submissions; the latest submission result updates.
5. Edits are saved automatically to SQLite. Wait for “草稿已保存” before exiting. Each mode has its own draft. Sample results appear only in the current window; the latest submission result survives a restart. Results correspond to the last executed code, so edits require another run.

Bundled problems share 25 locally maintained semantic tests across both modes:

| Problem                                        | Wrapper / comparison focus                                                                  |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Two Sum                                        | Array and target arguments; accept any valid pair of indices                                |
| Longest Substring Without Repeating Characters | Whole-line strings, preserving spaces and empty strings                                     |
| Reverse Linked List                            | Build from a value sequence; serialize the returned list; detect cycles or excessive length |
| Move Zeroes                                    | Check the array after in-place mutation by a void method                                    |

Statements are independently written equivalents. These small, locally maintained test suites are not LeetCode's official hidden tests and do not establish official acceptance or complete edge-case coverage.

## P1: Technical Deep Dive

The currently selected provider is **Claude Haiku 5.5** (`claude-haiku-5-5`), as requested. The DeepSeek adapter and its separate key remain available through settings. System Design is not implemented.

- Ten bundled topics: cache selection, hot keys and cache failures, idempotency and retries, reliable message delivery, queue selection, rate limiting/circuit breaking, concurrent transactions, replica failover, sharding/expansion, and RPC tail latency.
- Select a topic → explicitly generate a question online → cache it in SQLite → start an interview → submit answers and receive dynamic follow-ups → finish with five-dimension qualitative feedback.
- Random local topic selection, cached-question reuse, history viewing, and unfinished-session recovery do not call an API.
- “AI 参考作答并追问（联网）” makes the model answer the current original question or follow-up, submits it as a candidate message, and asks the interviewer to continue. These turns are labeled “AI 参考代答”; their key points can be reviewed. Alternate freely between AI-assisted and handwritten answers. Assisted answers enter follow-up context, while final feedback distinguishes them from independent user answers.
- Internal follow-up directions and evaluation checkpoints are excluded from frontend question/session DTOs. Feedback appears only after the interview ends, without a percentage score.
- Up to 20 answers per session, each at most 6000 characters. Follow-ups send the current question and the latest 12 messages. Feedback sends the full current session, without other sessions or the problem library.
- Model and timeout (15–120 seconds) are configurable. Existing sessions keep their original provider/model when settings change.
- Failures preserve content and offer manual retries, without automatic paid retries. The overall deadline covers the entire request; whitespace keepalives do not reset it. Responses are limited to 1 MiB.

Open **Technical Deep Dive** and check the model settings. An existing `.env` is used directly. Otherwise, save a key through the application's password field, or copy `.env.example` to `.env` and fill it in without overwriting existing credentials. New configurations default to `claude-haiku-5-5`. Existing DeepSeek configurations are retained; switch providers in the dropdown and save. Existing sessions retain their original provider/model. Buttons identify online operations, and the UI shows the latest request status.

An AI-assisted operation normally makes two API requests: generating the answer and obtaining the follow-up. These may incur charges. An existing answer cache for that round is reused. The answer is saved before requesting the follow-up; use “重试待处理请求” after a follow-up failure to resume without regenerating or appending another answer. Submit or clear any current draft before using AI assistance. Historical messages and AI-assistance flags survive restarts.

### Temporary credential policy (user requested)

`credentials::read_key/save_key/delete_key` **only access the project's `.env`; they do not automatically read or write the system Keychain**. `.env` is Git-ignored, updated atomically, and given Unix mode 0600. `.env.example` contains blank values only. `DEEPSEEK_API_KEY`, `OPENAI_API_KEY`, and `CLAUDE_API_KEY` are independent; updating or deleting one does not overwrite another. Never use a `VITE_` prefix for secrets. Real keys are not stored in SQLite, logs, or frontend responses.

The previously saved DeepSeek system credential was moved and deleted as requested. The system-credential adapter remains available **for migration after real acceptance testing**; migration is not automatic. Test builds read `.env` from the project root recorded at compile time rather than the launch directory. This is a temporary development arrangement; system-credential migration is required before a distributable release.

### Failure recovery

Answers are written to SQLite and marked `awaiting_reply` before the API call. On failure, the answer remains both a local message and a pending draft. Restarting and retrying does not append another candidate answer. Finishing similarly enters `feedback_pending`; only complete, validated feedback marks the session `finished`. A session cannot end while it has an unsubmitted draft, preventing silent answer loss.

HTTP 401/403, insufficient balance, rate limits, service failures, empty or truncated responses, invalid JSON, missing fields, and invalid enum values produce recoverable errors. Raw provider error bodies and request debug objects are not echoed because they could contain credentials. Production requests use fixed Claude / DeepSeek HTTPS endpoints without following redirects. Clearing interview caches/history preserves LC data and keys and requires an additional UI confirmation.

DeepSeek integration follows [Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/). It uses JSON Output (`response_format: json_object`) with a JSON schema in the prompt. Rust applies strict Serde deserialization plus field-length, list, enum, and five-dimension validation; it does not assume provider-enforced JSON Schema support.

Claude integration follows the [Haiku 5.5 migration guide](https://platform.claude.com/docs/en/models/haiku-5-5/migration-guide), [Messages API](https://platform.claude.com/docs/en/build-with-claude/working-with-messages), and [structured outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs). It uses native `x-api-key` authentication, a top-level `system` prompt, `output_config.format` JSON Schema, and medium effort. Text blocks are selected by type; thinking blocks are not treated as answers. Feedback uses five required dimension keys, projected into the shared storage structure. Refusals, truncation, and missing content never count as success.

Both providers currently use `stream: false`. SSE streaming display is not implemented; input is sent as a whole when submitted.

### P1 verification and limitations

- Thirty default automated tests passed: 11 P0 regressions, nine HTTP/JSON/deadline/Claude-protocol tests, and ten interview storage, AI assistance, recovery, migration, and temporary `.env` tests.
- Provider-failure tests use explicit localhost HTTP fixtures. State-recovery tests use an explicit test provider. These do not establish real AI question, follow-up, or feedback quality.
- An isolated real macOS Keychain test passed before switching to temporary `.env` storage. That test is ignored by default; the current test stage does not write system credentials.
- A real DeepSeek `/models` request returned `deepseek-flash` and `deepseek-v4-pro`. Flash generation returned HTTP 503. Pro once returned HTTP 200 with continuing whitespace keepalives, without a verifiable completion. The desktop UI displayed failure, cleared its busy state, and retained settings. DeepSeek's complete real interview acceptance remains pending service recovery.
- A real minimal Claude Haiku 5.5 request succeeded with 14 input / four output tokens; metadata is in `/tmp/intv-claude-probe.json`. The first real interview feedback had an empty dimensions array and was rejected. After requiring all five dimension keys, retrying the original session succeeded with four messages and no duplicated candidate answer. Database reopening and offline reuse passed. A fresh database at `/tmp/intv-p1-live-claude-final.sqlite3` then passed real generation → two follow-ups → five-dimension feedback → reopening → offline reuse. AI technical judgments still require human review; individual generations are not guaranteed correct.

Automated tests do not require real keys; localhost HTTP fixtures require permission to bind a local port:

```sh
npm test
```

The following explicit live acceptance test **uses `.env` credentials and incurs API charges**. Its default database is `/tmp/intv-p1-live.sqlite3`. It does not run automatically:

```sh
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_deep_dive
# Select another available model and an independent test database.
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-v4-pro cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_deep_dive -- /tmp/intv-p1-live-pro.sqlite3
```

Claude Haiku 5.5 has also passed real original-question assistance → automatic submission → follow-up → assisted follow-up answer → further questioning → five-dimension feedback → reopening. Recovery after a failed follow-up did not duplicate the answer. Claude requests allow 8192 output tokens for thinking and complete JSON. Truncated or empty follow-ups are rejected and can be retried.

The following live AI-assistance test starts a new session using a cached Claude question in a test database, answers the original question and a follow-up automatically, then checks feedback and restoration. It makes real paid API calls:

```sh
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_reference -- /tmp/intv-p1-live-claude-final.sqlite3
```

To test the system-credential adapter separately (unnecessary during this stage; uses an isolated temporary credential rather than a real key):

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --test p1 os_keyring_roundtrip -- --ignored
```

## Per-problem complexity requirements and execution budgets

Every problem JSON includes `complexity` (time, auxiliary space, official requirements, training notes, and sources) and `limits` (`timeLimitMs`, `memoryLimitMb`, `outputLimitKb`). Details display both. The Judge enforces each problem's settings and records the applied budget, measured duration, and peak sampled main-process memory. Complexity requirements apply to both modes; ACM input-storage overhead must be described separately.

| Problem                                        | Local time target | Auxiliary space requirement / target              | Official basis                                                                                                                                                                                            |
| ---------------------------------------------- | ----------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Two Sum                                        | Average O(n)      | O(n)                                              | The [official follow-up](https://leetcode.com/problems/two-sum/) asks for better than O(n²). O(n log n) sorting also qualifies; a hash table is the local recommendation.                                 |
| Longest Substring Without Repeating Characters | O(n)              | O(min(n, Σ)); O(1) for fixed ASCII                | The [official statement](https://leetcode.com/problems/longest-substring-without-repeating-characters/) does not explicitly mandate Big-O. Sliding-window complexity is a local training target.          |
| Reverse Linked List                            | O(n)              | Iterative O(1); recursive O(n) stack also allowed | The [official follow-up](https://leetcode.com/problems/reverse-linked-list/) encourages iterative and recursive solutions. Recursion must not be misclassified as violating an official O(1) requirement. |
| Move Zeroes                                    | O(n)              | O(1), in-place, without copying the array         | The [official statement](https://leetcode.com/problems/move-zeroes/) explicitly requires in-place operation without copying. Minimizing operations is a follow-up.                                        |

Sources checked on 2026-10-15. Those public statements do not provide verifiable fixed per-problem millisecond/MiB quotas. **1000 ms / 128 MiB are local budgets, not official limits**. The [official compiler-environment description](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages) uses `-O2`, C++23, and AddressSanitizer; this P0 implementation keeps C++20 / `-O0`, so timings are not directly comparable.

Local input scales follow public constraints: strings up to 100000 characters, linked lists up to 5000 nodes, and arrays up to 10000 elements. Explicit local variants remain: multiple valid Two Sum answers, empty Move Zeroes arrays, and ASCII-only string inputs. All four problems have maximum-size tests with deterministic expected results.

Timing tests and RSS budgets cannot prove arbitrary code's asymptotic complexity or in-place semantics. RSS includes inputs, the C++ runtime, and wrappers; **it is not algorithmic auxiliary space**. For example, copying an array and writing it back can pass output tests while violating Move Zeroes' in-place requirement. Code review is still necessary. The UI explains this distinction rather than certifying complexity from resource compliance. Older results may lack memory or budget fields and are labeled accordingly; resubmission applies the new budgets.

## Two Sum: nested-loop complexity regression

Submission tests include `worst-case-10000`: `nums = [0, 1, ..., 9997, 100000, 200000]`, `target = 300000`. The only valid answer is `[9998, 9999]`, in either order. This is within n ≤ 10000 and makes `for (i) for (j=i+1)` scan all n(n−1)/2 pairs before finding the answer. Both modes share this submission test; it does not add a sample.

`src-tauri/tests/benchmarks/two_sum.cpp` counts operations and measures algorithm time for two known implementations at sizes 1000, 2000, 4000, and 10000. Tests check the counts, without asserting unstable timing ratios or claiming automatic complexity detection for arbitrary code. Run it separately with:

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features two_sum_worst_case_and_complexity_profile -- --nocapture
```

One real local C++20 / `-O0` run with counters (algorithm timings exclude compilation, input generation, and process startup):

| n     | Nested-loop pairs | Nested-loop time | Hash lookups | Hash time | Peak hash keys |
| ----- | ----------------: | ---------------: | -----------: | --------: | -------------: |
| 1000  |            499500 |          1.97 ms |         1000 |   0.26 ms |            999 |
| 2000  |           1999000 |          6.91 ms |         2000 |   0.50 ms |           1999 |
| 4000  |           7998000 |         27.85 ms |         4000 |   1.04 ms |           3999 |
| 10000 |          49995000 |        169.02 ms |        10000 |   2.44 ms |           9999 |

Nested loops use worst-case O(n²) time and O(1) auxiliary space. The hash table uses average O(n) time and O(n) auxiliary space, with possible degradation under extreme collisions. Space analysis excludes the input array. Auxiliary key counts measure data-structure size, not RSS or measured memory bytes.

A complete nested-loop program without counters also passed the real Judge: Accepted / 383 ms, including process overhead. A one-second limit does not guarantee that O(n²) times out. Hardware, compiler options, and implementation affect speed. Correctness acceptance does not imply optimal complexity; tests do not fake TLE.

## Structure and storage

```text
content/leetcode/problems/      Problem, template, and logical-test JSON
src/main.tsx                   LC UI, Monaco, and Tauri calls
content/leetcode/adapters/      C++ main wrappers for Function mode
src-tauri/src/content.rs        Bundled library, input adapters, and comparators
src-tauri/src/code_runner.rs    Compilation, subprocess control, and judging
src-tauri/src/db.rs             SQLite initialization and LC drafts/status
src/features/interview/         Deep Dive settings, topics, sessions, feedback, and history UI
src-tauri/src/interview.rs      Topics, strict response types, and validation
src-tauri/src/interview_store.rs Question cache and resumable interview state
src-tauri/src/llm.rs            Claude / DeepSeek requests, schemas, deadlines, and safe errors
src-tauri/src/credentials.rs    Temporary .env credentials; retained system adapter
content/deep-dive/topics.json   Ten topic seeds
src-tauri/src/main.rs           Tauri commands
src-tauri/tests/p0.rs           Integration tests with real compilation and execution
```

Problems are embedded into Rust at compile time. Monaco and its workers are bundled by Vite; there is no runtime CDN or LeetCode API dependency. After installing dependencies and packaging, LC problems, editing, judging, and storage work without external network access. SQLite uses Tauri's application-data directory, for example on macOS:

```text
~/Library/Application Support/local.swe.interview-prep/prep.sqlite3
```

User progress and interview data are stored in the application-data directory. LC needs no `.env` or API key. Under the current testing policy, Claude / DeepSeek keys temporarily live in the project-root `.env`; non-sensitive model and timeout settings live in SQLite. The temporary key file is an explicitly authorized exception for this stage.

Execution boundary: run only personally authored, trusted C++ code. Each Judge run uses an automatically cleaned temporary directory. Compilation has a 30-second / 256 KiB output limit. Per-test budgets come from `limits` (currently 1000 ms / 128 MiB main-process memory / 64 KiB combined stdout and stderr). Unix programs run in their own process groups; completion, timeout, and output violations terminate the whole group. CPU time, individual file size, open-file count, and core dumps are restricted. Main-process RSS is sampled through macOS `proc_pidinfo` or Linux `/proc`; exceeding the problem's budget terminates the group. Sampling is approximately every 5 ms and can miss brief peaks. It does not aggregate child-process memory or enforce a hard OS memory quota. Persistent failure to read a live process's memory produces an explicit error rather than silently skipping enforcement. There is no strong memory/network/filesystem security sandbox, so this application must not execute untrusted code. Judge commands run on background workers rather than blocking compilation on the UI thread.

## Remaining work (outside this change)

- Keep temporary `.env` testing mode as requested; enable system-credential storage before distribution. DeepSeek's real multi-turn acceptance remains pending service recovery.
- P2: System Design scenarios, stage control, and explainable feedback.
- P3: Complete and verify the full Hot 100, add adapters/comparators, and improve quality.
