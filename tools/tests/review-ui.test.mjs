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

// Render the real shared component; this exercises parsing and escaping, not a mock renderer.
const { buildSync } = await import("esbuild");
const { createRequire } = await import("node:module");
const { fileURLToPath } = await import("node:url");
const markdownBundle = buildSync({
  stdin: {
    contents: `import { Markdown } from './src/Markdown';
      import { renderToStaticMarkup } from 'react-dom/server';
      import { createElement } from 'react';
      export const render = (text) => renderToStaticMarkup(createElement(Markdown, { text }));`,
    resolveDir: fileURLToPath(new URL("../../", import.meta.url)),
    loader: "tsx",
  },
  platform: "node",
  format: "cjs",
  bundle: true,
  write: false,
  loader: { ".css": "empty" },
});
const markdownModule = { exports: {} };
new Function(
  "module",
  "exports",
  "require",
  markdownBundle.outputFiles[0].text,
)(markdownModule, markdownModule.exports, createRequire(import.meta.url));
const renderMarkdown = markdownModule.exports.render;

test("interview Markdown preserves fenced code, language tags, indentation and line breaks", () => {
  const html = renderMarkdown(
    "比较 `count` 的更新：\n\n```javascript\nif (ready) {\n  count += 1;\n}\n```",
  );
  assert.match(html, /<code>count<\/code>/);
  assert.match(html, /<pre><code class="language-javascript">/);
  assert.match(html, /if \(ready\) \{\n  count \+= 1;\n\}\n/);
  assert.doesNotMatch(html, /```/);
});

test("interview Markdown supports lists, tables and incomplete streaming fences", () => {
  const html = renderMarkdown(
    "- first\n- second\n\n| A | B |\n| --- | --- |\n| 1 | 2 |",
  );
  assert.match(html, /<ul>/);
  assert.match(html, /<table>/);
  assert.match(html, /<td>2<\/td>/);
  assert.match(
    renderMarkdown("```sql\nSELECT id\n  FROM items"),
    /<pre><code class="language-sql">SELECT id\n  FROM items/,
  );
});

test("interview Markdown blocks HTML, external images and navigable links", () => {
  const html = renderMarkdown(
    '<script>alert(1)</script>\n\n<img src="https://example.com/beacon" onerror="alert(1)">\n\n![tracking](https://example.com/pixel)\n\n[reference](https://example.com)\n\n[bad](javascript:alert%281%29)',
  );
  assert.doesNotMatch(html, /<(script|img|a)\b|onerror=|href=|src=/);
  assert.match(html, /reference/);
});

test("a generated question cannot open its session after selection moves to another topic", async () => {
  const generated = deferred();
  let selected = "cache",
    opened = null,
    starts = 0;
  const operation = loadCurrent(
    async () => {
      const question = await generated.promise;
      if (selected !== "cache") return null;
      starts++;
      return { question, seedId: "cache" };
    },
    (session) => {
      opened = session;
    },
    () => selected === "cache",
  );
  selected = "network";
  generated.resolve("real-request-result-placeholder");
  await operation;
  assert.equal(selected, "network");
  assert.equal(starts, 0);
  assert.equal(opened, null);
});

test("a late session or failed draft save cannot replace the chosen view", async () => {
  const session = deferred();
  let version = 0,
    view = "original";
  const operation = loadCurrent(
    () => session.promise,
    (value) => {
      view = value;
    },
    () => version === 0,
  );
  version++;
  view = "another topic";
  session.resolve("old interview");
  await operation;
  assert.equal(view, "another topic");
  await assert.rejects(
    loadCurrent(
      async () => {
        throw new Error("draft save failed");
      },
      () => {
        view = "lost draft";
      },
      () => true,
    ),
    /draft save failed/,
  );
  assert.equal(view, "another topic");
});

const { registerUpdateGuard, prepareFrontendUpdate } = await sourceModule("../../src/features/updates/guards.ts");
const { downloadPercent } = await sourceModule("../../src/features/updates/policy.ts");
test("updates await actual draft-save guards and unregister closed practice pages", async () => {
  const saved = deferred();
  let done = false;
  const unregister = registerUpdateGuard(() => saved.promise);
  try {
    const updating = prepareFrontendUpdate().then(() => { done = true; });
    await Promise.resolve();
    assert.equal(done, false);
    saved.resolve(); await updating;
    assert.equal(done, true);
  } finally { unregister(); }
  await prepareFrontendUpdate();
});
test("unsaved drafts or temporary voice data stop update preparation", async () => {
  const unregister = registerUpdateGuard(() => { throw new Error("updateVoicePending"); });
  try { await assert.rejects(prepareFrontendUpdate(), /updateVoicePending/); }
  finally { unregister(); }
  await prepareFrontendUpdate();
});
test("update download progress remains indeterminate without a length and never claims verified completion", () => {
  assert.equal(downloadPercent(500), undefined);
  assert.equal(downloadPercent(500, 1000), 50);
  assert.equal(downloadPercent(1000, 1000), 99);
  assert.equal(downloadPercent(1200, 1000), 99);
});

const { localizeError } = await sourceModule("../../src/errors.ts");
const { nextMidnight, notifyReviewChanged, watchReviewChanges } = await sourceModule(
  "../../src/features/review/events.ts",
);
test("errors remove JavaScript prefixes in both languages and translate Review errors", () => {
  assert.equal(localizeError("Error: 无效的复习条目", "zh"), "无效的复习条目");
  assert.equal(localizeError("Error: 无效的复习条目", "en"), "Invalid review item");
  assert.equal(localizeError("草稿保存失败：Error: disk full", "zh"), "草稿保存失败：disk full");
});
test("Review invalidation covers local mutations, next midnight, wake-up and cleanup", () => {
  const originals = { window: globalThis.window, document: globalThis.document, setTimeout, clearTimeout };
  const window = new EventTarget(), document = new EventTarget();
  let scheduled, cancelled, calls = 0;
  Object.assign(globalThis, { window, document,
    setTimeout: (callback, delay) => { scheduled = { callback, delay }; return 17; },
    clearTimeout: (id) => { cancelled = id; },
  });
  try {
    const now = new Date(2026, 9, 17, 23, 59, 59).getTime();
    assert.equal(nextMidnight(now) - now, 1000);
    const stop = watchReviewChanges(() => calls++);
    assert.ok(scheduled.delay > 0 && scheduled.delay <= 25 * 60 * 60 * 1000);
    notifyReviewChanged();
    assert.equal(calls, 1);
    scheduled.callback();
    assert.equal(calls, 2);
    window.dispatchEvent(new Event("focus"));
    document.dispatchEvent(new Event("visibilitychange"));
    assert.equal(calls, 4);
    stop();
    assert.equal(cancelled, 17);
    notifyReviewChanged();
    window.dispatchEvent(new Event("focus"));
    document.dispatchEvent(new Event("visibilitychange"));
    assert.equal(calls, 4);
  } finally { Object.assign(globalThis, originals); }
});
