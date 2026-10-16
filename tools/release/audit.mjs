import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve, basename } from "node:path";
import { pathToFileURL } from "node:url";

export function containsSecret(content) {
  return /\bsk-(?:proj-|ant-)[A-Za-z0-9_-]{20,}|\bsk-[a-f0-9]{32}\b|\bgh[pousr]_[A-Za-z0-9]{30,}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/.test(
    content,
  );
}
export function forbiddenPath(path) {
  const file = basename(path);
  return (
    path.split(/[\\/]/).includes(".release-keys") ||
    (file.startsWith(".env") && file !== ".env.example") ||
    /\.(?:sqlite3?|db|p12|pfx|pem|key)(?:-(?:wal|shm))?$/i.test(file)
  );
}
export function auditRepository(root = process.cwd()) {
  const git = (...args) =>
    execFileSync("git", args, { cwd: root, maxBuffer: 64 * 1024 * 1024 });
  const violations = [];
  for (const path of git("ls-files", "-z")
    .toString()
    .split("\0")
    .filter(Boolean)) {
    if (forbiddenPath(path)) violations.push({ path, kind: "private-path" });
    // Deleted tracked files are deliberately absent from the next commit.
    try {
      if (containsSecret(readFileSync(resolve(root, path)).toString()))
        violations.push({ path, kind: "working-tree-secret" });
    } catch (error) {
      if (error.code !== "ENOENT") throw error;
    }
  }
  // Audit distributable refs; editor-private snapshots are not pushed or released.
  const objects = git("rev-list", "--objects", "--branches", "--tags", "--remotes")
    .toString()
    .trim()
    .split("\n")
    .filter(Boolean);
  const batch = spawnSync("git", ["cat-file", "--batch"], {
    cwd: root,
    input: objects.map((s) => s.split(" ")[0]).join("\n") + "\n",
    maxBuffer: 256 * 1024 * 1024,
  });
  if (batch.status !== 0)
    throw Error("Could not inspect Git history; audit did not pass.");
  let offset = 0;
  for (const object of objects) {
    const end = batch.stdout.indexOf(10, offset);
    const [hash, type, size] = batch.stdout
      .subarray(offset, end)
      .toString()
      .split(" ");
    offset = end + 1;
    const content = batch.stdout.subarray(offset, offset + Number(size));
    offset += Number(size) + 1;
    if (type === "blob" && containsSecret(content.toString()))
      violations.push({
        object: hash,
        path: object.slice(hash.length + 1),
        kind: "history-secret",
      });
  }
  if (violations.length) {
    // Report locations only; never echo matched credentials or content.
    console.error(JSON.stringify(violations, null, 2));
    throw Error(
      "Private material was detected; no release should be uploaded.",
    );
  }
  return objects.length;
}
if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href)
  console.log(
    `Repository audit OK: ${auditRepository()} Git objects inspected.`,
  );
