import { test } from "node:test";
import assert from "node:assert/strict";
import { validateRelease } from "../release/check.mjs";
import { containsSecret, forbiddenPath } from "../release/audit.mjs";
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
    bundle: { active: true, macOS: { signingIdentity: "-" } },
  },
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
