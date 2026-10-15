# Project guidelines

Follow `SWE_Interview_Prep_Requirements.md` and the user's currently authorized scope. Proceed through P0 → P1 → P2 → P3 without implementing later phases early.

## Requirements for every future LC problem

- Every new or updated LC problem must specify time and auxiliary space complexity in the problem JSON's `complexity` field and display it in the problem details.
- Define size variables, whether input storage counts toward space, and applicable assumptions about recursion stacks, in-place modification, and average/worst-case complexity.
- Check the official statement or editorial and record source links. Distinguish explicit official requirements from local training goals. Never present a recommended optimal complexity as an official mandatory requirement.
- Every problem must provide `limits.timeLimitMs`, `limits.memoryLimitMb`, and `limits.outputLimitKb`; the Judge must read and enforce them. Choose budgets using the problem's input scale and real execution results rather than mechanically copying existing values.
- Do not invent unverified official millisecond/MiB quotas. Clearly label local budgets. Measured execution time, process RSS, and asymptotic complexity are different metrics; finite tests cannot automatically prove arbitrary code's Big-O or in-place behavior.
- Function and ACM modes must share semantic tests, including maximum-size or representative worst-case inputs and necessary resource-limit regression tests. Expected answers must be verifiable. Never mock successful compilation, execution, or judging.
- Integrate new problems into the problem library and common policy checks. Verify complexity metadata, resource settings, the `io` spec (it generates the Function-mode wrapper, ACM stdin, and checking; see README "Adding a problem"), and correct solutions in both modes before considering a problem complete.

## Current phase status

P0 is complete. Preserve all LC complexity and resource-limit requirements. The user authorized P1, which now includes Claude / DeepSeek Technical Deep Dive UI, caching, interviews, feedback, and failure recovery. The currently requested model is Claude Haiku 5.5 (`claude-haiku-5-5`). Claude has passed real question generation, follow-ups, five-dimension feedback, and retry recovery. DeepSeek's real multi-turn acceptance remains affected by HTTP 503 / keepalive timeouts; do not claim that DeepSeek has passed. Do not start P2 or P3.

The user explicitly requested storing keys in a Git-ignored `.env` with mode 0600 for testing, before migrating to the system Keychain after tests pass. The current runtime only reads/writes `.env`; do not automatically fall back to system storage or migrate early. Preserve separate keys for each provider. Never commit `.env`, print real keys, or write keys to SQLite.

The user clarified the P1 reference-answer feature: clicking the AI-assisted answer and follow-up button must make the model answer the current original question or follow-up, automatically submit that answer, and continue the interview. Mark these turns with `aiAssisted`; feedback must not treat them as evidence of the user's independent understanding. Preserve draft protection. If the follow-up fails after submission, retries must not regenerate or append another AI answer. Do not implement this button as a read-only reference panel.

## Language conventions

Write project documentation, developer descriptions, and code comments in English. Keep user-facing UI text, problem statements, interview content, and user-facing error messages in Chinese. Code comments inside starter templates follow the English-comment convention.
