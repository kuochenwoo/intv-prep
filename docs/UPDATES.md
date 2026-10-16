# Signed preview updates

The updater supports installed Release `.app` bundles on macOS 13+ Apple Silicon,
running outside a mounted disk image. Development binaries cannot replace themselves.
Existing preview.1 users must install preview.2 manually once, since preview.1 has
no updater. Keep the application identifier `local.swe.interview-prep` unchanged.

## User flow

Automatic checks default to on, run five seconds after startup and every six hours,
and can be disabled in Settings. Settings also provides a manual check, current
version, last successful check, release notes and install progress. A failed or
unpublished feed is never reported as “up to date.” Background check failures stay
in Settings instead of interrupting practice with a global error.

Only the public GitHub feed/artifact endpoints are contacted. Interview content,
drafts, voice recordings and provider keys are not included. No model API is used.

Download/install/restart requires explicit confirmation. A required modal blocks
new actions during the transaction. Native guards reject existing judge, interview
or transcription tasks before download and immediately before installation; SQLite
is checkpointed. Tauri verifies the archive signature and signed version before
replacement. Restart failures offer a restart-only retry, avoiding a second install.
The application directory is replaced; data and `.env` in the runtime app-config
directory are preserved. Back up that directory before upgrades and keep `.env`
out of shared backups. Downgrade/schema compatibility is not promised.

## Signing identity

Updater signing is separate from Apple's Developer ID/ad-hoc signing. The official
Tauri updater and process plugins are included only in the desktop feature. The
macOS updater uses TAR/GZip support; its unused ZIP-format feature is disabled.
HTTPS, valid certificates, signature verification, `requireSignedVersion` and
normal greater-version comparison remain enabled. There is no insecure endpoint,
certificate bypass or forced downgrade option in production configuration.

The public key is in `src-tauri/tauri.conf.json`. The private identity is generated
locally at ignored `.release-keys/updater.key` (0600, parent 0700). Never commit,
print or package it. Keep an independent secure backup: losing it prevents issuing
updates trusted by installed clients. Rotation needs a transition signed by the
old identity, or a manual reinstall. Provider API keys are unrelated.

GitHub builds use repository secret `TAURI_SIGNING_PRIVATE_KEY`, available only to
the version-tag release build step. The generated local key has an empty password;
`TAURI_SIGNING_PRIVATE_KEY_PASSWORD` is optional for a future password-protected key.
General CI and pull requests receive no signing key. For a local signed build:

```sh
TAURI_SIGNING_PRIVATE_KEY="$PWD/.release-keys/updater.key" npm run build:macos
node tools/release/package.mjs
```

Tauri produces `.app.tar.gz` and `.sig`. Packaging cryptographically verifies them
with `verify_update`, checks the signed version, and includes `latest.json`, archive,
signature, notices and checksums in the draft prerelease. Tests use a disposable
key and real signatures; no successful application install is simulated.

## Publishing a preview

The feed is a public `latest.json` asset on the `updates-preview` prerelease:

`https://github.com/kuochenwoo/intv-prep/releases/download/updates-preview/latest.json`

The manifest points at an immutable versioned `.app.tar.gz` URL, not at a draft URL.
GitHub's `releases/latest` is deliberately unused because it excludes prereleases.

1. Merge the workflows onto the default branch, then build a new version tag.
2. Review the successful draft build, artifacts and installation acceptance.
3. Publish that versioned prerelease manually. This is the publication decision.
4. `publish-update.yml` validates the public release, checksums, clean commit/tag,
   signature and signed version, then creates/updates the `updates-preview` feed.

The feed workflow never publishes an application draft. Its `contents: write`
permission is confined to its publish job. Channel writes are serialized; older
versions and different payloads for the same version are refused. A manual dispatch
can retry an already published preview, but refuses drafts. First publication has
no feed yet, so checks before that point correctly report an unavailable feed.

Do not manually overwrite released versions or the channel manifest. For a bad
release, publish a corrected higher version. Signature verification does not replace
release review, Apple notarization or a clean-machine Gatekeeper test.

Official reference: https://v2.tauri.app/plugin/updater/
