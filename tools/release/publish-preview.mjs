import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { readRelease } from "./check.mjs";

export function verifyUploadedAssets(local, assets) {
  assert.deepEqual(
    assets.map((a) => a.name).sort(),
    Object.keys(local).sort(),
    "Uploaded release attachments differ from verified local attachments.",
  );
  for (const asset of assets)
    assert.equal(
      asset.digest,
      `sha256:${local[asset.name]}`,
      `Uploaded digest mismatch: ${asset.name}`,
    );
}
function main() {
  const tag = process.env.RELEASE_TAG;
  const version = readRelease(process.cwd(), tag);
  const git = (...args) =>
    execFileSync("git", args, { encoding: "utf8" }).trim();
  const sha = git("rev-parse", "HEAD");
  if (
    git("status", "--porcelain") ||
    git("rev-parse", `${tag}^{commit}`) !== sha
  )
    throw Error("Publication requires the exact clean release tag checkout.");
  const dir = resolve("release-artifacts");
  const manifest = JSON.parse(
    readFileSync(resolve(dir, "release-manifest.json")),
  );
  if (manifest.dirty || manifest.commit !== sha || manifest.version !== version)
    throw Error("Release manifest does not match this clean commit.");
  execFileSync(
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
      "src-tauri/tauri.conf.json",
      resolve(dir, `SWE-Prep_${version}_macos-arm64.app.tar.gz`),
      resolve(dir, `SWE-Prep_${version}_macos-arm64.app.tar.gz.sig`),
      version,
    ],
    { stdio: "inherit" },
  );
  const release = JSON.parse(
    execFileSync(
      "gh",
      [
        "release",
        "view",
        tag,
        "--json",
        "isDraft,isPrerelease,targetCommitish,assets",
      ],
      { encoding: "utf8" },
    ),
  );
  if (
    !release.isDraft ||
    !release.isPrerelease ||
    release.targetCommitish !== sha
  )
    throw Error("Only the matching draft prerelease can be published.");
  const local = Object.fromEntries(
    readdirSync(dir).map((name) => [
      name,
      createHash("sha256")
        .update(readFileSync(resolve(dir, name)))
        .digest("hex"),
    ]),
  );
  verifyUploadedAssets(local, release.assets);
  execFileSync(
    "gh",
    ["release", "edit", tag, "--draft=false", "--prerelease", "--latest=false"],
    { stdio: "inherit" },
  );
  console.log(`Published verified prerelease ${tag}.`);
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  main();
