# Project TODO

Updated: 2026-10-15 (Asia/Tokyo).

This list records the agreed next step and future feature ideas. Recording an item does not start its implementation. Finish AI-quality calibration before adding the deferred features below.

## Completed baseline

- [x] P0: project setup and real local C++20 judging in Function and ACM modes.
- [x] P1: streamed Technical Deep Dive interviews, reference answers, five-dimension feedback, caching and recovery; 40 bilingual topic seeds.
- [x] P2: five user-controlled interview stages, capacity estimates, architecture notes/Mermaid, eight-dimension feedback, caching and recovery; 15 bilingual scenario seeds.
- [x] LeetCode Hot 100 coverage with complexity metadata, enforced local resource budgets and reference solutions verified in both modes.
- [x] Reliability follow-up and reading-focused UI typography adjustments.

These completed workflows do not imply that every model-generated question or judgement is correct.

## AI calibration: completed for the agreed Claude baseline

- [x] Improve P1 question generation: assess one core mechanism in a concrete scenario, avoid giving away the answer in visible constraints, and keep optional extensions out of mandatory opening-question criteria.
- [x] Improve follow-ups in both modules: respond to the actual answer, acknowledge what is already explained, avoid repetitive demands, and explicitly identify changed assumptions. Keep P1 at mechanism/local-boundary depth and P2 at its selected architecture stage.
- [x] Improve evidence-based feedback: do not count unasked material or unanswered interviewer questions as demonstrated weaknesses. Accept equivalent explanations rather than requiring keywords or a prescribed phrasing.
- [x] Reliably exclude AI-assisted answers from evidence of independent understanding. AI-only evidence must not receive independent-ability credit; disclose assistance and use `missing` for dimensions with no independent evidence.
- [x] Calibrate P2 estimates: recognize explicitly stated illustrative or alternative assumptions, distinguish them from task defaults, verify formulas/units, and explain when task figures should be substituted. Do not invent benchmark results or demand unsupported precision.
- [x] Calibrate P2 architecture feedback: accept defensible alternatives, evaluate end-to-end data flow and cross-component trade-offs, and distinguish genuine contradictions from an interviewer silently changing the failure window.
- [x] Build regression samples from new P1 topics and P2 scenarios using correct, partially correct, incorrect and "I do not know" answers. Include AI-assisted and mixed independent/assisted histories, bilingual cases and the failures already observed in live probes.
- [x] Add necessary automated checks and review real-provider samples. Record provider/model, input assumptions, expected behavior and observed defects. Report transport/schema success separately from semantic-quality acceptance; never mock a successful live request or judgement.

The earlier DeepSeek batches remain historical evidence. Claude Haiku 5.5 closeout now has **11 human-reviewed positive question/feedback outputs, three rejected adversarial probes, and a separately verified epoll prerequisite follow-up**; the final affected test suite passed 54 checks, with one intentionally ignored Keychain round-trip. The earlier broader suite passed 74 checks. Failed attempts, scope corrections and final real-provider verdicts are preserved under `review/`.

Closeout work:

- [x] Strict generation-only answer terms, legacy cache compatibility and semantic review for synonymous hints/checklist scope.
- [x] Summaries derived from real ratings, independent/assisted counts and actual P2 stages; `missing` means insufficient independent evidence, not necessarily an unasked question or an error.
- [x] Preserve evidence supplied in any stage; check actual gap relevance rather than trusting a question citation.
- [x] Dedicated question/feedback review roles and bounded repair carrying the exact prior proposal plus critique; rejected/failed requests remain failures.
- [x] Final Claude live acceptance, including cross-stage reasoning, AI-assisted/mixed histories and adversarial hints, irrelevant gaps and uncertain-versus-missing probes.

See [calibration report](docs/AI_QUALITY_CALIBRATION.md) and [acceptance manifest](review/calibration-closeout-acceptance.json). This completes the agreed scripted baseline; it does not guarantee all future model judgements. Model review adds latency/API usage and can still reject valid content or miss defects. New policies do not rewrite old histories; final-policy DeepSeek semantics were not revalidated in this Claude-only closeout. No deferred feature below has started.

Existing evidence and defects:

- [P1 content expansion and live review](docs/P1_CONTENT_EXPANSION.md): visible answer leakage, optional checklist scope, AI-assisted ratings, and unasked material listed as gaps.
- [P2 content expansion and live review](docs/P2_CONTENT_EXPANSION.md): mischaracterized illustrative estimates, unsupported compression assumptions, and follow-ups that revisit a failure window already excluded by the answer.
- [Reliability follow-up](docs/RELIABILITY_2026-10-15.md): real-provider workflow acceptance and its limits.

## Deferred features, in suggested order

### 1. Voice answers with editable transcription

- [ ] Support speech input during P1 and P2 interviews.
- [ ] Show an editable transcript and let the user correct recognition mistakes before submission.
- [ ] Submit only after explicit user action, through the existing answer/follow-up workflow.

### 2. Review and weak-point tracking

- [ ] Summarize recurring mistakes, concepts to revisit and related exercises across interview history.
- [ ] Link each finding to the actual answer/feedback evidence and keep independent answers distinct from AI assistance.

### 3. Study plans and spaced review

- [ ] Turn weak points and past practice into a manageable review queue.
- [ ] Schedule spaced revisits and track whether previously weak concepts have improved.

### 4. Timed mock interviews

- [ ] Add timed practice to P1/P2, combining answers, follow-ups and a final review into an interview session.
- [ ] Make the time budget and progress visible while preserving drafts and existing recovery behavior.

### 5. API usage and cost records

- [ ] Record usage for question generation, follow-ups, reference answers and feedback by provider/model and interview.
- [ ] Show provider-reported token usage where available; distinguish cost estimates from billed amounts and unknown usage from zero usage.
- [ ] Keep credentials out of usage records and exports.

### 6. Interview exports

- [ ] Export questions, answers, follow-ups and feedback for later review.
- [ ] Include P2 design notes and diagram source, and preserve language and AI-assistance labels.
- [ ] Choose export formats when this feature is scheduled.

## Implementation constraints

Keep project documentation/comments in English, UI strings bilingual, and Astryx styling consistent. Preserve existing histories, drafts and LC complexity/resource policies. Keys remain provider-specific in the Git-ignored `.env` with mode `0600`; do not migrate to Keychain as part of these backlog items without the separately agreed migration step. Real API checks require intentional provider usage and honest reporting of failures and quality limitations.
