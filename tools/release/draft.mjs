import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { readRelease } from "./check.mjs";
const product = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8")).productName;
const tag = process.env.RELEASE_TAG;
if (!tag) throw Error("RELEASE_TAG is required.");
const version = readRelease(process.cwd(), tag);
const sha = execFileSync("git", ["rev-parse", "HEAD"]).toString().trim();
const manifest = JSON.parse(
  readFileSync("release-artifacts/release-manifest.json"),
);
if (manifest.dirty || manifest.commit !== sha || manifest.version !== version)
  throw Error("Draft assets must come from this exact clean commit.");
const ref = spawnSync("git", ["rev-parse", `${tag}^{commit}`], {
  encoding: "utf8",
});
if (ref.status === 0 && ref.stdout.trim() !== sha)
  throw Error("The tag points to another commit.");
const existing = spawnSync(
  "gh",
  ["release", "view", tag, "--json", "isDraft,targetCommitish"],
  { encoding: "utf8" },
);
if (existing.status === 0) {
  const value = JSON.parse(existing.stdout);
  if (!value.isDraft || value.targetCommitish !== sha)
    throw Error(
      "Refusing to overwrite a public release or a draft from another commit.",
    );
} else {
  if (!/release not found|HTTP 404/i.test(existing.stderr))
    throw Error("Cannot inspect release status; refusing to continue.");
  execFileSync(
    "gh",
    [
      "release",
      "create",
      tag,
      "--draft",
      "--prerelease",
      "--target",
      sha,
      "--title",
      `${product} ${version} (macOS preview)`,
      "--notes-file",
      `docs/releases/v${version}.md`,
    ],
    { stdio: "inherit" },
  );
}
const files = readdirSync("release-artifacts")
  .sort()
  .map((file) => resolve("release-artifacts", file));
execFileSync("gh", ["release", "upload", tag, "--clobber", ...files], {
  stdio: "inherit",
});
console.log(
  "Draft assets uploaded. This workflow never publishes the release.",
);
