import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = dirname(fileURLToPath(import.meta.url));
const targets = [
  join(root, "src-tauri", "tauri.conf.json"),
  join(root, "src-tauri", "Cargo.toml"),
  join(root, "package.json")
];

const versionRe = /"?version"?\s*[:=]\s*"(\d+)\.(\d+)\.(\d+)"/;
let newVersion = null;

for (const file of targets) {
  const text = readFileSync(file, "utf8");
  const match = text.match(versionRe);
  if (!match) {
    console.error("bump-version: no version field in " + file);
    process.exit(1);
  }
  const [, maj, min, patch] = match;
  const bumped = `${maj}.${min}.${Number(patch) + 1}`;
  if (!newVersion) {
    newVersion = bumped;
  } else if (bumped !== newVersion) {
    console.error(`bump-version: version mismatch in ${file}: ${bumped} vs ${newVersion}`);
    process.exit(1);
  }
}

for (const file of targets) {
  const text = readFileSync(file, "utf8");
  const updated = text.replace(versionRe, (m, maj, min, patch) => {
    const v = `${maj}.${min}.${Number(patch) + 1}`;
    return m.replace(`${maj}.${min}.${patch}`, v);
  });
  writeFileSync(file, updated, "utf8");
}

console.log("bump-version: " + newVersion);
