import { test } from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import ts from "typescript";

// Exercise production helpers without adding a browser test runtime or making API calls.
async function sourceModule(path) {
  const source = await readFile(new URL(path, import.meta.url), "utf8");
  const { outputText } = ts.transpileModule(source, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ES2022,
    },
  });
  return import(
    `data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`
  );
}
const { loadCurrent } = await sourceModule(
  "../../src/features/interview/navigation.ts",
);
const { summarizeCosts } = await sourceModule(
  "../../src/features/review/costs.ts",
);
const deferred = () => {
  let resolve;
  const promise = new Promise((r) => {
    resolve = r;
  });
  return { promise, resolve };
};

test("a late default restoration cannot overwrite explicit review navigation", async () => {
  let version = 0;
  const previous = deferred();
  const explicit = deferred();
  let shown;
  const restoring = loadCurrent(
    () => previous.promise,
    (value) => {
      shown = value;
    },
    () => version === 0,
  );
  version = 1;
  const navigating = loadCurrent(
    () => explicit.promise,
    (value) => {
      shown = value;
    },
    () => version === 1,
  );
  explicit.resolve("requested historical session");
  await navigating;
  previous.resolve("unrelated recent session");
  await restoring;
  assert.equal(shown, "requested historical session");
});

test("restoration is skipped when review focus exists before first mount", async () => {
  let loads = 0;
  await loadCurrent(
    async () => {
      loads++;
      return "recent";
    },
    () => assert.fail("must not restore"),
    () => false,
  );
  assert.equal(loads, 0);
});

test("late explicit requests and requests from an unmounted page cannot apply", async () => {
  let version = 1,
    mounted = true,
    shown = "current";
  const old = deferred();
  const first = loadCurrent(
    () => old.promise,
    (v) => {
      shown = v;
    },
    () => mounted && version === 1,
  );
  version = 2;
  await loadCurrent(
    async () => "newest",
    (v) => {
      shown = v;
    },
    () => mounted && version === 2,
  );
  old.resolve("outdated");
  await first;
  assert.equal(shown, "newest");
  const pending = deferred();
  const last = loadCurrent(
    () => pending.promise,
    (v) => {
      shown = v;
    },
    () => mounted,
  );
  mounted = false;
  pending.resolve("unmounted");
  await last;
  assert.equal(shown, "newest");
});

test("cost summaries distinguish unknown-only, mixed, zero and no requests", () => {
  assert.deepEqual(summarizeCosts([{ usd: null }]), {
    total: 0,
    known: 0,
    unknown: 1,
  });
  assert.deepEqual(summarizeCosts([{ usd: 0.25 }, { usd: null }, { usd: 0 }]), {
    total: 0.25,
    known: 2,
    unknown: 1,
  });
  assert.deepEqual(summarizeCosts([{ usd: 0 }]), {
    total: 0,
    known: 1,
    unknown: 0,
  });
  assert.deepEqual(summarizeCosts([]), { total: 0, known: 0, unknown: 0 });
});

const { appendTranscript } = await sourceModule(
  "../../src/features/voice/draft.ts",
);
test("voice insertion appends without overwriting a typed draft", () => {
  assert.equal(
    appendTranscript("Typed answer.", " 我用 Redis 做 cache-aside。 "),
    "Typed answer.\n\n我用 Redis 做 cache-aside。",
  );
  assert.equal(appendTranscript("", " spoken answer "), "spoken answer");
  assert.equal(appendTranscript("keep this", "   "), null);
});
test("voice insertion rejects overflow without truncating either input", () => {
  const draft = "x".repeat(5998),
    transcript = "中文";
  assert.equal(appendTranscript(draft, transcript), null);
  assert.equal(draft.length, 5998);
  assert.equal(transcript, "中文");
  assert.equal(appendTranscript("", "x".repeat(6000)).length, 6000);
});
