# Practical fundamentals and system-design coverage — 2026-10-16

## Development scope and status

This iteration implements the user's request to make P1 resemble practical backend technical interviews, using the non-Java-specific scope of [JavaGuide](https://javaguide.cn/home.html) and its public [scenario-question preview](https://javaguide.cn/zhuanlan/back-end-interview-high-frequency-system-design-and-scenario-questions.html). The user subsequently authorized complete versions of large design problems in P2, with concrete technical questions split into P1.

**Implementation only.** At the user's request, no automated tests, build, native UI acceptance or provider requests were run for this iteration. Test definitions were updated for the new contracts; they have not passed a run. Earlier calibration and acceptance reports apply to their recorded content/policies, not automatically to these changes.

The material is independently authored topic/scenario seeds, not copied article answers, a reproduction of the paid collection, or an assertion of exhaustive JavaGuide coverage. Source URLs are reading/authoring references, not proof of model-generated correctness. No scraping pipeline, retrieval database, provider or framework was introduced.

## P1: concrete technical questions

The library grows from 40 to **120 bilingual topics**: 40 foundation openings, 68 applied scenarios and 12 troubleshooting openings. Of these, 94 are marked core and 26 further practice. These priorities are local editorial learning order, not an official ranking or measured interview frequency.

| Area                        | Topics | Representative coverage                                                                                                           |
| --------------------------- | -----: | --------------------------------------------------------------------------------------------------------------------------------- |
| Databases                   |     19 | B+ trees, primary/secondary indexes, ACID, isolation, logs, locks, query plans, schema changes, connection pools, aggregation     |
| Networking                  |     14 | HTTP semantics and versions, DNS, TLS, TCP reliability/framing, connection states, WebSocket/SSE                                  |
| Operating systems           |     10 | Processes/threads, memory, I/O models, page cache, filesystem behavior, CPU/load, zero-copy                                       |
| Concurrency                 |     10 | Read-modify-write, publication, condition variables, worker pools, locks, deadlock, cancellation, backpressure                    |
| Redis                       |     11 | Data types, command execution, persistence, cache incidents, hot/large keys, sorted sets, scripts, cluster routing                |
| Messaging                   |      8 | Reasons for queues, acknowledgements, backlog, Kafka rebalancing, dead letters, outbox, ordering and redelivery                   |
| Security                    |      7 | Sessions/tokens, OAuth identity boundaries, password storage, XSS, SSRF, SQL parameters, CSRF/CORS                                |
| Distributed fundamentals    |      8 | CAP, consistency histories, consensus, leases/fencing, IDs, consistent hashing, retries, compensation                             |
| Applied data structures     |      5 | Hash collisions, LRU/LFU, Bloom filters, top K, bitmap deduplication                                                              |
| Engineering                 |      6 | Telemetry, cardinality, latency distributions, compatible releases, test boundaries, incident evidence                            |
| Business scenarios          |      7 | Order expiration, upload retries, OTP verification, payment callbacks, unique visitors, unread cursors, deduplication permissions |
| Search/storage              |      4 | Inverted indexes, refresh visibility, object APIs, document models                                                                |
| Languages/runtime           |      4 | JavaScript scheduling/exceptions, generic hash-key contracts, deterministic resource cleanup                                      |
| Existing service categories |      7 | Caching, idempotency, inventory correctness, replication, migration and retry amplification                                       |

Java language/framework internals, JDK collections thresholds, JVM/GC configuration, JMM trivia, Spring/AQS and API memorization are excluded. Seven existing Java-shaped seeds now use explicit portable teaching contracts; their IDs remain stable, including `volatile-counter`. That ID now represents atomic loads/stores with a non-atomic compound increment, not an assertion about C++ `volatile` or unspecified data-race results. Generic concepts such as CAS, publication, lock waits and worker starvation remain useful across languages. Existing JavaScript topics remain.

### Opening and follow-up policy

- `foundation`: ask a natural concept, mechanism or closely related comparison. A neutral subject name such as B+ tree, MVCC or TCP is legitimate in the question. Do not force every concept into a contrived timing puzzle.
- `scenario`: give one concrete work situation and ask about a local mechanism, invariant or decision.
- `troubleshooting`: supply symptoms and known evidence; ask how to test one hypothesis without assuming an unproven root cause.

Each opening still has one coherent objective. Optional topic focus and follow-up directions are not mandatory opening criteria. A foundation answer can progress to an application or boundary; not every correct answer requires another timestamp list or counterexample. Unasked dimensions remain `missing`. Reasonable unverified diagnostic hypotheses are not automatically errors; assess the requested evidence-gathering steps.

`answerTerms` remain private and enforced by the existing lexical guard. They now describe **answer propositions or causal explanations**, excluding the neutral subject being asked about. The semantic reviewer uses the same distinction and still rejects wording-level answer hints and out-of-scope mandatory criteria. Feedback keeps the five existing dimensions, independent-evidence restrictions and bounded review/repair. The policy version advances to `separate-task-v8-diagnostic-openings`; this is not a claim of live acceptance.

### Diagnostic openings must not be taxonomy matching

The user found that the cache-failure topic title and an opening comparing two fully specified canonical triggers made the task too leading. `redis-cache-failures` keeps its stable ID but now has the neutral visible title “Cache Misses and Database Pressure” and uses troubleshooting. Penetration, hot-key expiry and mass expiry remain private diagnostic directions; simply reciting these labels is not required.

Open with one incident and limited observations, such as a reduced cache hit rate and increased database query volume. Ask which additional evidence would distinguish causes and why. Do not pre-supply key nonexistence, hot-key expiry or every alternative as a diagnosis menu. Provide further exercise observations in response to the candidate's requested evidence, labeling assumptions rather than pretending to have actual production data. If the question asks for a definite cause, it must supply enough evidence; investigation questions may honestly leave the cause unresolved. Omit environment details unrelated to the mechanism being asked about.

The author and reviewer share this distinction: naming an ordinary learning subject is allowed; exposing a diagnosis that the candidate is supposed to infer is not. This correction does not turn every concept question into a puzzle or prohibit useful direct comparisons. Existing cached questions remain unchanged, so regenerate to exercise this policy. No tests or live requests were run for this correction.

### Navigation

The sidebar adds question-style and practice-priority selectors alongside search/category. Initial practice shows core topics with foundation openings first, starting at `mysql-index-structure`. Clear filters exposes the full library; random selection respects every active filter. Explicit Review navigation clears the new filters so advanced or troubleshooting review targets remain reachable. Filtering itself makes no API request and does not clear interview drafts.

Only neutral style/priority labels accompany titles. Private focus, constraints and evaluation criteria are not exposed as preparation hints. All new labels live in the bilingual `src/i18n.tsx` dictionary and use existing Astryx components.

## P2: complete system design

P2 grows from 15 to **20 bilingual scenarios**. The five new exercises are:

| ID                  | Complete design                              | Capacity and cross-component focus                                                                          |
| ------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `order-lifecycle`   | Orders, payments and expiration              | Order rate/history growth; inventory/payment authority, deadlines, late outcomes and reconciliation         |
| `leaderboard`       | Real-time leaderboards and season settlement | Rank-entry memory, event traffic; authoritative events, ranking views, partitioning and season recovery     |
| `identity-session`  | Identity, sessions and authorization         | Login versus authorization traffic, session state; trust boundaries, revocation, keys and degraded service  |
| `payment-ledger`    | Wallets, withdrawals and reconciliation      | Entry retention and hot accounts; ledger invariants, external uncertain outcomes, holds and audit recovery  |
| `visitor-analytics` | Event ingestion and real-time visitors       | Ingestion and raw retention, dimension cardinality; streaming/offline paths, windows, sketches and deletion |

Scale/SLA values are explicitly local training assumptions, not official production quotas. Capacity examples live in `content/system-design/review.json` and distinguish base data from indexes, replication, traffic peaks and average rates. Existing five stages, estimation/architecture depth and eight feedback dimensions remain unchanged; no P1 policy is added to the P2 interviewer system.

### Shared business context, different interview scope

These are authoring relationships rather than a new cross-module navigation feature:

| P2 complete scenario                      | P1 concrete topics to practice first                                                                    |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Orders/payment expiration; ticket booking | `business-order-expiry`, `database-concurrency`, `business-payment-callback`, `mq-outbox`               |
| Wallets and withdrawals                   | `transaction-acid`, `idempotency-retries`, `business-payment-callback`, `distributed-saga`              |
| Leaderboards                              | `redis-zset-ranking`, `redis-data-types`, `mq-poison-message`, `ds-top-k-stream`                        |
| Identity/session platform                 | `auth-cookie-session-token`, `auth-oauth-oidc`, `business-otp-login`, `security-password-storage`       |
| Visitor analytics                         | `business-uv-count`, `ds-bitmap-dedup`, `ds-bloom-filter`, `engineering-cardinality`                    |
| File storage/sync; video upload           | `business-upload-resume`, `business-content-dedup`, `storage-object-local`, `os-page-cache`             |
| Chat and notification platforms           | `business-unread-cursor`, `network-websocket`, `mq-publisher-confirm`, `message-delivery`               |
| Metrics platform                          | `engineering-cardinality`, `engineering-p99`, `engineering-logs-metrics-traces`, `mq-backlog-diagnosis` |
| Distributed rate limiting                 | `rate-limiting`, `distributed-cap`, `distributed-lease-fencing`, `distributed-retry-budget`             |
| Search/autocomplete                       | `search-inverted-index`, `search-refresh-delay`, `ds-top-k-stream`                                      |
| Task queues                               | `concurrency-backpressure`, `concurrency-cancellation`, `mq-poison-message`, `mq-kafka-rebalance`       |

P1 asks, for example, how one duplicate payment callback avoids a second ledger effect. P2 asks how order state, payment providers, authoritative records, asynchronous delivery, queries and recovery fit together at scale. The business name alone does not determine the module; the scope of the actual question does.

## Compatibility and follow-up work

All original P1/P2 IDs are retained. No database migration, deletion, cache rewrite, transcript replacement or model/key change is performed. Previously cached questions retain their original wording; use Generate a question to exercise the new policy. Topic metadata has backward-compatible scenario/core/empty-source defaults for authored legacy JSON. Sources stay in local content and scoped authoring context, never credential storage.

Before calling this update verified, run the affected Rust/content contracts, TypeScript/native builds and bilingual UI checks, then separately review representative foundation/scenario/troubleshooting outputs and P2 estimation/feedback. Preserve the user's provider boundary: OpenAI only for voice; DeepSeek for authorized text testing. Paid acceptance requires a deliberately scoped batch, not an automatic consequence of adding seeds.
