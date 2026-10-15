# P1 scenario-based fundamentals

P1 examines one underlying mechanism in a small concrete scenario. P2 is implemented separately for complete architectures. This document records the initial ten-topic policy revision; the subsequent [40-topic expansion](P1_CONTENT_EXPANSION.md) preserves it. The ten original topic IDs stay stable; their Chinese and English titles, focus areas, and scenario seeds now use concrete operations instead of combined throughput, availability, and scaling goals.

## Interview behavior

- Start with one causal or execution-trace question answerable in two to three minutes. Use two or three necessary constraints, explicitly stating implementation assumptions when they affect the answer.
- Treat topic focus items as optional directions. Do not combine explanation, component selection, optimization, and disaster recovery into the opening question.
- Follow the actual answer: ask for an execution step after jargon, use a minimal counterexample for a misconception, or narrow to a prerequisite when the candidate is unsure.
- After a sound explanation, probe one related concurrency/failure boundary or one local alternative's cost. Change at most one explicit condition per follow-up. Suggest ending after independent explanation of the mechanism and a related boundary, rather than expanding to fill rounds.
- Keep reference answers focused on the current question. Preserve the preview, copy-to-draft, explicit AI-assisted submission, and retry behavior.
- Keep the existing five feedback dimensions, but assess only actual evidence. Describe an unasked dimension as not assessed, rather than as demonstrated weakness. AI-assisted answers remain excluded from evidence of independent understanding.

For example, the cache topic gives this sequence: a reader obtains database value v1; a writer commits v2 and deletes the cache; the reader caches its previously read v1. The opening asks why stale values can persist. Follow-ups can trace the sequence, test a proposed fix against that same sequence, or examine one local cost.

Other seeds cover expired-key request coalescing, idempotency races, acknowledgement timing, partition ordering versus worker completion, token-bucket bursts, inventory read-modify-write, asynchronous replica lag, modulo remapping, and nested retry amplification. The existing topic collection remains ten topics; this change does not claim comprehensive coverage of every language/runtime/OS interview subject.

## Compatibility and validation

The initial policy revision recorded `deep-dive-v3-fundamentals`; the subsequent [AI quality calibration](AI_QUALITY_CALIBRATION.md) records `deep-dive-v4-calibrated` for newly cached questions. Existing cached questions and sessions are preserved, including their original wording. Generate a new question to use the new opening-question policy. Subsequent model requests use the updated follow-up, reference, and feedback policies.

Automated policy checks cover both providers' request construction, bilingual isolation, streaming flags, separation of candidate data from system instructions, and concrete bilingual seeds. Existing protocol, persistence, AI-reference, and retry tests remain applicable. These checks protect the prompt contract; they cannot prove every future model response follows it.

The explicit live probe is `src-tauri/examples/live_fundamentals.rs`. It reads the existing Claude key from `.env`, makes real streamed calls in both languages, and saves generated questions, a prerequisite follow-up, a boundary follow-up, and feedback to `/tmp/intv-p1-fundamentals-live.json`. It uses no application database and makes no credential writes. Sample candidate answers are scripted probe inputs, not claims about a human's ability. Run it only when live API usage is intended:

```sh
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_fundamentals
```

## Verification on 2026-10-15

- Final backend run: 14 library/protocol checks and 10 P1 integration checks passed; the OS Keychain round-trip remains intentionally ignored. The two new checks protect the bilingual policy in both adapters and the smaller topic seeds. Localhost transport fixtures needed execution outside the filesystem/network sandbox; sandbox-denied socket creation was rerun successfully rather than counted as a product failure.
- Production frontend build and native debug app packaging succeeded. Prettier and `git diff --check` passed.
- The final explicit Claude run completed eight real streamed requests: generation, a prerequisite follow-up, a boundary follow-up, and five-dimension feedback in each language. It used scripted candidate inputs and no application history. The English prerequisite became a single before/after judgement; both languages acknowledged the correct core explanation and moved to a local ordering boundary. Both summaries rated the core principles `good`, and selection rationale/trade-offs were identified as unassessed.
- Computer-use testing of the rebuilt app verified the ten narrowed topics, Chinese/English labels and draft guidance, dark-mode layout, and one real Chinese question generated and cached through the UI. The new question asks why a delayed refill survives deletion; the prior architecture-heavy session remains available. The UI-created practice session was kept with an empty draft, and the UI was restored to Chinese with the existing dark theme.

Model quality still needs review across varied answers and topics. Even the improved live samples sometimes bundled related timing details in a prerequisite or marked an already implied detail as an omission in feedback. Hidden generated directions also occasionally contained awkward timing. The tests and successful live transport do not certify every model judgement or guarantee a single semantic subquestion in every response. This change establishes the narrower P1 policy and seeds; it does not introduce an automatic semantic grader or claim these residual calibration issues are solved.
