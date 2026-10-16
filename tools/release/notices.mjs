import { readFileSync, readdirSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { execFileSync } from "node:child_process";

function licenseTexts(directory, explicit) {
  const names = readdirSync(directory).filter((name) =>
    /^(?:licen[sc]e|copying|notice|ofl|copyright)(?:$|[.-])/i.test(name),
  );
  const paths = new Set(names.map((name) => resolve(directory, name)));
  if (explicit) paths.add(resolve(directory, explicit));
  const texts = [];
  for (const path of paths) {
    try {
      texts.push(readFileSync(path, "utf8"));
    } catch (error) {
      if (!["EISDIR", "ENOENT"].includes(error.code)) throw error;
    }
  }
  return texts;
}
export function dependencyNotices(root = process.cwd()) {
  const lock = JSON.parse(
    readFileSync(resolve(root, "package-lock.json"), "utf8"),
  );
  const entries = [];
  const noticeDir = resolve(root, "licenses/third-party");
  const sources = JSON.parse(readFileSync(resolve(noticeDir, "sources.json"), "utf8"));
  const supplements = JSON.parse(readFileSync(resolve(noticeDir, "packages.json"), "utf8"));
  const sourcePackages = [];
  for (const [path, record] of Object.entries(lock.packages)) {
    if (!path || record.dev) continue;
    const dir = resolve(root, path);
    if (!existsSync(resolve(dir, "package.json")))
      throw Error(`Production dependency missing: ${path}`);
    const pkg = JSON.parse(readFileSync(resolve(dir, "package.json"), "utf8"));
    entries.push({
      ecosystem: "npm",
      name: pkg.name,
      version: pkg.version,
      license: pkg.license ?? record.license ?? "UNSPECIFIED",
      repository:
        typeof pkg.repository === "string"
          ? pkg.repository
          : pkg.repository?.url,
      texts: licenseTexts(dir),
    });
  }
  const metadata = JSON.parse(
    execFileSync(
      "cargo",
      [
        "metadata",
        "--manifest-path",
        "src-tauri/Cargo.toml",
        "--locked",
        "--format-version",
        "1",
        "--filter-platform",
        "aarch64-apple-darwin",
      ],
      { cwd: root, maxBuffer: 64 * 1024 * 1024 },
    ),
  );
  const ids = new Set(metadata.resolve.nodes.map((n) => n.id));
  for (const pkg of metadata.packages) {
    if (pkg.id === metadata.resolve.root || !ids.has(pkg.id)) continue;
    if (pkg.license?.includes("MPL-2.0")) {
      sourcePackages.push({ name: pkg.name, version: pkg.version, directory: dirname(pkg.manifest_path) });
    }
    entries.push({
      ecosystem: "Cargo",
      name: pkg.name,
      version: pkg.version,
      license: pkg.license ?? "SEE LICENSE FILE",
      repository: pkg.repository,
      texts: licenseTexts(dirname(pkg.manifest_path), pkg.license_file),
    });
  }
  for (const entry of entries) {
    const key = `${entry.ecosystem}:${entry.name}@${entry.version}`;
    if (!entry.texts.length && supplements[key]) {
      entry.texts = supplements[key].map((name) => {
        const record = sources[name];
        return `Supplemental upstream notice / standard license text
Source: ${record.source}

${readFileSync(resolve(noticeDir, record.file), "utf8")}`;
      });
    }
  }
  entries.sort((a, b) =>
    `${a.ecosystem}:${a.name}:${a.version}`.localeCompare(
      `${b.ecosystem}:${b.name}:${b.version}`,
    ),
  );
  const missing = entries
    .filter((e) => !e.texts.length)
    .map((e) => `${e.ecosystem}:${e.name}@${e.version}`);
  const header =
    "Third-party dependency notices\n\nThe proprietary project license does not replace upstream licenses. This inventory includes production npm dependencies and resolved Rust target/build dependencies; some build dependencies are not distributed in the executable. Original package notices and pinned supplemental upstream/standard SPDX texts follow. SPDX placeholder copyright text is not an attribution of copyright; upstream notices are retained separately. Unmodified MPL-covered crate sources are included in MPL_SOURCES.zip, with exact registry source links in its index. This inventory is not a compatibility opinion.\n";
  return {
    text:
      header +
      entries
        .map(
          (e) =>
            `\n${"=".repeat(72)}\n${e.ecosystem}: ${e.name}@${e.version}\nDeclared license: ${typeof e.license === "string" ? e.license : JSON.stringify(e.license)}\nRepository: ${e.repository ?? "See package registry"}\n\n${e.texts.join("\n---\n") || "No standalone license text in this upstream package; consult its declared license and repository."}\n`,
        )
        .join(""),
    count: entries.length,
    sourcePackages,
    missing,
  };
}
