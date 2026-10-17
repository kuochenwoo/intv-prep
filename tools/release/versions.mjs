import { valid, prerelease } from "semver";

/** Accept canonical stable versions and the existing numbered preview channels. */
export function releaseKind(version) {
  if (
    typeof version !== "string" ||
    valid(version) !== version ||
    !/^\d+\.\d+\.\d+(?:-(?:preview|alpha|beta|rc)\.\d+)?$/.test(version)
  )
    throw Error(
      "A canonical stable or numbered prerelease version is required.",
    );
  const isPrerelease = prerelease(version) !== null;
  return { isPrerelease, latest: !isPrerelease };
}
export function validatePublishedRelease(release, tag) {
  const version =
    typeof tag === "string" && tag.startsWith("v") ? tag.slice(1) : "";
  const { isPrerelease } = releaseKind(version);
  if (
    release.isDraft ||
    release.tagName !== tag ||
    release.isPrerelease !== isPrerelease
  )
    throw Error("Only the matching published version can update this feed.");
  return version;
}
