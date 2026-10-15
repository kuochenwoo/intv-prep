# P1 content expansion — 2026-10-15

## Scope

The local topic library now contains 40 bilingual seeds: the original ten stable IDs plus 30 new scenarios. Each seed still has one small operation sequence, two explicit constraints and three optional focus directions. The existing v3 fundamentals prompts, five-dimension feedback, AI-reference preview/submission workflow and P2 architecture workflow are unchanged. These are seeds for generated questions, not 40 pre-generated questions or a static answer bank.

The user-provided [Kamacoder notes](https://notes.kamacoder.com/) informed the breadth of fundamentals subjects; the new scenarios are independently authored and checked against the primary documentation linked in `content/deep-dive/review.json`. That companion file is an authoring review record, not content injected into candidate views or model requests.

## Coverage

| Category          | Topics |
| ----------------- | -----: |
| Caching           |      2 |
| Correctness       |      1 |
| Messaging         |      2 |
| Concurrency       |      6 |
| Consistency       |      1 |
| Availability      |      1 |
| Scalability       |      1 |
| Performance       |      1 |
| Database          |      7 |
| Networking        |      5 |
| Operating Systems |      4 |
| Runtime           |      4 |
| Redis             |      3 |
| Security          |      2 |

The expansion includes PostgreSQL snapshots, index scans, deadlocks, pagination, sequences and NULL; TCP framing and half-close; HTTP validators/cache policy and TLS identity; Linux file descriptors, unlink, epoll and fork; Java memory ordering, condition waits, CAS and thread pools; Java collections/resources and browser JavaScript; Redis pipelines, leases and eviction; SQL binding and CSRF. This is an initial broad fundamentals set, not comprehensive C++/Go/JVM/framework coverage.

## Content boundaries and sources

Assumptions are explicit rather than relying on implementation defaults. For example, PostgreSQL topics name version 18 and the relevant isolation level; Java topics name Java 21; event-loop topics specify a browser rather than Node.js; Linux-only process/I/O behavior is labelled. Redis scenarios avoid claiming a specific victim under approximate LRU or guaranteed pipeline isolation.

The following notes guide content review; they are not a required keyword checklist for candidate feedback.

| Topic ID                 | Review boundary                                                                                                                                                                                                                                         | Primary source                                                                                                                   |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `mvcc-snapshots`         | READ COMMITTED uses a fresh statement snapshot; do not generalize to every Repeatable Read implementation.                                                                                                                                              | [source 1](https://www.postgresql.org/docs/18/transaction-iso.html)                                                              |
| `composite-index`        | Equality on the leading key and a range on the next key define a contiguous interval; a usable index is not a promised planner choice. PostgreSQL 18 also supports skip scans, so do not claim every missing leading predicate makes an index unusable. | [source 1](https://www.postgresql.org/docs/18/indexes-multicolumn.html)                                                          |
| `covering-index`         | Covering all requested columns does not eliminate MVCC visibility checks; heap fetches depend on the visibility map.                                                                                                                                    | [source 1](https://www.postgresql.org/docs/18/indexes-index-only-scans.html)                                                     |
| `database-deadlock`      | Trace the circular wait; do not predict which transaction is selected as the victim.                                                                                                                                                                    | [source 1](https://www.postgresql.org/docs/18/explicit-locking.html)                                                             |
| `pagination-offset`      | Trace the inserted row with unique immutable ordering. Keyset pagination avoids this positional shift but does not create a cross-request snapshot.                                                                                                     | [source 1](https://www.postgresql.org/docs/18/queries-limit.html)                                                                |
| `transaction-rollback`   | Sequence advancement is not rolled back with row changes; a primary key need not be gapless.                                                                                                                                                            | [source 1](https://www.postgresql.org/docs/18/transaction-iso.html)                                                              |
| `tcp-framing`            | TCP preserves an ordered byte stream, not send boundaries; a parser must retain incomplete headers and bodies.                                                                                                                                          | [source 1](https://www.rfc-editor.org/rfc/rfc9293.html)                                                                          |
| `tcp-half-close`         | EOF after FIN concerns the receive direction. A server may still send a response on its open send direction.                                                                                                                                            | [source 1](https://www.rfc-editor.org/rfc/rfc9293.html)                                                                          |
| `http-conditional`       | 304 supports reuse of a matching stored representation and applicable metadata; it is not an empty replacement body.                                                                                                                                    | [source 1](https://www.rfc-editor.org/rfc/rfc9110.html#section-15.4.5), [source 2](https://www.rfc-editor.org/rfc/rfc9111.html)  |
| `http-cache-control`     | no-cache requires validation for reuse; no-store prohibits storage. Neither header should be described as equivalent to the other.                                                                                                                      | [source 1](https://www.rfc-editor.org/rfc/rfc9111.html#section-5.2.2.4)                                                          |
| `tls-identity`           | A trusted certificate chain does not establish that its identity matches the requested host; do not recommend disabling identity verification.                                                                                                          | [source 1](https://www.rfc-editor.org/rfc/rfc9110.html#section-4.3.4)                                                            |
| `file-descriptor-offset` | dup descriptors reference one open file description and share its offset; two independent opens differ.                                                                                                                                                 | [source 1](https://man7.org/linux/man-pages/man2/dup.2.html)                                                                     |
| `unlink-open-file`       | Removing the last link does not free the file while an open reference remains; filesystem/accounting delays are outside the seed.                                                                                                                       | [source 1](https://man7.org/linux/man-pages/man2/unlink.2.html)                                                                  |
| `epoll-edge-trigger`     | Unread bytes need not produce a new edge; use nonblocking draining to EAGAIN, without assuming readiness equals a complete message.                                                                                                                     | [source 1](https://man7.org/linux/man-pages/man7/epoll.7.html)                                                                   |
| `volatile-counter`       | A volatile read/write is not an atomic increment; join establishes visibility of completed work but does not repair lost updates.                                                                                                                       | [source 1](https://docs.oracle.com/javase/specs/jls/se21/html/jls-17.html)                                                       |
| `safe-publication`       | The observed volatile flag orders preceding writes; the seed deliberately excludes later mutations.                                                                                                                                                     | [source 1](https://docs.oracle.com/javase/specs/jls/se21/html/jls-17.html)                                                       |
| `condition-wait`         | wait releases the monitor and reacquires it before return; notifications do not reserve queue items, and spurious wakeups are permitted.                                                                                                                | [source 1](<https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/Object.html#wait()>)                          |
| `thread-pool-starvation` | All workers wait for queued children. Do not claim an unbounded queue causes more workers to be created beyond the configured core count.                                                                                                               | [source 1](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/concurrent/ThreadPoolExecutor.html)            |
| `cas-aba`                | Identity-only comparison cannot detect A-B-A history; a stamp helps only when updated consistently. This is not a native-memory reclamation question.                                                                                                   | [source 1](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/concurrent/atomic/AtomicStampedReference.html) |
| `javascript-event-loop`  | Use browser task and microtask semantics; do not import Node.js nextTick ordering or promise exact timer delays.                                                                                                                                        | [source 1](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Execution_model)                                    |
| `mutable-hash-key`       | The Map contract leaves behavior unspecified when mutations affect equality; explain why lookup becomes unreliable without promising that every such get must return null.                                                                              | [source 1](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Map.html)                                      |
| `resource-cleanup`       | With successful initialization, the body exception remains primary and close failure is suppressed; GC is not deterministic resource closure.                                                                                                           | [source 1](https://docs.oracle.com/javase/tutorial/essential/exceptions/tryResourceClose.html)                                   |
| `redis-pipeline`         | Pipelining reduces network waits and is not an isolation boundary; permit interleaving without asserting it occurs on every run.                                                                                                                        | [source 1](https://redis.io/docs/latest/develop/using-commands/pipelining/)                                                      |
| `redis-lock-lease`       | Compare the unique ownership token atomically before release. Safe release alone does not stop a stale owner from modifying an external resource.                                                                                                       | [source 1](https://redis.io/docs/latest/develop/clients/patterns/distributed-locks/)                                             |
| `redis-eviction`         | allkeys-lru can select keys without TTL and is approximate; do not guarantee that K is the exact victim.                                                                                                                                                | [source 1](https://redis.io/docs/latest/develop/reference/eviction/)                                                             |
| `sql-parameters`         | Bind values separately from SQL syntax; parameters cannot stand for arbitrary identifiers and do not replace authorization.                                                                                                                             | [source 1](https://cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet.html)                             |
| `csrf-cors`              | A simple cross-site form can send a credentialed request under the explicit cookie assumptions; blocking response reads does not prevent the state change.                                                                                              | [source 1](https://developer.mozilla.org/en-US/docs/Web/Security/Attacks/CSRF)                                                   |
| `sql-null`               | Ordinary comparisons with NULL produce unknown; a WHERE clause keeps only true rows.                                                                                                                                                                    | [source 1](https://www.postgresql.org/docs/18/functions-comparison.html)                                                         |
| `fork-copy-on-write`     | Private address spaces diverge on writes; copy-on-write is an implementation optimization, not shared-memory synchronization.                                                                                                                           | [source 1](https://man7.org/linux/man-pages/man2/fork.2.html)                                                                    |
| `async-exceptions`       | An async throw rejects the returned Promise, including before its first await; an outer synchronous catch does not catch that rejection.                                                                                                                | [source 1](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function)                          |

## Navigation and compatibility

P1 now offers keyword search, localized category filtering, a visible match count and an explicit empty state. Random selection draws only from the current matches; it is disabled when no topics match. A bounded, keyboard-focusable scroll region keeps interview history reachable with 40 topics. Filtering does not replace the selected topic, clear a draft, generate a question or make an API call.

Existing topic IDs, cached questions and sessions remain intact. Generation uses the selected seed in the current UI language; existing sessions keep their original wording and language. No database schema, provider setting, resource limit or credential migration is introduced.

## Validation

Automated checks protect all ten original IDs, uniqueness, bilingual content, small scenario shape, domain coverage, primary-source review coverage for every new seed, and English prompt isolation. Existing request-construction, streaming, reference-answer, persistence and failure-recovery tests remain applicable. Content checks validate the authoring contract; they do not prove every future generated question or judgement is technically correct.

The explicit `live_topic_expansion` example uses two new seeds in Chinese and English. Each sample makes real generation, reference-answer, follow-up and feedback requests. The reference is marked AI-assisted; feedback must not credit it as independent understanding. It reads the existing provider key without writing credentials or user history and writes `/tmp/intv-p1-expansion-live.json`.

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib --test p1
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-flash \
  cargo run --manifest-path src-tauri/Cargo.toml --no-default-features \
  --example live_topic_expansion
npm run build
```

Verification on 2026-10-15:

- Backend library and P1 integration tests: **27 passed, 0 failed, 1 intentionally ignored OS Keychain test**. This includes the expanded-content review check and bilingual/prompt-policy checks, plus existing reference, retry and persistence coverage. Logs: `/tmp/intv-p1-expansion-tests.log`.
- `npm run build` and native debug app packaging passed. Vite reports the existing large-chunk warning. Logs: `/tmp/intv-p1-expansion-build.log` and `/tmp/intv-p1-expansion-desktop.log`.
- Native computer-use acceptance verified all 40 topics, English and Chinese title search in the English UI, the five-topic Networking category, random selection within that category, topic focus/assumptions, and the zero-match state with random selection disabled. The dark layout keeps controls readable and the topic list bounded; existing interview history and an empty saved draft remained intact. Appearance, language, provider settings and credentials were not changed. No UI generation request was made.
- Live acceptance was initially blocked by automatic approval review over the new content payload. After the user explicitly approved these two topic payloads and their generated answers, the same probe was submitted for execution. The approved probe completed as described below.

## Live sample review

The explicitly approved DeepSeek `deepseek-flash` run completed eight real logical requests: generation, reference answer, follow-up and five-dimension feedback for `mvcc-snapshots` in Chinese and `javascript-event-loop` in English. Both generated questions used the selected small scenario rather than a complete architecture task; the reference explanations correctly described statement snapshots and browser log order A, D, C, B. Logs: `/tmp/intv-p1-expansion-live.log`; complete generated content: `/tmp/intv-p1-expansion-live.json`. No application history or credentials were written.

Transport and schema completion passed, but semantic quality did **not** fully pass:

- The English event-loop question's visible constraints already explained the queue rules and its question stated the final output order. This weakens the assessment even though the seed itself does not give the answer.
- The Chinese generated checklist included a Repeatable Read comparison not asked in the opening question; optional extensions should not become mandatory opening coverage.
- Both feedback samples disclosed AI assistance but still assigned some `good`/`partial` ratings to AI-only evidence, contrary to the existing policy that such dimensions should be `missing`. The English sample also put unasked selection rationale in `gaps` and discounted an explanation that already stated the checkpoint order.

These are recorded model-output defects, not claims of user weakness or successful semantic acceptance. The content-expansion change deliberately retains the current prompt and feedback implementation. The next AI-quality iteration should address answer leakage, optional-checklist scope, and reliable exclusion of AI-assisted evidence, with regression samples from this report. The 40 authored seeds and local navigation are complete; broad semantic certification across all topics remains out of scope for this content batch.
