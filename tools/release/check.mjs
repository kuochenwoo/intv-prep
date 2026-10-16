import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

export function validateRelease({ pkg, lock, cargo, cargoLock, tauri }, tag) {
  const version = pkg.version;
  if (!/^\d+\.\d+\.\d+-(?:preview|alpha|beta|rc)\.\d+$/.test(version))
    throw Error("This preview pipeline requires a prerelease version.");
  const cargoVersion = cargo.match(
    /\[package\][\s\S]*?\nversion\s*=\s*"([^"]+)"/,
  )?.[1];
  const lockedCargoVersion = cargoLock.match(
    /\[\[package\]\]\nname = "swe-interview-prep"\nversion = "([^"]+)"/,
  )?.[1];
  if (
    [
      lock.version,
      lock.packages[""].version,
      cargoVersion,
      lockedCargoVersion,
      tauri.version,
    ].some((v) => v !== version)
  )
    throw Error("Package, lockfile, Cargo and Tauri versions must match.");
  if (tag && tag !== `v${version}`)
    throw Error("Release tag does not match the application version.");
  if (!tauri.bundle.active || tauri.bundle.macOS.signingIdentity !== "-")
    throw Error("Preview packaging must be enabled and ad-hoc signed.");
  if (tauri.identifier !== "local.swe.interview-prep")
    throw Error(
      "Changing the application identifier would lose the existing data location.",
    );
  if (pkg.license !== "UNLICENSED" || !pkg.private)
    throw Error(
      "The preview must retain its proprietary/private-package declaration.",
    );
  const updater = tauri.plugins?.updater;
  if (tauri.bundle.createUpdaterArtifacts !== true || !updater?.pubkey?.trim() ||
      updater.requireSignedVersion !== true || updater.allowDowngrades ||
      updater.dangerousInsecureTransportProtocol || updater.dangerousAcceptInvalidCerts ||
      JSON.stringify(updater.endpoints) !== JSON.stringify(["https://github.com/kuochenwoo/intv-prep/releases/download/updates-preview/latest.json"]))
    throw Error("Updater artifacts, signed versions and the fixed HTTPS preview feed are required.");
  return version;
}
export function readRelease(root = process.cwd(), tag) {
  const json = (file) => JSON.parse(readFileSync(resolve(root, file), "utf8"));
  const version = validateRelease(
    {
      pkg: json("package.json"),
      lock: json("package-lock.json"),
      cargo: readFileSync(resolve(root, "src-tauri/Cargo.toml"), "utf8"),
      cargoLock: readFileSync(resolve(root, "src-tauri/Cargo.lock"), "utf8"),
      tauri: json("src-tauri/tauri.conf.json"),
    },
    tag,
  );
  if (!existsSync(resolve(root, `docs/releases/v${version}.md`)))
    throw Error("Versioned release notes are missing.");
  if (!existsSync(resolve(root, "LICENSE")))
    throw Error("The proprietary preview license is missing.");
  return version;
}
if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href)
  console.log(
    `Release configuration OK: v${readRelease(process.cwd(), process.argv[2])}`,
  );
