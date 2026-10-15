# Project guidelines

Follow `SWE_Interview_Prep_Requirements.md` and the user's currently authorized scope. Proceed through P0 → P1 → P2 → P3 without implementing later phases early.

## Requirements for every future LC problem

- Every new or updated LC problem must specify time and auxiliary space complexity in the problem JSON's `complexity` field and display it in the problem details.
- Define size variables, whether input storage counts toward space, and applicable assumptions about recursion stacks, in-place modification, and average/worst-case complexity.
- Check the official statement or editorial and record source links. Distinguish explicit official requirements from local training goals. Never present a recommended optimal complexity as an official mandatory requirement.
- Every problem must provide `limits.timeLimitMs`, `limits.memoryLimitMb`, and `limits.outputLimitKb`; the Judge must read and enforce them. Choose budgets using the problem's input scale and real execution results rather than mechanically copying existing values.
- Do not invent unverified official millisecond/MiB quotas. Clearly label local budgets. Measured execution time, process RSS, and asymptotic complexity are different metrics; finite tests cannot automatically prove arbitrary code's Big-O or in-place behavior.
- Function and ACM modes must share semantic tests, including maximum-size or representative worst-case inputs and necessary resource-limit regression tests. Expected answers must be verifiable. Never mock successful compilation, execution, or judging.
- Integrate new problems into the problem library and common policy checks. Verify complexity metadata, resource settings, the `io` spec (it generates the Function-mode wrapper, ACM stdin, and checking; see `tools/leetcode/README.md`), and correct solutions in both modes before considering a problem complete.

## Current phase status

P0 is complete. Preserve all LC complexity and resource-limit requirements. The user authorized P1, which now includes Claude / DeepSeek Technical Deep Dive UI, caching, interviews, feedback, and failure recovery. The currently requested model is Claude Haiku 5.5 (`claude-haiku-5-5`). Claude has passed real question generation, follow-ups, five-dimension feedback, and retry recovery. DeepSeek's real multi-turn acceptance remains affected by HTTP 503 / keepalive timeouts; do not claim that DeepSeek has passed. The user authorized P3's Hot 100 problem coverage: all 100 problems are bundled with complexity requirements and reference solutions verified in both modes (see `tools/leetcode/README.md` for authoring rules). The user authorized P2 on 2026-10-15: system design now has five bilingual scenarios, user-controlled stages, Markdown/Mermaid notes, persistent retry recovery and eight-dimension feedback. Keep P1 as it is; defer further P1 knowledge-library changes to the later content-expansion phase.

The user explicitly requested storing keys in a Git-ignored `.env` with mode 0600 for testing, before migrating to the system Keychain after tests pass. The current runtime only reads/writes `.env`; do not automatically fall back to system storage or migrate early. Preserve separate keys for each provider. Never commit `.env`, print real keys, or write keys to SQLite.

The user later changed the reference-answer feature (2026-10-15): the "AI 参考回答" button makes one request and shows one concise answer (150–300 characters, 3–5 key points) under the current question, without submitting it or asking a follow-up. Under that answer, the user can copy it into their draft, or explicitly submit it as an `aiAssisted` turn and continue. Feedback must not treat `aiAssisted` turns as evidence of the user's independent understanding. Retrying a failed follow-up must not regenerate or append another AI answer.

## Language conventions

Write project documentation, developer descriptions, and code comments in English. The UI is bilingual: every user-facing UI string lives in `src/i18n.tsx` with a Chinese and an English version, and the header button switches language. Content follows the UI language too: every problem JSON has an `en` block (title, tags, statement, constraints, complexity text, `acmIoSpec`), each topic in `content/deep-dive/topics.json` has an `en` block, the AI commands take `lang` and use the matching prompts in `src-tauri/src/llm.rs`, and backend errors stay Chinese in Rust but are translated for English mode by the table in `src/errors.ts` (add new errors there). Code comments inside starter templates follow the English-comment convention.

## UI

The frontend uses Meta's Astryx design system (`@astryxdesign/core`, neutral theme, pinned to 0.7.0) with a violet accent and light/dark modes. Use Astryx components and its CSS tokens (`var(--color-...)`) instead of hard-coded colors.
