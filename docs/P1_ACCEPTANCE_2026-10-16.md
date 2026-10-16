# P1 comprehensive regression and representative live acceptance

Date: 2026-10-16, Asia/Tokyo.

## Outcome and scope

The two reported defects are fixed: topic changes no longer retain an unrelated interview, and P1 prose/code now uses the shared safe Markdown renderer. Automated P1/shared regressions passed **103 checks**: 92 backend and 11 frontend. One OS Keychain round-trip remains intentionally ignored under the user's `.env`-only policy. TypeScript/Vite and native debug packaging passed; the existing large-bundle warning remains.

This run covers the local content/contracts for all 120 P1 topics, storage/recovery/reference/feedback flows, shared session tools, representative real DeepSeek output, and native computer-use acceptance. It does not mean every possible question from every topic has passed semantic review. No successful compilation, judging or live provider response was mocked. Local HTTP fixtures and explicitly labeled scripted answers are separate from live-provider evidence.

## Fixes

### Topic and request ownership

- Selecting a different topic flushes the current draft before clearing the visible interview and draft editor. The original interview remains available in history.
- Restoring a session synchronizes the selected topic and cached question, clearing incompatible filters when necessary.
- Every explicit action supersedes an in-flight default restoration. Async generated questions, started sessions, follow-ups, references, feedback and error refreshes apply only while their navigation owner remains current. A stale generated question can remain cached without opening its old session over a new selection.
- Synchronous request ownership prevents double submission before React has rendered the busy state. Streamed output is ignored once its owning navigation is stale. Review topic navigation clears the previous conversation too.
- The backend still persists submitted answers and pending work before network calls. The UI fix does not delete old sessions or rewrite caches.

### Markdown

`src/Markdown.tsx` extracts the existing renderer for P1 and P2. P1 cached previews, questions/background/constraints, submitted turns, reference answers/key points, feedback and streamed content now support Markdown. Fenced code retains language classes, line breaks, indentation and horizontal scrolling; inline code, lists and tables render normally. The answer editor still accepts editable Markdown source.

Raw HTML is skipped, images are not loaded and links remain non-navigable. Stored text and exports stay Markdown strings rather than generated HTML. P2 retains the same safety rules and receives shared styling. Author prompts request language-tagged fenced code inside JSON strings, without fencing the entire JSON response.

New cached questions record the actual current policy version, rather than the old hard-coded version. Older caches are untouched.

## Automated and build evidence

| Run                                                          | Result                                           |
| ------------------------------------------------------------ | ------------------------------------------------ |
| Backend library                                              | 58 passed                                        |
| P1 integration                                               | 14 passed, 1 intentionally ignored Keychain test |
| P2 shared regression                                         | 12 passed                                        |
| Process/resource reliability                                 | 3 passed                                         |
| Timers, exports and API accounting                           | 5 passed                                         |
| Frontend navigation, Markdown, costs and voice draft helpers | 11 passed                                        |
| TypeScript/Vite production build                             | Passed                                           |
| Native macOS debug app packaging                             | Passed                                           |

The first sandboxed Rust run had ten failures because localhost fixture sockets could not bind. It also exposed two obsolete assertions: exact anti-leak wording changed, and a p99 teaching topic legitimately mentions p99. These were corrected without weakening the runtime anti-leak guard; the p99 exception is scoped to that topic. The final rerun outside the socket restriction passed.

Commands:

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features \
  --lib --test p1 --test p2 --test reliability --test session_tools
npm run test:review-ui
npm run build
npm run tauri build -- --debug --bundles app
```

Native packaging used an isolated identity `local.swe.interview-prep.p1-qa`, separate from the user's normal application storage. Final logs: `/tmp/intv-p1-final-rust.log`, `/tmp/intv-p1-final-ui-tests.log`, `/tmp/intv-p1-post-navigation-build.log` and `/tmp/intv-p1-final-policy-native-build.log`. The last packaging run reuses the already verified, unchanged frontend assets and rebuilds the final Rust policy. The QA bundle is `/tmp/intv-calibration-check-target/debug/bundle/macos/SWE Interview Prep P1 QA.app`.

## Native computer-use acceptance

Verified in the real Tauri window at 1280 x 850:

- Initial P1 view: 120 topics, 94 core matches, collapsed filters and usable topic-list height.
- Expanded/collapsed filters retain search and show active counts; a no-match search disables random selection; Clear filters restores all 120.
- English/Chinese labels and light/dark rendering are readable. Real generated JavaScript code appears in a separate monospaced block with preserved line breaks rather than visible fence markers.
- Offline cache reuse opens the correct interview. Entering a draft, changing topics and reopening history preserves the original draft while removing the old question from the new topic's conversation area.
- Selecting ten minutes does not start a timer. Start enables the running countdown; Pause preserves elapsed time.
- Export opens the confirmation dialog and Cancel leaves the interview/draft intact.
- A saved cache-diagnostics feedback failure exposes Retry. The approved native retry sends the scripted content to DeepSeek, finishes with two candidate turns preserved, and displays Added to Review. Busy controls disable competing submissions/topic changes.

The code preview used a real generated initial sample that was subsequently rejected for a wording-level answer hint. Its rendering is valid UI evidence; its question quality is not an accepted sample. Revised question quality is evaluated separately below.

The final build was restarted: dark/English preferences persisted, the saved SQL interview restored with its matching topic and fenced SQL, and Review → Practice selected the cache topic without retaining the SQL conversation.

## Real-provider batch and findings

Only DeepSeek `deepseek-flash` was used for text. The six topics were B+ tree indexes (Chinese), TCP reliability (English), payment callbacks (Chinese), event-loop scheduling (English), cache diagnostics (Chinese), and slow SQL diagnosis (English). All requests used prepared training content and an isolated database, not the user's personal history or drafts. No provider/key setting in the normal application was changed. OpenAI was not called.

The initial batch made 16 HTTP attempts and had transport failures. Automatic approval initially rejected a further paid run because repository instructions prohibit automatic paid reruns; the user explicitly approved the bounded four-topic recheck and native UI calls. The scripted recheck and saved-feedback continuation brought the cumulative scripted total to 45. The final B+ feedback check used another two scripted calls (47 scripted total), and native cache-feedback retry used two UI calls: **49 actual HTTP attempts total**, below the approved cap of 60. Original failures remain recorded.

The recheck exposed and corrected a real answer-hint regression: a JavaScript question used “explain why zero delay does not make B precede C,” which supplies a partial output order. Generation/review policy now explicitly rejects negation, corrected-colleague and why-not variants of this hint. It also rejects diagnostic constraints that pre-list the evidence the candidate is meant to propose. The revised JavaScript sample simply presents fenced code and asks for output/reasoning; the revised cache opening leaves the cause unknown and asks for discriminating evidence.

The predefined workflows completed across the six representative topics; only the three feedback cases were intentionally finished:

| Topic             | Live coverage                                                                                    | Final state                     |
| ----------------- | ------------------------------------------------------------------------------------------------ | ------------------------------- |
| B+ tree           | Generation, independent answer, follow-up, feedback and saved-feedback recovery                  | Finished                        |
| TCP reliability   | Generation and independent-answer follow-up                                                      | Active, intentionally left open |
| Payment callback  | Generation and independent-answer follow-up                                                      | Active, intentionally left open |
| Event loop        | Generation, reference preview, assisted submission, follow-up and AI-only feedback               | Finished                        |
| Cache diagnostics | Generation, independent/assisted follow-ups, preview and mixed feedback recovered through the UI | Finished                        |
| Slow SQL          | Generation, independent-answer follow-up and non-submitting reference preview                    | Active, intentionally left open |

B+ feedback resumed after TLS failure. AI-only event-loop feedback has all five dimensions `missing`, empty strengths/gaps and explicit assistance disclosure. Mixed cache feedback credits one independent answer, not the assisted follow-up. Reference previews do not submit answers; follow-up retry does not append another answer. Bilingual cached questions can be reused offline after reopening storage.

The final manual score review found another concrete defect: the earlier B+ feedback awarded partial ratings because the candidate had not volunteered unasked internal-node details or a specific boundary example, creating false weak points. Policy v10 explicitly rejects that absence-based downgrade. The same real question and independent answer were re-evaluated, without changing stored history: principles, local rationale and communication became good; trade-offs and failure/correctness were missing; gaps were empty. The final correction used two HTTP attempts and is recorded in the separate foundation-feedback artifact. The earlier scores remain visible in the isolated QA history as historical evidence, not as accepted final ratings.

Semantic limits remain: the cache feedback can overstate missing verification detail even though the candidate proposed sampling database results. Explicit evidence-to-hypothesis mapping remains a reasonable practice target, but that does not prove the candidate had no diagnostic knowledge. Reviewer acceptance is not infallible, and successful recovery does not make DeepSeek's intermittent network/TLS failures disappear. TCP/payment samples come from the earlier policy in this run; four affected/failed topics were rechecked under the final generation policy.

Artifacts:

- [Acceptance manifest](../review/p1-practical-acceptance.json)
- [Initial batch, including failures](../review/p1-practical-live-initial.json)
- [Recheck before feedback resume](../review/p1-practical-recheck-before-resume.json)
- [Recheck and saved feedback continuation](../review/p1-practical-live-recheck.json)
- [Native UI retry result](../review/p1-practical-ui-feedback.json)
- [Final foundation-feedback correction](../review/p1-foundation-feedback-final.json)

No full LC reference-solution sweep or new voice-transcription batch was needed for these P1 changes. Voice draft helpers and microphone placement remain covered by current automated/native checks and the earlier dedicated voice acceptance. No Keychain migration is authorized.
