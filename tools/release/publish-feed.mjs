import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, mkdtempSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { createHash } from "node:crypto";
import {
  UPDATE_CHANNEL_TAG,
  validateUpdateFeed,
  shouldPublishFeed,
  updaterFile,
} from "./update-feed.mjs";
const tag = process.env.RELEASE_TAG;
if (!/^v\d+\.\d+\.\d+-(?:preview|alpha|beta|rc)\.\d+$/.test(tag ?? ""))
  throw Error("A versioned preview tag is required.");
const gh = (...args) => execFileSync("gh", args, { encoding: "utf8" });
const release = JSON.parse(
  gh("release", "view", tag, "--json", "isDraft,isPrerelease,tagName,assets"),
);
if (release.isDraft || release.tagName !== tag || !release.isPrerelease)
  throw Error("Only a published preview can update this feed.");
const dir = mkdtempSync(join(tmpdir(), "swe-published-update-"));
const names = [
  "latest.json",
  "release-manifest.json",
  "SHA256SUMS.txt",
  updaterFile(tag.slice(1)),
  `${updaterFile(tag.slice(1))}.sig`,
];
for (const name of names)
  gh("release", "download", tag, "--pattern", name, "--dir", dir);
const sums = new Map(
  readFileSync(join(dir, "SHA256SUMS.txt"), "utf8")
    .trim()
    .split("\n")
    .map((line) => [line.slice(66), line.slice(0, 64)]),
);
for (const name of names.filter((name) => name !== "SHA256SUMS.txt")) {
  const actual = createHash("sha256")
    .update(readFileSync(join(dir, name)))
    .digest("hex");
  if (actual !== sums.get(name)) throw Error(`Invalid checksum for ${name}.`);
}
const feed = validateUpdateFeed(
  JSON.parse(readFileSync(join(dir, "latest.json"), "utf8")),
);
const manifest = JSON.parse(
  readFileSync(join(dir, "release-manifest.json"), "utf8"),
);
const sha = execFileSync("git", ["rev-parse", `${tag}^{commit}`], {
  encoding: "utf8",
}).trim();
if (
  feed.version !== tag.slice(1) ||
  manifest.version !== feed.version ||
  manifest.dirty ||
  manifest.commit !== sha
)
  throw Error("Published metadata does not match the clean release tag.");
if (
  feed.platforms["darwin-aarch64"].signature !==
  readFileSync(join(dir, `${updaterFile(feed.version)}.sig`), "utf8").trim()
)
  throw Error("Feed and artifact signatures differ.");
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
    join(dir, updaterFile(feed.version)),
    join(dir, `${updaterFile(feed.version)}.sig`),
    feed.version,
  ],
  { stdio: "inherit" },
);
const channel = spawnSync(
  "gh",
  ["release", "view", UPDATE_CHANNEL_TAG, "--json", "isDraft,assets"],
  { encoding: "utf8" },
);
let current = null;
if (channel.status === 0) {
  const existing = JSON.parse(channel.stdout);
  if (existing.isDraft) throw Error("The update channel must be public.");
  if (existing.assets.some((asset) => asset.name === "latest.json")) {
    const previous = join(dir, "previous");
    gh(
      "release",
      "download",
      UPDATE_CHANNEL_TAG,
      "--pattern",
      "latest.json",
      "--dir",
      previous,
    );
    current = JSON.parse(readFileSync(join(previous, "latest.json"), "utf8"));
  } else if (existing.assets.length)
    throw Error("Unexpected update channel assets.");
} else if (!/release not found|HTTP 404/i.test(channel.stderr))
  throw Error("Could not inspect the update channel; refusing to publish.");
if (!shouldPublishFeed(feed, current)) {
  console.log("The update feed already points to this exact version.");
} else {
  if (channel.status !== 0)
    gh(
      "release",
      "create",
      UPDATE_CHANNEL_TAG,
      "--prerelease",
      "--target",
      sha,
      "--title",
      "SWE Prep preview update feed",
      "--notes",
      "Automatic update manifest for published macOS arm64 previews. Installers and release notes live in the versioned releases. Do not edit this feed manually.",
    );
  if (!existsSync(join(dir, "latest.json")))
    throw Error("Missing verified feed.");
  gh(
    "release",
    "upload",
    UPDATE_CHANNEL_TAG,
    join(dir, "latest.json"),
    "--clobber",
  );
  console.log(
    `Published the verified preview update feed for ${feed.version}.`,
  );
}
