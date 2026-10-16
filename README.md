# SWE Interview Prep

English · [简体中文](README.zh-CN.md)

A local desktop app for software-engineering interview practice, built with Tauri 2 (Rust) and React.

See [the project TODO](TODO.md) for AI-quality calibration and the deferred feature backlog.

- **LeetCode Judge:** all 100 LeetCode Hot 100 problems, in C++20. Each problem can be solved in Function mode (write only `class Solution`) or ACM mode (write the full program). Tests run offline with a real local compiler, with time, memory and output limits. Each problem lists its time and space requirements.
- **Technical Deep Dive:** an AI interviewer (Claude or DeepSeek) uses 120 bilingual topics for foundation questions, applied scenarios and troubleshooting, asks follow-ups on your answers, and gives five-dimension feedback at the end. Answers stream in as they are written. An optional AI reference answer shows one concise model answer for the current question.
- **System Design:** 20 bilingual scenarios, user-controlled interview stages, capacity estimates and architecture discussions, Markdown notes, local Mermaid previews, and eight-dimension feedback. Questions, drafts and history persist locally. See [the P2 guide](docs/P2_SYSTEM_DESIGN.md).
- **Interface:** Chinese or English, switched with the button in the header. Light and dark modes. Built with Meta's [Astryx](https://github.com/facebook/astryx) design system.

Problem statements, interview topics, AI questions and answers, and error messages follow the selected language.

The latest [practical fundamentals expansion](docs/P1_PRACTICAL_FUNDAMENTALS.md) adds P1 question-style/priority filters and five complete P2 exercises. This iteration is implementation-only; its automated, build, UI and live-provider validation is pending at the user’s request.

## Requirements

- Node.js 22.12+ and npm
- Stable Rust and the [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform
- A C++20 compiler: `clang++` or `g++`. On macOS, Xcode Command Line Tools provide one.

## Run

```sh
npm install
npm run tauri dev
```

`npm run dev` starts only the web frontend, which cannot reach the compiler. Use the Tauri window.

## Build and test

```sh
npm run build                         # type-check and build the frontend
npm test                              # Rust tests: judge, every problem in both modes, storage, AI client (no paid API calls)
npm run tauri build -- --bundles app  # unsigned macOS .app
```

The app is written to `src-tauri/target/release/bundle/macos/`. It still needs a C++ compiler installed.

## AI settings

Open **Settings** and pick an interview provider, model and timeout. Then save your API key there, or copy `.env.example` to `.env` and fill it in. Keys are kept only in the project's `.env` (Git-ignored, mode 0600). They are never written to SQLite or sent to the frontend. The default model is `claude-haiku-5-5`.

Generating questions, answering, AI reference answers and feedback call the provider's API and may cost money. Viewing history, reusing cached questions and saving drafts stay offline.

## Voice input

P1/P2 answer panels include a small microphone icon for optional recording, followed by playback, OpenAI transcription and an editable preview. Record, stop, choose Transcribe, correct the text and append it to your answer draft; submitting the answer remains a separate action. `OPENAI_API_KEY` in the ignored `.env` is used only for `gpt-4o-mini-transcribe`; it never changes the Claude/DeepSeek interview provider. A separate key field lives under Settings → Voice transcription settings. Audio stays in memory until you explicitly request transcription. See [voice input](docs/VOICE_INPUT.md) for limits and pending validation. Core voice validation passed with three real OpenAI transcriptions, native microphone checks and 91 automated checks. The accompanying DeepSeek text batch retains a P2 feedback connection failure; see the report for its limits.

## Data

Progress, drafts and interviews are stored in SQLite, in the app-data folder. On macOS that is:

```text
~/Library/Application Support/local.swe.interview-prep/prep.sqlite3
```

## Adding problems

Each problem is one JSON file in `content/leetcode/problems/`. Its `io` spec drives the Function-mode wrapper, the ACM input and the answer checking. `build.rs` bundles new files automatically. For the full workflow (generator scripts, reference solutions, the `verify_problem` tool and content rules), see [tools/leetcode/README.md](tools/leetcode/README.md).

## Project layout

```text
src/                         React UI (main.tsx = LeetCode, features/interview = Deep Dive, i18n.tsx = UI strings)
src-tauri/src/               Rust: judge, problem library, SQLite, LLM client, Tauri commands
src-tauri/tests/             Integration tests and reference solutions
content/leetcode/            Problem JSON and custom C++ harnesses
content/deep-dive/           Interview topic seeds
tools/leetcode/              Problem generators and authoring guide
```

## Limitations

- The judge runs your own code with your user's permissions. It is not a sandbox, so do not run untrusted code.
- Judging uses Unix process groups and resource limits. It works on macOS and Linux; Windows is not supported yet.
- Time and memory limits are local settings, not LeetCode's official ones. Passing the tests does not prove an algorithm meets its Big-O or in-place requirement.
- Some large tests are scaled below LeetCode's maximum sizes to keep the app small. Each problem's notes say where.
- AI questions and feedback can be wrong. Treat them as practice material.
