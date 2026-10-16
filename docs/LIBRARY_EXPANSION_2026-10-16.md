# Broader interview library and P1 question acceptance

The user authorized additional P1/P2 content and broad P1 testing. This iteration adds **60 independently authored bilingual P1 seeds** and **six complete P2 scenarios**. No voice tests were performed. Text testing started with DeepSeek and switched to Claude after a real DeepSeek connection/TLS failure, without changing application model settings or credentials.

## Content coverage

P1 now contains **180 topics across 19 categories**: 65 foundation, 98 scenario and 17 troubleshooting openings; 146 core and 34 advanced topics. A topic is a generation seed, not a promise of one fixed pre-generated question. The 13 broader technical categories cover:

| Category | Total | Added coverage |
| --- | ---: | --- |
| Database | 25 | Unique constraints, foreign keys, window functions, vacuum investigation, prepared plans, restore verification |
| Networking | 20 | TCP flow/congestion control, handshake, connection reuse, L4/L7 balancing, CDN variants, reused-connection failures |
| Redis | 16 | Replication acknowledgement, Sentinel, WATCH, SCAN, memory investigation |
| Operating Systems | 15 | Zombie/orphan processes, signals, mmap, context switches, accept failures |
| Concurrency | 14 | Semaphores, barriers, false sharing, bounded-buffer shutdown |
| Messaging | 13 | Offset commits, acks/ISR, transaction scope, retention/compaction, partition growth |
| Distributed Systems | 12 | Clocks, discovery freshness, conditional config updates, quorum limits |
| Security | 11 | Object authorization, refresh rotation, signatures/HMAC, path resolution |
| Engineering | 11 | Containers, CPU limits, health probes, shared Git history, schema evolution |
| Business Scenarios | 11 | Coupon redemption, callback replay, reservation release, cursor pagination |
| Runtime | 9 | Reachability/resources, shallow copies, generators, blocking async work, monetary arithmetic |
| Data Structures | 9 | Arrays/lists, amortized growth, tries, external sorting |
| Search and Storage | 7 | LSM compaction, analyzers/exact fields, deletion lifecycle |

The six original service categories remain: Caching (2), Correctness, Consistency, Availability, Scalability and Performance (1 each). Java-only framework/runtime trivia remains excluded; existing portable concurrency and JavaScript topics remain.

P2 now contains **26 bilingual scenarios**. New scenarios are product catalog/search, configuration/feature flags, centralized logs/search, durable workflows, distributed key-value storage and live streaming. Each has functional scope, scale assumptions, reliability boundaries, architecture directions, primary reading references and a checked arithmetic example. Scale/SLA values are local exercise assumptions. Source payload, indexes, replication, compression, fanout and peaks must be estimated separately.

Reading references are recorded in the content review JSON files, including PostgreSQL, Redis, TCP/HTTP RFCs, Kafka, Linux man-pages, Python, Docker, Kubernetes, Protocol Buffers, RocksDB, Temporal and OpenFeature documentation. They support authorship; they do not certify model-generated answers or reproduce a paid interview collection.

All original IDs remain. Only the two legacy RPC/replica seeds change: they no longer state the causal answer in their assumptions, and optional follow-ups are labeled. Existing cached questions, transcripts and scores are not rewritten. Generate a new question to use updated seeds/policies.

## Automated and native checks

**95 backend checks and 11 frontend checks passed**; one Keychain round-trip remains intentionally ignored. The affected Rust suite covers all seed metadata and bilingual localization, 360 P1 and 52 P2 offline question-cache round-trips, interview recovery, exports/timers/accounting and localhost transport fixtures. These fixtures are storage/transport tests, not fake successful provider requests. Frontend checks cover navigation ownership, Markdown and existing review helpers.

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features \
  --lib --test p1 --test p2 --test reliability --test session_tools
npm run test:review-ui
npm run build
```

Frontend production build and final native debug bundle passed. Vite retains the existing large-chunk warning. Start development with `npm run tauri dev`. The isolated acceptance bundle is `/tmp/intv-calibration-check-target/debug/bundle/macos/SWE Interview Prep Library QA.app`.

Native computer-use checks at 1280x850 confirmed 180 topics, 146 default core topics, clear-all/advanced search, retained folded filters, bilingual selection, all 26 P2 scenarios and workflow assumptions. Two real generated code questions were imported into the isolated `local.swe.interview-prep.library-qa` cache for Markdown checks. Offline interview start, saved draft, switching to another topic without stale conversation, language-specific cache visibility and restoring the correct topic/draft from the history drawer all worked. There were zero UI provider requests or microphone actions. Personal application data was untouched.

## Real provider results and limits

The bounded batch made **129 actual HTTP attempts of a 180-attempt maximum**:

| Provider/model | Success | Content error | Transport error |
| --- | ---: | ---: | ---: |
| DeepSeek / `deepseek-flash` | 4 | 0 | 1 |
| Claude / `claude-haiku-5-5` | 109 | 15 | 0 |

These are per-attempt accounting statuses, not independent quality ratings. Content errors include bounded format/validation retries; model reviewer rejections and manual rejections are recorded separately. There is no configured price snapshot in this isolated ledger, so costs stay unknown; provider token counters are retained without guessed pricing. No OpenAI or voice calls occurred.

The initial batch generated 30 questions across all 19 categories (15 Chinese/15 English; 9 foundation/18 scenario/3 troubleshooting). Subsequent checks produced 14 targeted replacements and seven standalone review probes, for **44 positive generations and seven review-only operations**. The initial broad batch used v10; premise rechecks used v11; scope/seed rechecks used v12. Only listed samples were tested with each policy.

Manual review of visible questions and private mandatory criteria found:

- An impossible duplicate-registration incident despite an active UNIQUE constraint. The corrected reviewer rejected the original, and both replacement openings have consistent premises.
- A diagnostic question listing the views/counters the candidate was meant to propose. The original was rejected; the visible menu was removed and the topic title made neutral.
- Consumer-only shutdown requiring producer handling, stale-read questions requiring replication-position routing, and retry-count questions requiring cancellation remedies. Review rules now check each mandatory criterion against the actual visible ask; extensions belong in private follow-ups.
- Ambiguous cumulative CDN criteria. The clearer replacement asks about response headers and does not force every alternative caching strategy.

**Do not call all outputs passed.** Of 44 positive generations, manual dispositions are 30 accepted, five rejected, one superseded and eight needing scope refinement. Several are older intermediate versions of the same topics. Six of seven standalone probes were rejected; the reviewer accepted the conditional CDN branch, so it is not a seven-for-seven result. Full per-output dispositions and original payloads are preserved in [the manifest](../review/p1-library-acceptance-2026-10-16.json).

Residual examples remain: a free-choice PostgreSQL investigation can still receive a mandatory old-transaction criterion; a replica-read opening may demand an unasked trade-off; a retry opening may demand idempotency without explicitly asking for a remedy. The final English retry sample and Chinese replica sample align with their questions, but that does not erase other failures. Model self-review remains fallible. The application does not promote these test outputs into the personal cache.

This completes content expansion, broad question-library testing and local regression, with documented semantic limits. It does **not** re-certify all 180 topics, every future generation, P1 feedback across all new topics, or live five-stage P2 interviews for the six additions. Previous multi-turn/calibration reports retain their own scope. The newest P2 scenarios have content, offline persistence and native selection coverage; their live acceptance remains a separate TODO.

The example `src-tauri/examples/live_p1_library_expansion.rs` refuses to overwrite completed batch reports/databases. Its explicit continuation modes share the 180-attempt ledger cap. Do not rerun paid batches automatically.
