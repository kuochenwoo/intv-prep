import { resolve } from "node:path";
import { writeDependencyNotices } from "./notices.mjs";
const result = writeDependencyNotices(resolve("dist/release-notices"));
console.log(`Embedded ${result.count} dependency notices and MPL sources in the application assets.`);
