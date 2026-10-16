# Automatic preview releases

A merged release PR now supplies the publication decision: increase the prerelease version and include versioned release notes. After successful main CI, GitHub creates the matching tag, builds/signs the macOS arm64 app, verifies uploaded attachments, publishes a prerelease, and advances the signed update feed. Ordinary PRs without a version increase run CI only.

## Preparing a release PR

1. Use a new version greater than the previous main version, for example `0.1.0-preview.5` after preview.4. Never reuse an existing version.
2. Update `package.json`, both root version entries in `package-lock.json`, the package version in `src-tauri/Cargo.toml` and the `swe-interview-prep` entry in `Cargo.lock`, and `src-tauri/tauri.conf.json`.
3. Write `docs/releases/v<version>.md` with actual changes and limitations. Keep the README notes link current.
4. Commit, push a feature branch and open a PR. Wait for CI, review and merge into main.
5. Watch **CI**, then **Automatic preview release** in Actions. No local tag command or manual Publish is required for this path.

No new PAT, provider API keys or Apple certificate is required. The existing updater signing secret is reused. Existing macOS/ad-hoc preview limitations apply. Provider credentials stay excluded from source and artifacts.

## Workflow and trust boundaries

`auto-preview.yml` listens only for completed CI runs on main. It requires successful push CI in this repository, checks out its exact tested commit, verifies ancestry on main and confirms the matching merged PR via GitHub. PR/fork CI, failed CI, direct pushes and unchanged versions cannot release. A new version must exceed the first-parent main version. Existing tags belonging to other commits and mismatched releases fail closed.

The tag is created with the default Actions token, then the shared `build-preview.yml` is called explicitly. GitHub suppresses ordinary push/release workflow events created with `GITHUB_TOKEN`; relying on that tag event would silently stop the chain. No PAT workaround is used. Manual tag pushes and manual dispatch still use `release.yml`, test the tag, and build a draft by default.

The shared build pins checkout to the tag. Signing secrets are injected only into the build step. Packaging validates the actual updater signature and signed version. Publication additionally requires a clean commit/tag/manifest match and exact GitHub-reported SHA-256 digests for every uploaded attachment. Unknown or differing digests refuse publication.

Automatic publication calls the feed script explicitly, since its token-created release event will not start another workflow. Build/publication and manual feed publication share the same repository-wide mutex, including the feed's read/compare/write transaction. Existing rollback and immutable-version checks remain in force. CI itself is read-only; only release planning/tagging and publication jobs receive scoped repository write permissions.

## Retrying and failures

Re-run a failed **Automatic preview release** in Actions. A matching draft can be rebuilt. If the app was already published but advancing the feed failed, planning selects feed repair only; it never replaces public binaries. A conflicting tag/release requires investigation, not force-pushing. Fix a released app by publishing a greater version.

The manual **Publish preview update feed** workflow can also retry an already published tag. Drafts remain ineligible for the feed. GitHub may supersede pending automatic runs during rapid successive merges; inspect Actions and rerun the intended higher version if necessary. This is a preview distribution pipeline, not Apple notarization or a promise of schema downgrade support.

Official behavior: https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow
