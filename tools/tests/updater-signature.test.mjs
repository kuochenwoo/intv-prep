import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import {
  mkdtempSync,
  writeFileSync,
  readFileSync,
  chmodSync,
  rmSync,
} from "node:fs";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";

test(
  "real updater signatures reject changed bytes and mismatched signed versions",
  { timeout: 300_000 },
  () => {
    const dir = mkdtempSync(join(tmpdir(), "swe-updater-signature-test-"));
    const key = join(dir, "ephemeral.key"),
      archive = join(dir, "fixture.app.tar.gz"),
      config = join(dir, "config.json");
    const cli = resolve("node_modules/.bin/tauri");
    const env = { ...process.env, TAURI_SIGNING_PRIVATE_KEY_PASSWORD: "" };
    delete env.TAURI_SIGNING_PRIVATE_KEY;
    try {
      execFileSync(
        cli,
        ["signer", "generate", "--ci", "--password", "", "--write-keys", key],
        { env, stdio: "pipe" },
      );
      chmodSync(key, 0o600);
      writeFileSync(
        archive,
        "Genuine signed fixture bytes; this is not an installer.\n",
      );
      writeFileSync(
        config,
        JSON.stringify({
          plugins: {
            updater: { pubkey: readFileSync(`${key}.pub`, "utf8").trim() },
          },
        }),
      );
      execFileSync(
        cli,
        [
          "signer",
          "sign",
          "--private-key-path",
          key,
          "--password",
          "",
          "--app-version",
          "0.1.0-preview.2",
          archive,
        ],
        { env, stdio: "pipe" },
      );
      const verify = (version) =>
        spawnSync(
          "cargo",
          [
            "run",
            "--manifest-path",
            "src-tauri/Cargo.toml",
            "--no-default-features",
            "--locked",
            "--example",
            "verify_update",
            "--",
            config,
            archive,
            `${archive}.sig`,
            version,
          ],
          { encoding: "utf8", env },
        );
      const valid = verify("0.1.0-preview.2");
      assert.equal(valid.status, 0, valid.stderr);
      assert.notEqual(
        verify("0.1.0-preview.3").status,
        0,
        "Manifest version tampering must fail",
      );
      writeFileSync(archive, "Tampered fixture bytes.\n");
      assert.notEqual(
        verify("0.1.0-preview.2").status,
        0,
        "Changed artifact bytes must fail",
      );
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  },
);
