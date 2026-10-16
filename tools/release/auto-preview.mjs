import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, appendFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { gt } from "semver";
import { readRelease } from "./check.mjs";

const repository = "kuochenwoo/intv-prep";
export function planPreview({
  run,
  version,
  previousVersion,
  merged,
  tagCommit,
  release,
}) {
  if (
    run.conclusion !== "success" ||
    run.event !== "push" ||
    run.head_branch !== "main" ||
    run.head_repository?.full_name !== repository
  )
    return { action: "skip" };
  if (!merged || version === previousVersion) return { action: "skip" };
  if (
    !/^\d+\.\d+\.\d+-(?:preview|alpha|beta|rc)\.\d+$/.test(version) ||
    !gt(version, previousVersion)
  )
    throw Error(
      "Automatic previews require a strictly greater prerelease version.",
    );
  if (tagCommit && tagCommit !== run.head_sha)
    throw Error("The version tag already belongs to another commit.");
  if (
    release &&
    (release.targetCommitish !== run.head_sha || !release.isPrerelease)
  )
    throw Error("The existing release does not match this preview commit.");
  return {
    action: release && !release.isDraft ? "feed" : "build",
    tag: `v${version}`,
  };
}

function main() {
  const event = JSON.parse(readFileSync(process.env.GITHUB_EVENT_PATH, "utf8"));
  const run = event.workflow_run;
  if (
    process.env.GITHUB_REPOSITORY !== repository ||
    !run ||
    run.conclusion !== "success" ||
    run.event !== "push" ||
    run.head_branch !== "main" ||
    run.head_repository?.full_name !== repository
  )
    throw Error(
      "Only successful main push CI in the release repository is trusted.",
    );
  const git = (...args) =>
    execFileSync("git", args, { encoding: "utf8" }).trim();
  const gh = (...args) => execFileSync("gh", args, { encoding: "utf8" }).trim();
  if (git("rev-parse", "HEAD") !== run.head_sha)
    throw Error("Checkout differs from tested commit.");
  execFileSync("git", [
    "merge-base",
    "--is-ancestor",
    run.head_sha,
    "origin/main",
  ]);
  const version = readRelease();
  const previousVersion = JSON.parse(git("show", "HEAD^:package.json")).version;
  const pulls = JSON.parse(
    gh("api", `repos/${repository}/commits/${run.head_sha}/pulls`),
  );
  const merged = pulls.some(
    (p) =>
      p.merged_at &&
      p.base?.ref === "main" &&
      p.merge_commit_sha === run.head_sha,
  );
  if (
    planPreview({ run, version, previousVersion, merged }).action === "skip"
  ) {
    appendFileSync(process.env.GITHUB_OUTPUT, "action=skip\ntag=\n");
    console.log("No merged version increase; no release requested.");
    return;
  }
  const tag = `v${version}`;
  const ref = spawnSync("git", ["rev-parse", "--verify", `${tag}^{commit}`], {
    encoding: "utf8",
  });
  let release = null;
  const existing = spawnSync(
    "gh",
    ["release", "view", tag, "--json", "isDraft,isPrerelease,targetCommitish"],
    { encoding: "utf8" },
  );
  if (existing.status === 0) release = JSON.parse(existing.stdout);
  else if (!/release not found|HTTP 404/i.test(existing.stderr))
    throw Error("Cannot inspect existing release.");
  const plan = planPreview({
    run,
    version,
    previousVersion,
    merged,
    tagCommit: ref.status === 0 ? ref.stdout.trim() : null,
    release,
  });
  if (plan.action === "build" && ref.status !== 0) {
    // GITHUB_TOKEN tag pushes do not trigger push workflows. The caller explicitly reuses the build workflow.
    gh(
      "api",
      `repos/${repository}/git/refs`,
      "-f",
      `ref=refs/tags/${tag}`,
      "-f",
      `sha=${run.head_sha}`,
    );
  }
  appendFileSync(
    process.env.GITHUB_OUTPUT,
    `action=${plan.action}\ntag=${plan.tag ?? ""}\n`,
  );
  console.log(
    `Automatic preview: ${plan.action}${plan.tag ? ` ${plan.tag}` : " (no merged version increase)"}.`,
  );
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  main();
