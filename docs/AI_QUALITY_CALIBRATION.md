# AI quality calibration

Date: 2026-10-15 (Asia/Tokyo).

## Scope

Calibrate P1/P2 against the expanded 40-topic and 15-scenario libraries. This work does not implement the deferred voice, study-plan, mock-interview, usage-accounting or export features. Existing cached questions and feedback remain historical records; the changes apply to new requests, without rewriting history.

## Changes

- P1 generation explicitly separates visible code/state/environment from the hidden answer mechanism. Visible constraints must not supply the requested output or explanation. Private opening checkpoints cover only the actual opening question; optional comparisons and optimizations belong in follow-up directions.
- P1 follow-ups accept equivalent natural language, acknowledge explanations already given, narrow for uncertainty and explicitly name a changed condition. P2 follows one main decision in the selected stage and preserves the candidate's stated commit/durability ordering.
- P1 feedback receives only independent candidate answers and the interviewer questions immediately followed by independent answers. It excludes AI-assisted answer text, unanswered prompts and hidden generated directions/checklists. Trusted assistance counts provide disclosure context without giving the grader assisted text as ability evidence.
- After a successful, schema-valid provider response, a local provenance guard forces AI-only feedback to five `missing` dimensions, explicit unassessed evidence, independent-practice advice, and no independent strengths or technical gaps. This is application policy, not a claim that the model obeyed. Mixed histories retain the model's assessment of independent evidence and receive an enforced assistance disclosure. A provider/network/decoding failure still returns an error; the guard does not fabricate a successful request.
- P2 feedback excludes model-authored observations, unsubmitted drafts, private checklists and unanswered prompts. Submitted answers, relevant answered questions and shared design notes remain available.
- P2 prompts distinguish task inputs, explicit illustrative/alternative baselines, arithmetic and conclusions. They recognize explicitly declared uncompressed estimates, permit task-number substitution without falsely alleging missing assumptions, avoid invented compression ratios/benchmarks/prices/instance counts, and distinguish percentile latency from mean processing time.
- New question cache provenance records `deep-dive-v4-calibrated` and `system-design-v2-calibrated`. Existing IDs, cache entries, schemas, drafts, retry states, credentials and interview workflows are preserved.

## Regression inputs

[`content/evals/interview-quality.json`](../content/evals/interview-quality.json) contains scripted inputs and human-review criteria. They are not transcripts of a human candidate or expected model outputs hard-coded as successes.

| Case                        | Topic/scenario                                    | Intended review                                                                            |
| --------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `p1-correct-event-en`       | Browser event loop                                | Credit an equivalent correct explanation; no keyword demand or unasked gaps                |
| `p1-partial-mvcc-zh`        | PostgreSQL statement snapshots                    | Acknowledge the result and probe its causal step                                           |
| `p1-incorrect-volatile-zh`  | Java volatile increment                           | Identify the specific atomicity misconception                                              |
| `p1-unknown-epoll-en`       | Linux edge-triggered readiness                    | Ask one concrete prerequisite rather than a full implementation                            |
| `p1-assisted-only-event-en` | AI-only event-loop answer                         | Application-enforced unassessed ratings and disclosure                                     |
| `p1-mixed-mvcc-zh`          | Assisted explanation plus independent uncertainty | Exclude assisted evidence; disclose assistance; do not invent mastery                      |
| `p2-file-alternative-zh`    | File sync estimates/architecture                  | Recognize illustrative arithmetic, valid alternatives and durable-chunk ordering           |
| `p2-metrics-estimate-en`    | Metrics capacity                                  | Credit explicit baseline alignment; do not demand unmeasured percentages or machine counts |

Two additional real generation probes use `javascript-event-loop` (English) and `mvcc-snapshots` (Chinese) to inspect visible answer leakage and checklist scope. The technical assumptions and primary sources remain in the P1/P2 content-review manifests.

## Validation

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib --test p1 --test p2
# Intentional real API usage with existing provider-specific .env credentials:
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-flash \
  cargo run --manifest-path src-tauri/Cargo.toml --no-default-features \
  --example live_ai_quality
```

The live example makes 16 logical requests (two generations, ten P1 follow-up/feedback requests, four P2 follow-up/feedback requests), with the shared bounded content-decoding retries. It writes `/tmp/intv-ai-quality-live.json` incrementally, prints case completion only, and does not write credentials or application history. Expected criteria require human semantic review; passing transport and schema checks cannot certify them. The AI-only assertion checks the application guard rather than model compliance.

Automated fixtures verify request contracts, evidence filtering, both-language provenance enforcement, prompt-language isolation, P2 assumptions policy, and existing persistence/retry behavior. They do not claim a real provider call or automatically prove semantic quality.

## Execution status

- Local library/P1/P2 suite: **42 passed, 0 failed, 1 intentionally ignored OS Keychain round-trip** (21 library, 11 P1, 10 P2). Logs: `/tmp/intv-ai-quality-tests-final.log`.
- `live_ai_quality` compiled successfully in an offline check; this is compilation, not a provider request. Log: `/tmp/intv-ai-quality-example-check.log`.
- Native debug app build/packaging passed (73.66 MiB). Log: `/tmp/intv-ai-quality-desktop.log`. No frontend source changed in this calibration, and the previously built reading-font assets are retained.
- Real-provider semantic review is pending approval of the concrete 16-request scripted batch; no real request in this batch has been sent yet.

Semantic results and remaining findings will be appended after execution. Earlier P1/P2 expansion reports remain the before-change evidence, not acceptance of this new policy.
