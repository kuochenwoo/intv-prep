# P2 System Design

P2 is a separate whole-system design module. P1's topic library, prompts, five-dimension rubric, reference-answer workflow and session data remain separate. Further P1 knowledge-library expansion is deferred to the later content phase.

## Run

```sh
npm install
npm run tauri dev
```

A C++20 compiler is needed only for LC judging. Open **System Design** in the header. Use the shared model settings to select Claude or DeepSeek; the configured Claude model is `claude-haiku-5-5`. Existing provider-specific keys remain in the Git-ignored `.env`; this change neither migrates keys to Keychain nor stores them in SQLite.

1. Choose URL Shortener, Coupons and Flash Sales, Chat and Realtime Messaging, Activity Feed, or Distributed Task Queue. All seeds have Chinese and English content with functional scope, scale ranges, reliability targets and deep-dive directions. These are local training assumptions, not claims about real services.
2. Generate a question online. The concrete variation is cached, with its language and generation provenance. Reuse a cached question to start another interview offline.
3. Move freely among Requirements, Estimation, Architecture, Deep Dive and Trade-offs. Stage changes are local; AI suggestions never change the stage. Each stage retains its own draft. Sessions pin the provider/model selected at their start and the question language.
4. Write Markdown notes and Mermaid source in the design workspace. Both save locally. Preview Markdown and render Mermaid without a model call. Notes and diagram source accompany submitted answers and feedback requests; unsubmitted answer drafts do not. Raw HTML, remote images and clickable external links are excluded from note rendering. Mermaid uses strict rendering and an isolated iframe for its SVG preview.
5. Submit an answer for a streamed, stage-specific discussion. Estimation examines formulas, units, average/peak QPS and capacity assumptions; later stages examine APIs, models, end-to-end data flows, bottlenecks and cross-component failure/trade-offs.
6. Submit or clear remaining stage drafts, then finish at any stage. Review eight qualitative dimensions: requirements, estimation, architecture/data flow, scalability, availability/failure, consistency/correctness, trade-offs and communication. Each contains a rating, evidence and actionable improvement. No numerical score is fabricated.
7. Restore history offline, including notes, diagrams, stage drafts, conversation and feedback. A pending online request survives restart and can be retried without adding another candidate answer.

## Implementation boundaries

- `content/system-design/scenarios.json`: five bilingual seeds.
- `src-tauri/src/system_design.rs`: contracts, stage guidance, independent bilingual prompts and strict eight-dimension validation.
- `src-tauri/src/system_design_store.rs`: two new SQLite tables, atomic session updates and durable pending states. Existing coding and P1 tables are preserved.
- `src/features/system-design/`: Astryx UI, Markdown and lazy-loaded Mermaid preview.
- The existing provider transport handles streaming, deadlines and structured decoding. P2 supplies its own system instructions rather than inheriting P1's single-mechanism restrictions. P1's request path continues to use its existing instructions.

A stage jump can leave an AI question unanswered. Grading receives all candidate answers, but only interviewer questions immediately followed by an answer in that stage. Skipped and trailing unanswered questions remain visible in local history, while being excluded from grading evidence. Private planning/checklist fields are also excluded from the grading question context.

Limits keep this MVP bounded: 6,000 characters per stage draft/answer, 20,000 in notes, 12,000 in diagram source, and 100 conversation entries / 120,000 answer-history characters before further submissions are rejected. Users can still finish the interview at the conversation limit. Runtime estimates and architecture judgements are model feedback, not measured infrastructure benchmarks or formal correctness proofs.

## Verification

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib --test p1 --test p2
npm run build
npm run tauri build -- --debug --bundles app
```

P2 automated cases cover bilingual seeds, separate prompt policy, schema validation, draft/notes restoration, free stage changes, advisory-only suggestions, pending answer/feedback recovery, duplicate prevention, bounds, and isolation from P1/LC data. A unit check excludes private drafts and unanswered stage prompts from grading context. HTTP protocol tests use explicit local fixtures; they do not stand in for live API acceptance.

Explicit real-provider acceptance (uses API credits, an isolated `/tmp` database and the existing `.env` key):

```sh
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_system_design
# Optional English run:
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_system_design -- --english
# Recheck feedback against the saved scripted live interview:
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_system_design -- --feedback-only
```

The live probe uses clearly scripted candidate inputs, not evidence about a human's ability. It generates a real scenario, discusses all five stages, produces eight-dimension feedback, reopens the database and verifies offline question reuse. Reports are `/tmp/intv-p2-live-report.json` and `/tmp/intv-p2-feedback-review.json`. No credentials are printed or included in these reports.

## Deferred

Further P1/SD content expansion, a drag-and-drop whiteboard, numerical scoring/radar charts, and Keychain migration are outside this change. DeepSeek subsequently passed a real staged interview and recovery checks; see [the reliability follow-up](RELIABILITY_2026-10-15.md). Provider availability can still vary. Model judgement quality still benefits from review across more scenarios and candidate answers.

### Acceptance results (2026-10-15)

- Final backend regression: **33 passed** (15 library/protocol, 10 P1, 8 P2), with the existing OS Keychain round-trip intentionally ignored. No LC content, resource budgets or judge implementation changed.
- Real Claude acceptance completed the generated Chinese task, all five stages, eight-dimension feedback, database reopen and cached reuse. A separate real feedback request checked the revised grading context and compared the two submitted architectures without treating the skipped final migration question as a failed answer.
- Computer-use acceptance generated an English task in the desktop app, preserved independent requirements/estimation drafts through stage switches, submitted both answers, and finished with eight English feedback dimensions. The model checked the submitted arithmetic (about 57.9 writes/s and 2.74 TB raw storage under those scenario assumptions). These were scripted practice inputs, not claims about the user's performance.
- UI checks covered Chinese/English labels, conversation-language preservation, Markdown headings/tables, actual Mermaid rendering in light and dark modes, an invalid diagram with preserved source, and successful rendering after restoring valid source. The generated practice session and notes remain in local history; the UI language was restored to Chinese.
- Production TypeScript/Vite builds and native debug packaging succeeded. A production bundle-size warning remains; Mermaid is loaded only when previewing a diagram. Model evaluations are qualitative practice feedback and are not guaranteed to be correct for every answer.

- Final app restart restored the completed UI interview and all eight feedback entries. Expanding the read-only design workspace showed the saved Markdown notes and valid Mermaid source; the compact default panels and feedback order were also checked in the rebuilt app.

The focused checks above do not claim exhaustive accessibility, viewport, provider or scenario coverage. Further topic expansion remains deferred as requested.
