import { valid, lt } from "semver";
export const UPDATE_REPOSITORY = "kuochenwoo/intv-prep";
export const UPDATE_CHANNEL_TAG = "updates-preview";
export function updaterFile(version) {
  return `SWE-Prep_${version}_macos-arm64.app.tar.gz`;
}
export function validateUpdateFeed(feed) {
  if (!valid(feed?.version)) throw Error("Invalid update version.");
  const platform = feed.platforms?.["darwin-aarch64"];
  const expected = `https://github.com/${UPDATE_REPOSITORY}/releases/download/v${feed.version}/${updaterFile(feed.version)}`;
  if (platform?.url !== expected)
    throw Error("Update URL must point to its immutable release asset.");
  const signature = platform.signature;
  if (typeof signature !== "string" || !/^[A-Za-z0-9+/=]+$/.test(signature))
    throw Error("Missing updater signature.");
  // Cryptographic verification is performed by verify_update and again by the installed app.
  const comment = Buffer.from(signature, "base64")
    .toString("utf8")
    .split("\n")
    .find((line) => line.startsWith("trusted comment: "))
    ?.slice("trusted comment: ".length);
  if (!comment?.split("\t").includes(`version:${feed.version}`))
    throw Error("Signature must be bound to the manifest version.");
  return feed;
}
export function createUpdateFeed(
  version,
  signature,
  notes,
  pubDate = new Date().toISOString(),
) {
  return validateUpdateFeed({
    version,
    notes,
    pub_date: pubDate,
    platforms: {
      "darwin-aarch64": {
        signature: signature.trim(),
        url: `https://github.com/${UPDATE_REPOSITORY}/releases/download/v${version}/${updaterFile(version)}`,
      },
    },
  });
}
export function shouldPublishFeed(next, current) {
  validateUpdateFeed(next);
  if (!current) return true;
  validateUpdateFeed(current);
  if (lt(next.version, current.version))
    throw Error("Refusing to move the update feed backwards.");
  if (next.version === current.version) {
    if (JSON.stringify(next.platforms) !== JSON.stringify(current.platforms))
      throw Error("A published update version is immutable.");
    return false;
  }
  return true;
}
