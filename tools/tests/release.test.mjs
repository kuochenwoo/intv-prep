import { test } from "node:test";
import assert from "node:assert/strict";
import { validateRelease } from "../release/check.mjs";
import { containsSecret, forbiddenPath } from "../release/audit.mjs";
import { planPreview } from "../release/auto-preview.mjs";
import { verifyUploadedAssets } from "../release/publish-preview.mjs";
const fixture = () => ({
  pkg: { version: "0.1.0-preview.1", license: "UNLICENSED", private: true },
  lock: {
    version: "0.1.0-preview.1",
    packages: { "": { version: "0.1.0-preview.1" } },
  },
  cargo: '[package]\nversion = "0.1.0-preview.1"',
  cargoLock:
    '[[package]]\nname = "swe-interview-prep"\nversion = "0.1.0-preview.1"',
  tauri: {
    version: "0.1.0-preview.1",
    identifier: "local.swe.interview-prep",
    bundle: {
      active: true,
      createUpdaterArtifacts: true,
      macOS: { signingIdentity: "-" },
    },
    plugins: {
      updater: {
        pubkey: "public-key-fixture",
        requireSignedVersion: true,
        endpoints: [
          "https://github.com/kuochenwoo/intv-prep/releases/download/updates-preview/latest.json",
        ],
      },
    },
  },
});
const automatic = () => ({
  run: {
    conclusion: "success",
    event: "push",
    head_branch: "main",
    head_sha: "tested-commit",
    head_repository: { full_name: "kuochenwoo/intv-prep" },
  },
  version: "0.1.0-preview.4",
  previousVersion: "0.1.0-preview.3",
  merged: true,
});
test("automatic previews require successful trusted main CI and a merged version increase", () => {
  assert.deepEqual(planPreview(automatic()), {
    action: "build",
    tag: "v0.1.0-preview.4",
  });
  for (const run of [
    { conclusion: "failure" },
    { event: "pull_request" },
    { head_branch: "feature" },
    { head_repository: { full_name: "untrusted/fork" } },
  ])
    assert.equal(
      planPreview({ ...automatic(), run: { ...automatic().run, ...run } })
        .action,
      "skip",
    );
  assert.equal(planPreview({ ...automatic(), merged: false }).action, "skip");
  assert.equal(
    planPreview({ ...automatic(), previousVersion: "0.1.0-preview.4" }).action,
    "skip",
  );
});
test("automatic previews reject downgraded versions, stable versions and conflicting tags", () => {
  assert.throws(
    () => planPreview({ ...automatic(), version: "0.1.0-preview.2" }),
    /strictly greater/,
  );
  assert.throws(
    () => planPreview({ ...automatic(), version: "0.1.0" }),
    /prerelease/,
  );
  assert.throws(
    () => planPreview({ ...automatic(), tagCommit: "different-commit" }),
    /another commit/,
  );
});
test("automatic reruns rebuild matching drafts but repair feeds for matching published previews", () => {
  const release = {
    targetCommitish: "tested-commit",
    isPrerelease: true,
    isDraft: true,
  };
  assert.equal(
    planPreview({ ...automatic(), tagCommit: "tested-commit", release }).action,
    "build",
  );
  assert.equal(
    planPreview({ ...automatic(), release: { ...release, isDraft: false } })
      .action,
    "feed",
  );
  assert.throws(
    () =>
      planPreview({
        ...automatic(),
        release: { ...release, targetCommitish: "another" },
      }),
    /does not match/,
  );
  assert.throws(
    () =>
      planPreview({
        ...automatic(),
        release: { ...release, isPrerelease: false },
      }),
    /does not match/,
  );
});
test("publication requires exact uploaded attachments and provider-reported SHA256 digests", () => {
  const files = { "app.tar.gz": "abc", "app.tar.gz.sig": "def" };
  const assets = [
    { name: "app.tar.gz", digest: "sha256:abc" },
    { name: "app.tar.gz.sig", digest: "sha256:def" },
  ];
  verifyUploadedAssets(files, assets);
  assert.throws(
    () => verifyUploadedAssets(files, assets.slice(0, 1)),
    /attachments differ/,
  );
  assert.throws(
    () => verifyUploadedAssets(files, [...assets, { name: "unexpected" }]),
    /attachments differ/,
  );
  assert.throws(
    () =>
      verifyUploadedAssets(files, [{ ...assets[0], digest: null }, assets[1]]),
    /digest mismatch/,
  );
  assert.throws(
    () =>
      verifyUploadedAssets(files, [
        { ...assets[0], digest: "sha256:changed" },
        assets[1],
      ]),
    /digest mismatch/,
  );
});
test("release checks reject version drift, wrong tags and accidental stable releases", () => {
  assert.equal(
    validateRelease(fixture(), "v0.1.0-preview.1"),
    "0.1.0-preview.1",
  );
  const source = fixture();
  source.lock.packages[""].version = "0.1.0";
  assert.throws(() => validateRelease(source), /versions must match/);
  assert.throws(() => validateRelease(fixture(), "v0.2.0-preview.1"), /tag/);
  const stable = fixture();
  stable.pkg.version = "0.1.0";
  assert.throws(() => validateRelease(stable), /prerelease/);
});
test("preview checks protect identity, packaging and authorization scope", () => {
  for (const change of [
    (x) => (x.tauri.identifier = "other-app"),
    (x) => (x.tauri.bundle.active = false),
    (x) => (x.tauri.bundle.macOS.signingIdentity = null),
    (x) => (x.pkg.license = "MIT"),
    (x) => (x.tauri.plugins.updater.requireSignedVersion = false),
    (x) => (x.tauri.plugins.updater.dangerousAcceptInvalidCerts = true),
    (x) => (x.tauri.bundle.createUpdaterArtifacts = false),
  ]) {
    const source = fixture();
    change(source);
    assert.throws(() => validateRelease(source));
  }
});
test("private-path checks preserve the empty template but reject credential and data files", () => {
  for (const path of [
    ".env",
    "config/.env.production",
    "prep.sqlite3",
    "prep.sqlite3-wal",
    "certificate.p12",
    "signing.key",
    ".release-keys/generate.log",
  ])
    assert.equal(forbiddenPath(path), true, path);
  for (const path of [
    ".env.example",
    "review/results.json",
    "content/problem.json",
    "src/db.rs",
  ])
    assert.equal(forbiddenPath(path), false, path);
});
test("secret checks detect credentials without treating test fixture descriptions as keys", () => {
  for (const key of [
    "sk-proj-" + "a".repeat(40),
    "sk-ant-" + "x".repeat(40),
    "sk-" + "a".repeat(32),
    "ghp_" + "b".repeat(36),
    "-----BEGIN " + "PRIVATE KEY-----",
  ])
    assert.equal(containsSecret(key), true);
  assert.equal(containsSecret("fixture-release-claude-key"), false);
  assert.equal(
    containsSecret("DEEPSEEK_API_KEY=\nCLAUDE_API_KEY=\nOPENAI_API_KEY=\n"),
    false,
  );
});

const { createUpdateFeed, shouldPublishFeed, validateUpdateFeed } =
  await import("../release/update-feed.mjs");
const signatureFor = (version) =>
  Buffer.from(
    `untrusted comment: fixture\nfixture\ntrusted comment: timestamp:1\tversion:${version}\nfixture\n`,
  ).toString("base64");
const feedFor = (version) =>
  createUpdateFeed(
    version,
    signatureFor(version),
    "Fixture release notes",
    "2026-10-16T00:00:00Z",
  );
test("update feeds bind versions to immutable HTTPS download URLs", () => {
  const feed = feedFor("0.1.0-preview.2");
  assert.equal(validateUpdateFeed(feed).version, "0.1.0-preview.2");
  for (const mutate of [
    (x) => (x.version = "0.1.0-preview.3"),
    (x) =>
      (x.platforms["darwin-aarch64"].url = "http://example.com/app.tar.gz"),
    (x) => (x.platforms["darwin-aarch64"].signature = ""),
  ]) {
    const changed = structuredClone(feed);
    mutate(changed);
    assert.throws(() => validateUpdateFeed(changed));
  }
});
test("update publishing refuses rollback or replacement of the same version", () => {
  const old = feedFor("0.1.0-preview.2"),
    next = feedFor("0.1.0-preview.3");
  assert.equal(shouldPublishFeed(old, null), true);
  assert.equal(shouldPublishFeed(next, old), true);
  assert.equal(shouldPublishFeed(old, structuredClone(old)), false);
  assert.throws(() => shouldPublishFeed(old, next), /backwards/);
  const replacement = structuredClone(old);
  replacement.platforms["darwin-aarch64"].signature = Buffer.from(
    `trusted comment: timestamp:2\tversion:${old.version}\n`,
  ).toString("base64");
  assert.throws(() => shouldPublishFeed(replacement, old), /immutable/);
});
