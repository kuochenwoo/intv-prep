# macOS preview release

The first release was `0.1.0-preview.1`; the updater-enabled candidate is `0.1.0-preview.2`, targeting macOS 13+ on Apple Silicon. It is
ad-hoc signed and not Apple-notarized. Original project material remains proprietary;
see LICENSE. Intel, other operating systems, automatic updates and Developer ID
signing/notarization are outside this preview.

## Storage and compatibility

The application identifier remains `local.swe.interview-prep` to preserve the current
SQLite data location. Release builds initialize credentials from Tauri's runtime
`app_config_dir()` and create a provider-specific `.env` there with mode 0600. On
macOS that directory is `~/Library/Application Support/local.swe.interview-prep/`.
The exact credential path is visible in Settings. Release credentials cannot fall
back to the build machine's checkout. No existing project key is copied automatically.
Debug development builds and manually invoked debug probes retain the ignored
project `.env`. OpenAI remains exclusively for voice transcription.

Quit before upgrading and back up the application data directory. Replace the app
bundle without deleting the data directory. Keep credential files out of shared
backups. Downgrades require a compatible data backup; schema downgrade compatibility
is not promised. A signed/notarized update can retain the same identifier later.

## CI

`.github/workflows/ci.yml` runs on pull requests, pushes to main, manual dispatch
and calls from the release workflow. It performs:

1. `npm ci`, version/tag/identity checks and Git-history/private-file auditing.
2. Release-tool tests, frontend tests and production frontend build.
3. Full Rust tests, including every LC reference in both modes and local HTTP fixtures.
4. A real `--release` credential-storage integration test with synthetic keys only.

Actions are pinned to immutable commits. Node 24, stable Rust and a macOS 15 ARM
runner are used. CI has read-only repository permissions and requires no model API
keys, signing certificates or personal data. Do not run paid probes in CI.

## Draft release pipeline

`.github/workflows/release.yml` runs when a `v*` tag is pushed, or when manually
called with a matching tag (manual dispatch becomes available after the workflow
exists on the default branch). It first calls CI, then:

1. Verifies the requested tag equals all application/lockfile versions.
2. Embeds license notices and MPL dependency sources in application assets, then builds a Release arm64 `.app` and `.dmg`, with ad-hoc signing enabled in Tauri.
3. Checks the real signature and architecture and scans the app for private files,
   credential patterns and a compiled-in checkout `.env` path.
4. Prepares the app ZIP, DMG, proprietary license, versioned release notes, dependency
   license texts, MPL dependency sources, build manifest and SHA-256 checksums.
5. Uploads workflow artifacts and creates/updates a **draft prerelease** only.

Only the final release job receives `contents: write`; its `GH_TOKEN` is GitHub's
workflow token. No Claude, DeepSeek or OpenAI key belongs in repository secrets for
this pipeline. The workflow refuses public-release overwrites and drafts from a
different commit. It does not publish an application release. Signed updater artifacts are included; after a human publishes the versioned prerelease, a separate workflow advances the preview feed. See `UPDATES.md`.

After review, create a version tag at the tested commit and push it:

```sh
git tag v0.1.0-preview.2 <tested-commit>
git push origin v0.1.0-preview.2
```

Preview assets are generated in ignored `release-artifacts/`. To build locally:

```sh
npm ci
npm run check:release
npm run audit:release
npm run test:release-tools
npm run test:review-ui
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --locked
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --release --locked --test release_credentials
TAURI_SIGNING_PRIVATE_KEY="$PWD/.release-keys/updater.key" npm run build:macos
node tools/release/package.mjs
```

Packaging requires a fresh artifact directory and preserves any existing output.
Use `CARGO_TARGET_DIR` consistently for build and packaging if using an external
build cache. A local dirty build is labeled in its manifest and cannot be uploaded
by `draft.mjs`. Draft uploads must refer to the exact clean Git commit and tag.

## Installation acceptance and publication

Use a copy outside the source checkout. Check cold start, missing-key state, compiler
detection, a real LC submission in both modes, settings persistence, history, exports
and restart. Use an isolated release-profile QA identifier for destructive/dummy-key
checks; do not alter the personal profile or call providers during this acceptance.
Test Release configuration, not `--debug`. An ad-hoc build can still require macOS's
explicit Privacy & Security approval; do not disable Gatekeeper or clear quarantine.

Inspect the draft artifacts and checksums, release notes, known AI limitations and
license notices. Dependency notices include some build dependencies and preserve
upstream texts and pinned supplemental license texts. Unmodified MPL-covered crate sources are attached separately. They do
not assert that all dependencies are embedded or replace a license-compatibility
review for broader/commercial distribution.

Publishing remains a human decision in GitHub Releases. No workflow publishes
silently, and no developer certificate or notarization claim is implied by ad-hoc
signature verification. A clean-machine Gatekeeper test and additional OS versions
remain separate from testing a relocated bundle on the development Mac.

Official references: [Tauri GitHub pipelines](https://v2.tauri.app/distribute/pipelines/github/),
[Tauri macOS signing](https://v2.tauri.app/distribute/sign/macos/),
[GitHub Actions secrets](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-secrets).
