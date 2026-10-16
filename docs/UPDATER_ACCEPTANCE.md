# Updater acceptance

Candidate: `0.1.0-preview.2`, macOS 13+ arm64. Local validation on macOS 27.0.1.

- TypeScript check and production frontend build passed.
- 14 frontend/helper tests passed, including awaited draft guards, guard failures,
  unregistration and indeterminate/clamped download progress.
- Six release-policy tests passed, including immutable HTTPS asset URLs, signed
  version metadata, rollback prevention and same-version replacement prevention.
- One real cryptographic test generated an ephemeral minisign identity, accepted
  the actual signed fixture, rejected changed bytes and rejected a different
  announced version. This fixture is explicitly not an application installer.
- Native desktop compilation and one actual Release credential test passed.
- The real Release app, DMG, updater TAR/GZip and updater signature were built.
  Packaging verified the archive against the configured public key and signed
  version, checked app code signing/architecture/private-material patterns, and
  generated eleven attachments with SHA-256 checksums. All checksums passed.
- All 562 resolved dependency entries have notice texts; MPL source attachments
  remain included. The signing identity is excluded from source and artifacts.
- A separate Release app (`local.swe.interview-prep.updater-qa`) was opened with
  computer-use tooling. Native Settings showed the correct version and enabled
  manual updater control. An actual GitHub feed check reported unavailable, not
  current. Automatic-check opt-out survived process restart. English labels were
  also checked. Screenshot: ignored `release-artifacts/updater-ui/settings-check.png`.
- Repository audit passed. No paid model/transcription calls or personal profile
  changes were made. The updater-only GitHub signing secret was uploaded with
  explicit user approval; no secret value was displayed.

Limits: there is no published updater-enabled preview yet. The public channel's
publication and an installed-client upgrade/restart against that published channel
have not been certified. The workflow deliberately waits for a human to publish a
versioned preview. Preview.1 requires a manual bootstrap install. Ad-hoc signing is
not Apple notarization, and this is not a clean-machine Gatekeeper test. Local
working-tree packages are labeled dirty and are not eligible for draft upload;
CI builds produce the clean-commit release attachments.
