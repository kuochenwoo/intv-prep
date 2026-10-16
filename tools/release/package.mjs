import {
  readFileSync,
  readdirSync,
  mkdirSync,
  copyFileSync,
  writeFileSync,
  mkdtempSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { resolve, join } from "node:path";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readRelease } from "./check.mjs";
import { containsSecret, forbiddenPath } from "./audit.mjs";
import { dependencyNotices } from "./notices.mjs";

const root = process.cwd(),
  version = readRelease(root);
const target = resolve(process.env.CARGO_TARGET_DIR ?? "src-tauri/target");
const bundle = join(target, "aarch64-apple-darwin/release/bundle");
const product = JSON.parse(readFileSync(resolve("src-tauri/tauri.conf.json"), "utf8")).productName;
const app = join(bundle, "macos", `${product}.app`);
execFileSync("codesign", ["--verify", "--deep", "--strict", app], {
  stdio: "inherit",
});
const arch = execFileSync("lipo", [
  "-archs",
  join(app, "Contents/MacOS/swe-interview-prep"),
])
  .toString()
  .trim();
if (arch !== "arm64")
  throw Error("The preview must contain an arm64 executable.");
function auditApp(dir) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (forbiddenPath(path))
      throw Error("Private data file found in the app bundle.");
    if (entry.isDirectory()) auditApp(path);
    else if (entry.isFile()) {
      const data = readFileSync(path);
      if (containsSecret(data.toString()))
        throw Error("Credential pattern found in the app bundle.");
      if (data.includes(Buffer.from(join(root, ".env"))))
        throw Error("The app embeds a checkout credential path.");
    }
  }
}
auditApp(app);
const out = resolve("release-artifacts");
mkdirSync(out, { recursive: true });
if (readdirSync(out).length)
  throw Error(
    "Use a fresh release-artifacts directory; existing output is preserved.",
  );
const prefix = `${product.replace(/[^A-Za-z0-9]+/g, "-")}_${version}_macos-arm64`;
execFileSync("ditto", [
  "-c",
  "-k",
  "--sequesterRsrc",
  "--keepParent",
  app,
  join(out, `${prefix}.app.zip`),
]);
const dmgs = readdirSync(join(bundle, "dmg")).filter(
  (f) => f.endsWith(".dmg") && f.includes(version),
);
if (dmgs.length !== 1) throw Error("Expected one DMG for the current version.");
copyFileSync(join(bundle, "dmg", dmgs[0]), join(out, `${prefix}.dmg`));
copyFileSync(resolve("LICENSE"), join(out, "LICENSE.txt"));
copyFileSync(
  resolve(`docs/releases/v${version}.md`),
  join(out, "RELEASE_NOTES.md"),
);
const notices = dependencyNotices(root);
if (notices.missing.length) throw Error(`Missing license notices: ${notices.missing.join(", ")}`);
writeFileSync(join(out, "THIRD_PARTY_NOTICES.txt"), notices.text);
const sourceDir = mkdtempSync(join(tmpdir(), "swe-preview-mpl-"));
const sourceIndex = [];
for (const pkg of notices.sourcePackages) {
  const filename = `${pkg.name}-${pkg.version}`;
  execFileSync("ditto", [pkg.directory, join(sourceDir, filename)]);
  sourceIndex.push(`${filename}: unmodified Cargo registry source; https://crates.io/api/v1/crates/${pkg.name}/${pkg.version}/download`);
}
writeFileSync(join(sourceDir, "SOURCES.txt"), "Unmodified MPL-covered dependency sources. These retain upstream licenses and are not governed by the proprietary application license.\n\n" + sourceIndex.join("\n") + "\n");
execFileSync("ditto", ["-c", "-k", sourceDir, join(out, "MPL_SOURCES.zip")]);
const commit = execFileSync("git", ["rev-parse", "HEAD"]).toString().trim();
const dirty = !!execFileSync("git", ["status", "--porcelain"])
  .toString()
  .trim();
writeFileSync(
  join(out, "release-manifest.json"),
  JSON.stringify(
    {
      version,
      commit,
      dirty,
      platform: "macOS",
      architecture: "arm64",
      minimumSystemVersion: "13.0",
      signing: "ad-hoc",
      notarized: false,
      dependencyCount: notices.count,
      packagesWithoutStandaloneLicenseText: notices.missing,
    },
    null,
    2,
  ) + "\n",
);
const files = readdirSync(out).sort();
writeFileSync(
  join(out, "SHA256SUMS.txt"),
  files
    .map(
      (file) =>
        `${createHash("sha256")
          .update(readFileSync(join(out, file)))
          .digest("hex")}  ${file}`,
    )
    .join("\n") + "\n",
);
console.log(
  `Prepared ${files.length + 1} preview files; ${notices.count} dependency notices (${notices.missing.length} without standalone license text).`,
);
