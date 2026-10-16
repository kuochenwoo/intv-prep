import { useEffect, useMemo, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke, isTauri } from "@tauri-apps/api/core";
import Editor, { loader } from "@monaco-editor/react";
import * as monaco from "monaco-editor";
import EditorWorker from "monaco-editor/editor/editor.worker.js?worker";
import "@fontsource-variable/figtree";
import "@astryxdesign/core/reset.css";
import "@astryxdesign/core/astryx.css";
import "@astryxdesign/theme-neutral/theme.css";
import { Theme } from "@astryxdesign/core/theme";
import { neutralTheme } from "@astryxdesign/theme-neutral/built";
import { InternationalizationProvider } from "@astryxdesign/core/i18n";
import zhCN from "@astryxdesign/core/locales/zh-CN.generated.js";
import { TopNav } from "@astryxdesign/core/TopNav";
import {
  SegmentedControl,
  SegmentedControlItem,
} from "@astryxdesign/core/SegmentedControl";
import { Button } from "@astryxdesign/core/Button";
import { TextInput } from "@astryxdesign/core/TextInput";
import { Selector } from "@astryxdesign/core/Selector";
import { Badge } from "@astryxdesign/core/Badge";
import { Banner } from "@astryxdesign/core/Banner";
import { AlertDialog } from "@astryxdesign/core/AlertDialog";
import "./style.css";
import logo from "./assets/logo.png";
import { registerUpdateGuard } from "./features/updates/guards";
import { UpdateProvider, UpdateNotice } from "./features/updates/UpdateCenter";
import SettingsPage from "./features/interview/SettingsPage";
import InterviewApp from "./features/interview/InterviewApp";
import SystemDesignApp from "./features/system-design/SystemDesignApp";
import { ReviewPage, type ReviewTarget } from "./features/review/ReviewPage";

/** A request from the Review tab to open something; `n` makes each request new. */
type Focus = { id: string; session: boolean; n: number };
import { LangProvider, lookup, useI18n } from "./i18n";
import { Timer } from "./Timer";
import { EmptyState } from "./EmptyState";
import { localizeError } from "./errors";
// Bundle Monaco locally: editing works without a CDN or network connection.
(self as unknown as { MonacoEnvironment: unknown }).MonacoEnvironment = {
  getWorker: () => new EditorWorker(),
};
loader.config({ monaco });
type Mode = "function" | "acm";
type Scheme = "light" | "dark";
type JudgeLimits = {
  timeLimitMs: number;
  memoryLimitMb: number;
  outputLimitKb: number;
};
type Problem = {
  id: string;
  title: string;
  difficulty: string;
  tags: string[];
  statement: string;
  constraints: string[];
  limits: JudgeLimits;
  complexity: {
    time: string;
    space: string;
    officialRequirement: string;
    notes: string;
    sourceUrl: string;
  };
  cpp: { functionTemplate: string; acmTemplate: string; acmIoSpec: string };
  examples: { input: string; output: string }[];
  en?: {
    title: string;
    tags: string[];
    statement: string;
    constraints: string[];
    complexity: Omit<Problem["complexity"], "sourceUrl">;
    acmIoSpec: string;
  };
};
// Every problem carries an English block; English mode shows it in place of the Chinese text.
function localProblem(p: Problem, lang: string): Problem {
  if (lang !== "en" || !p.en) return p;
  const { acmIoSpec, complexity, ...text } = p.en;
  return {
    ...p,
    ...text,
    complexity: { ...complexity, sourceUrl: p.complexity.sourceUrl },
    cpp: { ...p.cpp, acmIoSpec },
  };
}
type Progress = {
  problemId: string;
  mode: Mode;
  code: string;
  status: string;
  lastResult: string | null;
};
type Result = {
  status: string;
  passed: number;
  total: number;
  compiler: string;
  diagnostics: string;
  limits?: JudgeLimits;
  cases: {
    id: string;
    status: string;
    exitCode: number | null;
    input: string;
    expected: unknown;
    actual: string;
    stderr: string;
    durationMs: number;
    peakMemoryBytes?: number | null;
  }[];
};
const keyOf = (id: string, mode: Mode) => `${id}:${mode}`;
// Judge outcomes: green when accepted, orange for resource limits, red for wrong or broken code.
function resultTint(status: string) {
  if (status === "accepted") return "tint-green";
  if (
    ["timeout", "memory_limit", "output_limit", "compile_timeout"].includes(
      status,
    )
  )
    return "tint-orange";
  return "tint-red";
}
const difficultyBadge = {
  easy: "green",
  medium: "orange",
  hard: "red",
} as const;
function CodingApp({ scheme, focus }: { scheme: Scheme; focus?: Focus }) {
  const { t, lang } = useI18n();
  const [allProblems, setProblems] = useState<Problem[]>([]),
    [selected, setSelected] = useState("lc-1"),
    [mode, setMode] = useState<Mode>("function");
  const [progress, setProgress] = useState<Record<string, Progress>>({}),
    [results, setResults] = useState<Record<string, Result>>({});
  const [query, setQuery] = useState(""),
    [difficulty, setDifficulty] = useState("all"),
    [tag, setTag] = useState("all");
  const problems = useMemo(
    () => allProblems.map((p) => localProblem(p, lang)),
    [allProblems, lang],
  );
  useEffect(() => {
    if (!focus) return;
    setQuery("");
    setDifficulty("all");
    setTag("all");
    setSelected(focus.id);
  }, [focus?.n]);
  // Tag names differ per language, so a tag filter does not carry over.
  useEffect(() => setTag("all"), [lang]);
  const [error, setError] = useState(""),
    [busy, setBusy] = useState(false),
    [ready, setReady] = useState(false),
    [saving, setSaving] = useState(false);
  const saveQueue = useRef(Promise.resolve()),
    saveError = useRef(""),
    version = useRef(0);
  useEffect(() => registerUpdateGuard(async () => {
    if (busy) throw new Error("updatePracticeBusy");
    await saveQueue.current;
    if (saveError.current) throw new Error("updateDraftFailed");
  }), [busy]);
  useEffect(() => {
    if (!isTauri()) {
      setError(t("needDesktop"));
      return;
    }
    Promise.all([
      invoke<Problem[]>("list_problems"),
      invoke<Progress[]>("load_progress"),
    ])
      .then(([ps, rows]) => {
        setProblems(ps);
        setProgress(
          Object.fromEntries(rows.map((r) => [keyOf(r.problemId, r.mode), r])),
        );
        const rs: Record<string, Result> = {};
        for (const r of rows) {
          if (r.lastResult) {
            try {
              rs[keyOf(r.problemId, r.mode)] = JSON.parse(r.lastResult);
            } catch {
              /* damaged summary does not hide code */
            }
          }
        }
        setResults(rs);
        setReady(true);
      })
      .catch((e) => setError(String(e)));
    // Loads once; switching language later does not reload data.
  }, []);
  const problem = problems.find((p) => p.id === selected),
    key = keyOf(selected, mode),
    row = progress[key];
  const code =
    row?.code ??
    (mode === "function"
      ? problem?.cpp.functionTemplate
      : problem?.cpp.acmTemplate) ??
    "";
  function edit(value: string | undefined) {
    if (value === undefined || !problem) return;
    const id = selected,
      m = mode,
      k = key;
    const n = ++version.current;
    setProgress((prev) => ({
      ...prev,
      [k]: {
        problemId: id,
        mode: m,
        status: prev[k]?.status ?? "in_progress",
        lastResult: prev[k]?.lastResult ?? null,
        code: value,
      },
    }));
    setSaving(true);
    saveQueue.current = saveQueue.current
      .then(() => invoke("save_draft", { problemId: id, mode: m, code: value }))
      .then(() => {
        saveError.current = "";
      })
      .catch((e) => {
        saveError.current = String(e);
        setError(t("draftSaveFailed", { e: String(e) }));
      })
      .then(() => {
        if (version.current === n) setSaving(false);
      });
  }
  async function run(samples: boolean) {
    if (!problem) return;
    setBusy(true);
    setError("");
    try {
      await saveQueue.current;
      if (saveError.current) throw new Error(saveError.current);
      const r = await invoke<Result>("run_judge", {
        problemId: selected,
        mode,
        code,
        samples,
      });
      setResults((prev) => ({ ...prev, [key]: r }));
      const rows = await invoke<Progress[]>("load_progress");
      setProgress(
        Object.fromEntries(rows.map((r) => [keyOf(r.problemId, r.mode), r])),
      );
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  const [clearing, setClearing] = useState<"one" | "all" | null>(null);
  const savedProblems = new Set(
    Object.values(progress).map((r) => r.problemId),
  );
  async function clearSaved(scope: "one" | "all") {
    setBusy(true);
    setError("");
    try {
      // Let queued draft saves land first so none recreates the cleared code.
      await saveQueue.current;
      version.current += 1;
      setSaving(false);
      await invoke("clear_solutions", {
        problemId: scope === "one" ? selected : null,
      });
      const keep = (id: string) => scope === "one" && id !== selected;
      setProgress((prev) =>
        Object.fromEntries(
          Object.entries(prev).filter(([, r]) => keep(r.problemId)),
        ),
      );
      setResults((prev) =>
        Object.fromEntries(
          Object.entries(prev).filter(([k]) =>
            keep(k.slice(0, k.lastIndexOf(":"))),
          ),
        ),
      );
      setClearing(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  const result = results[key];
  const status = (s: string) => lookup(t, "status_", s);
  const visible = problems.filter(
    (p) =>
      (difficulty === "all" || difficulty === p.difficulty) &&
      (tag === "all" || p.tags.includes(tag)) &&
      `${p.id} ${p.title} ${p.tags.join(" ")}`
        .toLowerCase()
        .includes(query.toLowerCase()),
  );
  const progressLabel = (p: Problem, m: Mode) => {
    const r = progress[keyOf(p.id, m)];
    return r?.status === "completed" ? "done" : r ? "doing" : "todo";
  };
  return (
    <div className="coding-app">
      {error && (
        <div className="page-banner">
          <Banner
            status="error"
            title={localizeError(error, lang)}
            isDismissable
            onDismiss={() => setError("")}
          />
        </div>
      )}
      <main className="coding-main">
        <aside className="problem-list">
          <div className="list-head">
            <h2>{t("problems")}</h2>
            <Badge label={t("problemCount", { n: visible.length })} />
            <Button
              label={t("clearAllSolutions")}
              tooltip={t("clearAllSolutionsTip")}
              size="sm"
              variant="destructive"
              isDisabled={busy || !ready || savedProblems.size === 0}
              onClick={() => setClearing("all")}
            />
          </div>
          <TextInput
            label={t("searchProblems")}
            isLabelHidden
            placeholder={t("searchPlaceholder")}
            value={query}
            onChange={setQuery}
            hasClear
            size="sm"
            width="100%"
          />
          <div className="filters">
            <Selector
              label={t("difficulty")}
              isLabelHidden
              size="sm"
              width="100%"
              value={difficulty}
              onChange={setDifficulty}
              options={[
                { value: "all", label: t("allDifficulty") },
                { value: "easy", label: t("easy") },
                { value: "medium", label: t("medium") },
                { value: "hard", label: t("hard") },
              ]}
            />
            <Selector
              label={t("tag")}
              isLabelHidden
              size="sm"
              width="100%"
              hasSearch
              value={tag}
              onChange={setTag}
              options={[
                { value: "all", label: t("allTags") },
                ...[...new Set(problems.flatMap((p) => p.tags))].map((x) => ({
                  value: x,
                  label: x,
                })),
              ]}
            />
          </div>
          <div className="problem-scroll">
            {visible.map((p) => (
              <button
                className={`problem ${selected === p.id ? "active" : ""}`}
                key={p.id}
                disabled={busy}
                onClick={() => setSelected(p.id)}
              >
                <span className="problem-meta">
                  <span>{p.id.replace("lc-", "#")}</span>
                  <Badge
                    variant={
                      difficultyBadge[
                        p.difficulty as keyof typeof difficultyBadge
                      ] ?? "neutral"
                    }
                    label={lookup(t, "", p.difficulty)}
                  />
                </span>
                <strong>{p.title}</strong>
                <span className="progress-dots">
                  {(["function", "acm"] as Mode[]).map((m) => (
                    <span key={m} className={`dot ${progressLabel(p, m)}`}>
                      {m === "function" ? t("modeFunction") : t("modeAcm")}
                    </span>
                  ))}
                </span>
              </button>
            ))}
            {visible.length === 0 && ready && (
              <p className="muted">{t("noMatch")}</p>
            )}
          </div>
        </aside>
        <article className="statement">
          {problem && (
            <>
              <h1>{problem.title}</h1>
              <div className="tags">
                <Badge
                  variant={
                    difficultyBadge[
                      problem.difficulty as keyof typeof difficultyBadge
                    ] ?? "neutral"
                  }
                  label={lookup(t, "", problem.difficulty)}
                />
                {problem.tags.map((x) => (
                  <Badge key={x} label={x} />
                ))}
              </div>
              <p className="prose">{problem.statement}</p>
              <h3>{t("constraints")}</h3>
              <ul>
                {problem.constraints.map((c) => (
                  <li key={c}>{c}</li>
                ))}
              </ul>
              <section className="complexity-card">
                <h3>{t("complexity")}</h3>
                <div className="complexity-grid">
                  <span>{t("timeLabel")}</span>
                  <code>{problem.complexity.time}</code>
                  <span>{t("spaceLabel")}</span>
                  <code>{problem.complexity.space}</code>
                </div>
                <p>{problem.complexity.officialRequirement}</p>
                <p className="muted">{problem.complexity.notes}</p>
                <Button
                  label={t("officialSource")}
                  variant="ghost"
                  size="sm"
                  tooltip={problem.complexity.sourceUrl}
                  onClick={() => {
                    void invoke("open_problem_source", {
                      problemId: problem.id,
                    }).catch((e) => setError(String(e)));
                  }}
                />
              </section>
              <h3>{t("localLimits")}</h3>
              <p>
                {t("limitsLine", {
                  t: problem.limits.timeLimitMs,
                  m: problem.limits.memoryLimitMb,
                  o: problem.limits.outputLimitKb,
                })}
              </p>
              <p className="muted">{t("limitsNote")}</p>
              <p className="muted">{t("bigONote")}</p>
              <h3>{t("examples")}</h3>
              {problem.examples.map((ex, i) => (
                <pre key={i}>
                  {t("exampleIO", { i: ex.input, o: ex.output })}
                </pre>
              ))}
              <h3>{t("acmProtocol")}</h3>
              <p className="prose">{problem.cpp.acmIoSpec}</p>
              <p className="muted">{t("notSandbox")}</p>
            </>
          )}
        </article>
        <section className="workspace">
          <div className="editor-toolbar">
            <SegmentedControl
              label={t("codingMode")}
              size="sm"
              value={mode}
              isDisabled={busy}
              onChange={(v) => setMode(v as Mode)}
            >
              <SegmentedControlItem
                value="function"
                label={t("modeFunctionFull")}
              />
              <SegmentedControlItem value="acm" label={t("modeAcmFull")} />
            </SegmentedControl>
            <small className="muted">
              {saving ? t("saving") : t("saved")} ·{" "}
              {mode === "function" ? t("mainProvided") : t("mainYours")}
            </small>
            <Badge variant="purple" label={t("offlineJudge")} />
            <div className="coding-timer-right">
              <Timer key={selected} problemId={selected} />
            </div>
          </div>
          <div className="editor-frame">
            <Editor
              height="400px"
              language="cpp"
              theme={scheme === "dark" ? "vs-dark" : "vs"}
              value={code}
              path={key + ".cpp"}
              onChange={edit}
              options={{
                minimap: { enabled: false },
                fontSize: 14,
                scrollBeyondLastLine: false,
                automaticLayout: true,
                readOnly: busy || !ready,
              }}
            />
          </div>
          <div className="actions">
            <Button
              label={t("runSamples")}
              variant="secondary"
              className="tint-btn tint-blue"
              isDisabled={busy || !ready}
              onClick={() => run(true)}
            />
            <Button
              label={t("submitTests")}
              variant="primary"
              isDisabled={!ready}
              isLoading={busy}
              onClick={() => run(false)}
            />
            <Button
              label={t("clearSolution")}
              tooltip={t("clearSolutionTip")}
              variant="destructive"
              className="actions-end"
              isDisabled={busy || !ready || !savedProblems.has(selected)}
              onClick={() => setClearing("one")}
            />
          </div>
          <AlertDialog
            isOpen={clearing !== null}
            onOpenChange={(open) => {
              if (!open && !busy) setClearing(null);
            }}
            title={
              clearing === "all"
                ? t("clearAllSolutionsTitle")
                : t("clearSolutionTitle")
            }
            description={
              clearing === "all"
                ? t("clearAllSolutionsDescription", {
                    n: savedProblems.size,
                  })
                : t("clearSolutionDescription", {
                    title: problem?.title ?? selected,
                  })
            }
            actionLabel={t("clearConfirm")}
            actionVariant="destructive"
            cancelLabel={t("cancel")}
            isActionLoading={busy}
            onAction={() => clearing && clearSaved(clearing)}
          />
          <section className="results" aria-live="polite">
            {!result ? (
              <EmptyState icon="▶" text={t("resultsPlaceholder")} />
            ) : (
              <>
                <div
                  className={`result-head result-banner ${resultTint(result.status)}`}
                >
                  <Badge
                    variant={result.status === "accepted" ? "success" : "error"}
                    label={status(result.status)}
                  />
                  <strong>
                    {t("passedOf", { p: result.passed, t: result.total })}
                  </strong>
                </div>
                <small className="muted">
                  {result.compiler} · {t("lastRunNote")}
                  {result.limits &&
                    ` · ${t("runLimits", { t: result.limits.timeLimitMs, m: result.limits.memoryLimitMb })}`}
                </small>
                {result.diagnostics && <pre>{result.diagnostics}</pre>}
                {result.cases.map((c) => (
                  <details
                    key={c.id}
                    className={`case ${c.status === "accepted" ? "ok" : "bad"} ${resultTint(c.status)}`}
                    open={c.status !== "accepted"}
                  >
                    <summary>
                      {c.id} · {status(c.status)} · {c.durationMs} ms · exit{" "}
                      {c.exitCode ?? t("killed")} ·{" "}
                      {c.peakMemoryBytes != null
                        ? t("peakMemory", {
                            m: (c.peakMemoryBytes / 1024 / 1024).toFixed(1),
                          })
                        : t("noMemory")}
                    </summary>
                    <pre>
                      {t("caseStdin")}
                      {"\n"}
                      {c.input}
                      {"\n"}
                      {t("caseExpected")}
                      {JSON.stringify(c.expected)}
                      {"\n"}
                      {t("caseActual")}
                      {c.actual || t("emptyOutput")}
                      {c.stderr && `\nstderr: ${c.stderr}`}
                    </pre>
                  </details>
                ))}
                {result.cases.length > 0 &&
                  result.cases.length < result.total && (
                    <p className="muted">{t("stoppedEarly")}</p>
                  )}
              </>
            )}
          </section>
        </section>
      </main>
    </div>
  );
}
function storedScheme(): Scheme {
  try {
    const s = localStorage.getItem("scheme");
    if (s === "light" || s === "dark") return s;
  } catch {
    /* fall through to the system setting */
  }
  return matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
}
function App() {
  const { lang, setLang, t } = useI18n();
  const [view, setView] = useState<
    "lc" | "deep_dive" | "system_design" | "review" | "settings"
  >("lc");
  const [focus, setFocus] = useState<(Focus & { module: string }) | null>(null);
  const [due, setDue] = useState(0);
  const openFromReview = (target: ReviewTarget) => {
    setFocus({ ...target, n: Date.now() });
    if (target.module === "p1") setVisited(true);
    if (target.module === "p2") setDesignVisited(true);
    setView(
      target.module === "lc"
        ? "lc"
        : target.module === "p1"
          ? "deep_dive"
          : "system_design",
    );
    window.scrollTo(0, 0);
  };
  const focusFor = (module: string) =>
    focus?.module === module ? focus : undefined;
  const [settingsVersion, setSettingsVersion] = useState(0);
  const [settingsVisited, setSettingsVisited] = useState(false);
  const [visited, setVisited] = useState(false);
  const [designVisited, setDesignVisited] = useState(false);
  const [scheme, setScheme] = useState<Scheme>(storedScheme);
  useEffect(() => {
    document.documentElement.lang = lang === "zh" ? "zh-CN" : "en";
  }, [lang]);
  const changeScheme = (s: Scheme) => {
    setScheme(s);
    try {
      localStorage.setItem("scheme", s);
    } catch {
      /* not remembered */
    }
  };
  return (
    <Theme theme={neutralTheme} mode={scheme}>
      <InternationalizationProvider
        locale={lang === "zh" ? "zh-CN" : "en"}
        messages={{ "zh-CN": zhCN }}
      >
        <div className="app">
          <UpdateNotice />
          <TopNav
            label={t("moduleNav")}
            heading={
              <span className="brand">
                <img src={logo} alt="" width={32} height={32} />
                <strong>{t("appTitle")}</strong>
              </span>
            }
            centerContent={
              <SegmentedControl
                label={t("moduleNav")}
                value={view}
                onChange={(v) => {
                  if (v === "deep_dive") setVisited(true);
                  if (v === "system_design") setDesignVisited(true);
                  if (v === "settings") setSettingsVisited(true);
                  setView(
                    v as
                      | "lc"
                      | "deep_dive"
                      | "system_design"
                      | "review"
                      | "settings",
                  );
                }}
              >
                <SegmentedControlItem value="lc" label={t("moduleLc")} />
                <SegmentedControlItem
                  value="deep_dive"
                  label={t("moduleDeepDive")}
                />
                <SegmentedControlItem
                  value="system_design"
                  label={t("moduleSystemDesign")}
                />
                <SegmentedControlItem
                  value="review"
                  label={
                    due > 0
                      ? `${t("moduleReview")} · ${due}`
                      : t("moduleReview")
                  }
                />
                <SegmentedControlItem
                  value="settings"
                  label={t("settingsPage")}
                />
              </SegmentedControl>
            }
            endContent={
              <span className="nav-actions">
                <SegmentedControl
                  label={t("themeMode")}
                  size="sm"
                  value={scheme}
                  onChange={(v) => changeScheme(v as Scheme)}
                >
                  <SegmentedControlItem value="light" label={t("themeLight")} />
                  <SegmentedControlItem value="dark" label={t("themeDark")} />
                </SegmentedControl>
                <Button
                  label={t("switchLang")}
                  tooltip={t("switchLangTip")}
                  size="sm"
                  onClick={() => setLang(lang === "zh" ? "en" : "zh")}
                />
              </span>
            }
          />
          <div hidden={view !== "lc"}>
            <CodingApp scheme={scheme} focus={focusFor("lc")} />
          </div>
          <div hidden={view !== "review"}>
            <ReviewPage
              active={view === "review"}
              onNavigate={openFromReview}
              onDue={setDue}
            />
          </div>
          {settingsVisited && (
            <div hidden={view !== "settings"}>
              <SettingsPage
                active={view === "settings"}
                onSaved={() => setSettingsVersion((n) => n + 1)}
              />
            </div>
          )}
          {designVisited && (
            <div hidden={view !== "system_design"}>
              <SystemDesignApp
                visible={view === "system_design"}
                dark={scheme === "dark"}
                settingsVersion={settingsVersion}
                focus={focusFor("p2")}
              />
            </div>
          )}
          {visited && (
            <div hidden={view !== "deep_dive"}>
              <InterviewApp
                visible={view === "deep_dive"}
                settingsVersion={settingsVersion}
                focus={focusFor("p1")}
              />
            </div>
          )}
        </div>
      </InternationalizationProvider>
    </Theme>
  );
}
createRoot(document.getElementById("root")!).render(
  <LangProvider>
    <UpdateProvider><App /></UpdateProvider>
  </LangProvider>,
);
