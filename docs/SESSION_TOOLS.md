# Interview session tools

Delivered: 2026-10-16 (Asia/Tokyo).

## Scope and operation

P1 and P2 share a small session toolbar. No voice input, weak-point tracking, study planning or spaced review was added.

- **Exports:** Markdown for reading and versioned JSON for structured review. Use the export buttons in a selected interview; files download through the native webview. They include the saved question, constraints, submitted answers, follow-ups, reference previews, feedback, provider/model, AI-assistance labels, mock timing and that session's usage. P2 also includes saved design notes and literal Mermaid source. Unsubmitted drafts, private generation checkpoints/directions/answer terms, settings and credentials are excluded. The UI language controls export headings, while original interview text and P2 language remain intact. P1's legacy question language is detected from the text, not a historically stored language field.
- **Hint/scope cleanup:** the existing fresh-question lexical checks and bounded semantic review remain in force. P1 seed focus and scenario direction lists no longer appear in the preparation panel; P2 optional deep-dive directions are hidden. Necessary P2 requirements/scale/failure assumptions remain available. P1's draft placeholder asks only for the current mechanism/execution steps. Existing caches and histories are not silently rewritten or claimed to be retroactively reviewed. Explicitly requested reference answers remain available as learning aids. Private metadata stays out of candidate streams and exports. Opening-question drafts are not displayed before validation/review returns; follow-ups and explicitly requested reference answers still stream. The APIs themselves continue to use real streaming responses.
- **Timed mock practice:** default off. Select 10, 20 or 30 minutes, then explicitly press Start. Pause/resume and reset work locally. Reset returns to the selected duration awaiting another Start; it clears that timer's previous interval. A running countdown uses a persisted wall-clock deadline, so navigation, relaunch and computer sleep do not grant extra time; a paused clock preserves the remaining duration. Answers/follow-ups/final feedback use the existing durable session workflow. Submitted-turn count and P2 stage remain visible. Expiry is advisory: preserve drafts and let the candidate finish; never auto-submit or call a paid API. Starting final feedback atomically stops the timer before the network request, including when feedback later fails. No percentage ability score or timeout penalty is invented. Deleting a history also removes its clock. As with P0, changing the system clock affects wall-clock timing.
- **Usage/cost accounting:** one durable SQLite row per HTTP attempt, including question generation, follow-ups, reference answers, feedback, format retries, semantic reviews and repairs. Provider/model, module, session, purpose, reviewer flag, UTC timestamp and request outcome are recorded. A request row is inserted before sending; a crash leaves it interrupted/unfinished rather than pretending success. Cache-only actions create no HTTP rows. Successful generation is assigned to the first interview using its question; subsequent cached reuse incurs no new generation charge. Failed generation stays in the global ledger. The ledger is independent of interview history deletion, retains no conversation text, and is not backfilled from earlier requests.

Open the collapsed **API usage and costs** panel for the current interview or all requests. The table shows at most the newest 100 records; totals include every row in the selected scope. Rates are entered per provider/model, in USD per million tokens for uncached input, output, cache reads and cache writes. There are no hard-coded prices. Each HTTP row snapshots the then-current rate configuration; changing a rate does not rewrite historical estimates. Missing rates/counters or incomplete streaming leave cost unknown. Zero is a reported/configured zero, not a replacement for unknown data. The known subtotal is explicitly partial when any estimates are unknown. Provider invoices, taxes, currency conversion, promotions and subscription balances are not available through this ledger.

The request status is transport/content validation status, not a claim of semantic correctness. Review calls are separate charged requests. Content errors can retain complete provider usage. Interrupted streaming may retain partial reported counters, but the application does not estimate a final charge from incomplete usage. A failed local ledger write surfaces as an error; it never silently drops an attempt or substitutes invented tokens.

## Provider contracts

- [Claude streaming documentation](https://platform.claude.com/docs/en/build-with-claude/streaming): merge `message_start.message.usage` with `message_delta.usage`; output token counters are cumulative snapshots and must not be summed. Claude's uncached input excludes cache reads/writes. Only a completed `message_stop` marks the streamed usage complete.
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/): request `stream_options.include_usage`; retain the final usage chunk even when `choices` is empty. Prompt tokens include cache hits, so subtract hits before applying the uncached input rate. `[DONE]` marks completion. Cache fields follow the [context caching contract](https://api-docs.deepseek.com/guides/kv_cache/).

A provider omitting usage leaves unknown counters, including zero-token HTTP/auth failures. No character-count or locally tokenized approximation is presented as provider-reported usage. Keys remain provider-specific in the ignored `.env`; no Keychain migration or SQLite key storage was added.

## Validation

Automated checks cover timer state transitions/expiry/restart/reset, atomic feedback completion, draft preservation, export privacy and P2 notes/diagram/assistance, generation association/cache reuse, cache price arithmetic, rate snapshots, missing usage, cumulative stream counters, incomplete streams and charged content failures. Existing P1/P2 calibration and recovery tests remain in the affected suite. Local HTTP fixtures explicitly test protocol behavior; they are not represented as successful live model calls.

A separately authorized live DeepSeek check used only the authored English JavaScript event-loop topic, with a shared cap of 12 HTTP attempts. It completed in **2 attempts** (generation plus quality review). Actual provider usage:

| Request | Input | Output | Cache read | Outcome |
| --- | ---: | ---: | ---: | --- |
| Generation | 1,155 | 390 | 512 | Request/content validation succeeded |
| Review | 756 | 11 | 0 | Accepted |

The run used an isolated ignored SQLite file and deliberately illustrative rates (not official prices) to verify cost math. It did not change app settings, configured rates, keys or personal history. The sanitized result, raw question metadata and reported counters are in [the real usage artifact](../review/session-features-live-usage.json). This minimal run tests accounting; it does not broaden the earlier calibration acceptance to all DeepSeek outputs.

Desktop computer-use checks verified default-off selection, explicit Start, countdown, pause, cached reuse without a request and an actual Markdown file download. The final affected suite passed **66 tests** (38 library, 11 P1, 12 P2, five session-tool tests), with one intentionally ignored Keychain test. Frontend TypeScript/Vite and native debug `.app` builds passed. Vite retains the pre-existing large-chunk advisory. After a full app relaunch, the P1 clock remained paused at its previous remaining time. An actual P2 JSON download was also inspected: original English language and notes/diagram keys were retained, with no private question fields or drafts. No UI action submitted an answer or requested AI feedback.

The final app bundle is `src-tauri/target/session-tools/SWE Interview Prep.app` (Git-ignored). Local validation logs: `/tmp/intv-features-tests-final.log`, `/tmp/intv-features-frontend-final.log`, `/tmp/intv-features-desktop-final.log`.

## Run

From the repository root, use `npm run tauri dev`, or build with `npm run tauri -- build --debug --bundles app`. Node dependencies and the Rust/macOS desktop prerequisites must already be installed. In this machine's isolated validation setup, Cargo uses `/tmp/intv-cargo` and `/tmp/intv-rustup`; those are validation paths, not product requirements.

The explicit paid usage check is `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_usage`. Run only intentionally with an existing DeepSeek `.env` key and a fresh/archived evaluation ledger: it sends the authored topic and can incur charges. The script never selects personal histories or changes app settings.
