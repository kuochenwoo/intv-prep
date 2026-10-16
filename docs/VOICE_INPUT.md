# Voice input

Implementation and core acceptance: 2026-10-16. The user subsequently authorized testing with OpenAI for voice and DeepSeek for text. The original implementation-only delivery made no requests; the results below are from the later authorized run.

## Operation

P1 and P2 active answer panels share `VoiceInput.tsx`:

1. Click the small microphone icon next to the answer field to record locally. The OS may request microphone access. Starting recording does not call OpenAI.
2. Click the same icon, now showing Stop, and optionally listen to the temporary recording.
3. Click **Transcribe with OpenAI (online)** to send this recording to the transcription API. Each click is one potentially billable attempt; there is no automatic retry.
4. Edit the transcript preview, checking Chinese/English terms, omissions and recognition errors.
5. Explicitly append it to the current answer draft. Existing draft text is preserved; combined answers over 6,000 characters require editing instead of silent truncation.
6. Submit the answer through the existing interview controls. Transcribing/appending never submits it, advances a stage or requests feedback.

Recording/transcription temporarily disables other actions for that interview. P2 previews are tied to the current session/stage. Leaving the module stops capture; returning to the same session can retain the temporary clip. Changing interviews/stages or closing the app clears audio and uninserted previews. Re-recording replaces temporary content. The inserted draft uses existing local autosave/retry behavior. Voice recognition does not mark an answer as AI-assisted: it transcribes the candidate's speech rather than generating an answer.

Recordings are limited to five minutes and 20 MiB. The frontend negotiates WebM/Opus, MP4 or Ogg recording depending on browser support. Audio and previews remain in memory, with no audio file or raw transcript written by the transcription backend. Object URLs and microphone tracks are released on cleanup. Size overflow rejects the clip; reaching the time limit stops recording without sending it.

## Provider and credentials

The Rust `transcription.rs` path calls only `https://api.openai.com/v1/audio/transcriptions`, with fixed model `gpt-4o-mini-transcribe`, JSON output and a generic language hint. It sends no question, history, private checkpoint, reference answer, source code or draft text. The language hint follows the UI language, while automatic audio-language detection allows mixed speech. There is no LLM cleanup/answer-generation step.

`OPENAI_API_KEY` is stored in the Git-ignored project `.env`, mode 0600, with separate Claude and DeepSeek entries preserved. Settings has an independent transcription key field; it exposes key status, never the stored key. The interview provider selector and `LlmSettings::validate` still accept only Claude/DeepSeek. **The OpenAI key must never be used for interview question generation, follow-ups, reference answers, feedback, reviews or repairs.** No system Keychain migration is performed.

Requests use a fixed HTTPS destination, disable redirects and have bounded connection/response timeouts and response size. Failures retain the local clip for an explicit retry; UI/backend errors do not expose keys or raw provider bodies. A later user retry is a separate possible charge. A failed usage update after a successful transcription reports a warning and retains the recognized text.

## Usage accounting

Every actual attempt records provider `openai`, model `gpt-4o-mini-transcribe`, operation `transcription` and its P1/P2 session in the existing ledger. Only provider-reported input/output token totals are saved. The input count combines audio and text; chat-model rates must not be applied to it. Transcription cost therefore remains unknown in usage, Review and exports, with no fabricated duration-based bill. Audio/text rate configuration and detailed split counters are not part of this implementation. Cached/local recording, playback, editing and appending create no usage row.

## Platform setup

`src-tauri/Info.plist` provides the native macOS microphone-purpose description. `Entitlements.plist` declares audio input for signed builds, and the webview CSP permits local blob audio playback. Camera access is not requested. Actual permission behavior, recording format support, development versus bundled app behavior and non-macOS behavior still require validation. On macOS, test the newly built bundle rather than an old copied application.

## Additional validation scope

- Frontend/native compilation and meaningful automated coverage of multipart framing, response/error handling, usage counters, secret isolation, stale async results and transcript insertion.
- Real microphone permission allow/deny, capture/playback/stop, size/time limits and track cleanup.
- Chinese, English and mixed technical speech; correct preview editing and explicit insertion/submission in P1/P2.
- Existing drafts, 6,000-character limit, stage/session navigation and leaving the module during permission prompts or upload.
- Offline/reconnect, authentication/quota/timeout failures, manual retry without automatic paid duplication, usage persistence and unknown cost display.
- Broader real-voice/accent/device testing; never reuse the OpenAI key for interview questions or send personal history for acceptance.

## API references

- [Create transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create)
- [GPT-4o Mini Transcribe](https://developers.openai.com/api/docs/models/gpt-4o-mini-transcribe)

## Acceptance results

- 85 Rust tests passed (57 library, 11 P1, 12 P2, five session tools); one intentionally ignored Keychain test. The four voice-specific tests also passed after adding a regression that chat-style rates cannot price audio tokens and OpenAI cannot be selected as the interview LLM.
- Six frontend regressions passed, including typed-draft preservation and overflow rejection without truncation.
- TypeScript/Vite and native debug `.app` builds passed; the existing Vite chunk-size advisory remains.
- Native computer-use checks used a separate `local.swe.interview-prep.voice-qa` profile with authored interviews and DeepSeek settings. P1 and P2 showed the small icon without a large idle voice panel. Real local microphone capture, the Stop icon, explicit Transcribe, local audio controls and temporary-clip discard worked. Leaving P2 during capture stopped it; returning showed the stopped clip; switching stages cleared it. P1's authored typed draft stayed intact. The microphone clips were not uploaded.
- OpenAI made **three real transcription requests**, with locally synthesized authored English, Chinese and mixed Chinese/English speech. All returned meaningful matching text. The mixed sample retained Redis, Cache Aside and MVCC. This tests those synthetic samples, not recognition accuracy for every accent or microphone.
- DeepSeek `deepseek-flash` made **seven real text HTTP attempts**: P1 generation plus semantic review, P1 follow-up and reviewed feedback, P2 follow-up, and a P2 feedback attempt. P1 and both follow-ups passed; **P2 feedback failed with a connection/network/proxy/TLS error**. Its session remains `feedback_pending`, with the candidate answer and error preserved. The batch is therefore explicitly `failed_or_incomplete` overall, not a full DeepSeek acceptance pass. No extra paid retry was made.
- Usage verifies OpenAI rows are exclusively `gpt-4o-mini-transcribe` / `transcription`, and every text row uses DeepSeek. No Claude request was made. No normal-profile settings/history, keys or drafts were changed by these tests. Usage costs stay unknown without suitable configured rates.

The sanitized live evidence is `review/voice-live-acceptance.json`; the local ignored ledger is `review/voice-acceptance.sqlite`. `review/voice-core-acceptance.json` records scope and source hashes. The runnable bundle is `src-tauri/target/voice-input/SWE Interview Prep.app` (Git-ignored). Test and build logs are `/tmp/intv-voice-regression.log`, `/tmp/intv-voice-ui-unit.log`, `/tmp/intv-voice-unit-final.log`, `/tmp/intv-voice-frontend-final.log` and `/tmp/intv-voice-desktop-final.log`.

The opt-in `live_voice_acceptance` example requires explicit live-test intent. Its initial run requires a fresh isolated SQLite path and prepared `en.m4a`/`zh.m4a`; text requests share an eight-attempt cap. Its `--mixed` branch uses the same authored ledger and checks the four-attempt OpenAI ceiling. Running it again can incur charges; it is not part of automated tests.
