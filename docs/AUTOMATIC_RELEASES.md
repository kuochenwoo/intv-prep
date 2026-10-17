# Automatic releases

A merged release PR now supplies the publication decision: increase the application version and include versioned release notes. After successful main CI, GitHub creates the matching tag, builds/signs the macOS arm64 app, verifies uploaded attachments, publishes a release, and advances the signed update feed. Ordinary PRs without a version increase run CI only.

## Preparing a release PR

1. Use a new version greater than the previous main version, for example `1.0.0` after preview.6. Never reuse an existing version.
2. Update `package.json`, both root version entries in `package-lock.json`, the package version in `src-tauri/Cargo.toml` and the `swe-interview-prep` entry in `Cargo.lock`, and `src-tauri/tauri.conf.json`.
3. Write `docs/releases/v<version>.md` with actual changes and limitations. Keep the README notes link current.
4. Commit, push a feature branch and open a PR. Wait for CI, review and merge into main.
5. Watch **CI**, then **Automatic release** in Actions. No local tag command or manual Publish is required for this path.

No new PAT, provider API keys or Apple certificate is required. The existing updater signing secret is reused. Existing macOS/ad-hoc preview limitations apply. Provider credentials stay excluded from source and artifacts.

## Workflow and trust boundaries

`auto-preview.yml` listens only for completed CI runs on main. It requires successful push CI in this repository, checks out its exact tested commit, verifies ancestry on main and confirms the matching merged PR via GitHub. PR/fork CI, failed CI, direct pushes and unchanged versions cannot release. A stable or numbered prerelease version must exceed the first-parent main version. Existing tags belonging to other commits and mismatched releases fail closed.

The tag is created with the default Actions token, then the shared `build-preview.yml` is called explicitly. GitHub suppresses ordinary push/release workflow events created with `GITHUB_TOKEN`; relying on that tag event would silently stop the chain. No PAT workaround is used. Manual tag pushes and manual dispatch still use `release.yml`, test the tag, and build a draft by default.

The shared build pins checkout to the tag. Signing secrets are injected only into the build step. Packaging validates the actual updater signature and signed version. Publication additionally requires a clean commit/tag/manifest match and exact GitHub-reported SHA-256 digests for every uploaded attachment. Unknown or differing digests refuse publication.

Automatic publication calls the feed script explicitly, since its token-created release event will not start another workflow. Build/publication and manual feed publication share the same repository-wide mutex, including the feed's read/compare/write transaction. Existing rollback and immutable-version checks remain in force. CI itself is read-only; only release planning/tagging and publication jobs receive scoped repository write permissions.

## Retrying and failures

Re-run a failed **Automatic release** in Actions. A matching draft can be rebuilt. If the app was already published but advancing the feed failed, planning selects feed repair only; it never replaces public binaries. A conflicting tag/release requires investigation, not force-pushing. Fix a released app by publishing a greater version.

The manual **Publish update feed** workflow can also retry an already published tag. Drafts remain ineligible for the feed. GitHub may supersede pending automatic runs during rapid successive merges; inspect Actions and rerun the intended higher version if necessary. Stable releases are marked Latest; numbered previews remain prereleases. Distribution remains ad-hoc signed, without Apple notarization or a promise of schema downgrade support.

Official behavior: https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow


## Preview-to-stable compatibility

The legacy `updates-preview/latest.json` endpoint and signing identity stay unchanged. Installed previews can receive stable 1.0.0. Once the feed reaches a stable version, a later prerelease cannot advance it; the preview release itself may still be published. New stable versions can advance it normally. Checksums, signed-version verification, immutable uploaded digests, clean-tag binding and rollback guards apply to both kinds. Legacy script and workflow filenames containing `preview` are retained for compatibility.
