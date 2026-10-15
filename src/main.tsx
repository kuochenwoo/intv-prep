import { useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke, isTauri } from "@tauri-apps/api/core";
import Editor, { loader } from "@monaco-editor/react";
import * as monaco from "monaco-editor";
import EditorWorker from "monaco-editor/editor/editor.worker.js?worker";
import "./style.css";
import InterviewApp from "./features/interview/InterviewApp";
// Bundle Monaco locally: editing works without a CDN or network connection.
(self as unknown as { MonacoEnvironment: unknown }).MonacoEnvironment = {
  getWorker: () => new EditorWorker(),
};
loader.config({ monaco });
type Mode = "function" | "acm";
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
};
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
const labels: Record<string, string> = {
  accepted: "通过",
  wrong_answer: "答案错误",
  compile_error: "编译错误",
  runtime_error: "运行错误",
  timeout: "运行超时",
  output_limit: "输出超限",
  memory_limit: "記憶體超限",
  compile_timeout: "编译超时",
  compile_output_limit: "编译输出超限",
};
const keyOf = (id: string, mode: Mode) => `${id}:${mode}`;
function CodingApp() {
  const [problems, setProblems] = useState<Problem[]>([]),
    [selected, setSelected] = useState("lc-1"),
    [mode, setMode] = useState<Mode>("function");
  const [progress, setProgress] = useState<Record<string, Progress>>({}),
    [results, setResults] = useState<Record<string, Result>>({});
  const [query, setQuery] = useState(""),
    [difficulty, setDifficulty] = useState("all"),
    [tag, setTag] = useState("all");
  const [compiler, setCompiler] = useState(""),
    [compilerInfo, setCompilerInfo] = useState(""),
    [error, setError] = useState(""),
    [busy, setBusy] = useState(false),
    [ready, setReady] = useState(false),
    [saving, setSaving] = useState(false);
  const saveQueue = useRef(Promise.resolve()),
    saveError = useRef(""),
    version = useRef(0);
  useEffect(() => {
    if (!isTauri()) {
      setError(
        "请通过 npm run tauri dev 打开桌面应用。浏览器预览无法调用本地编译器。",
      );
      return;
    }
    Promise.all([
      invoke<Problem[]>("list_problems"),
      invoke<Progress[]>("load_progress"),
      invoke<string>("get_compiler"),
    ])
      .then(([ps, rows, path]) => {
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
        setCompiler(path);
        setReady(true);
      })
      .catch((e) => setError(String(e)));
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
        setError(`草稿保存失败：${e}`);
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
  async function configure() {
    setBusy(true);
    setError("");
    try {
      const path = await invoke<string>("set_compiler", { path: compiler });
      setCompiler(path);
      setCompilerInfo(
        `可执行文件已检测：${path}；判题时使用 -std=c++20 实际编译。`,
      );
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  const result = results[key];
  return (
    <>
      <header>
        <div>
          <strong>SWE Interview Prep</strong>
          <span>本地 Coding</span>
        </div>
        <span className="badge">C++20 · 离线判题</span>
      </header>
      <section className="settings">
        <label>
          编译器{" "}
          <input
            aria-label="编译器路径"
            placeholder="自动探测 clang++ / g++，或填写完整路径"
            value={compiler}
            onChange={(e) => setCompiler(e.target.value)}
            disabled={busy}
          />
        </label>
        <button disabled={!ready || busy} onClick={configure}>
          检测并保存
        </button>
        <small>{compilerInfo}</small>
      </section>
      {error && (
        <div role="alert" className="error">
          {error}
        </div>
      )}
      <main>
        <aside>
          <h2>
            练习题 <small>{problems.length} 道已适配</small>
          </h2>
          <input
            aria-label="搜索题目"
            placeholder="搜索标题 / 编号 / 标签"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <div className="filters">
            <select
              aria-label="难度"
              value={difficulty}
              onChange={(e) => setDifficulty(e.target.value)}
            >
              <option value="all">全部难度</option>
              <option value="easy">简单</option>
              <option value="medium">中等</option>
              <option value="hard">困难</option>
            </select>
            <select
              aria-label="标签"
              value={tag}
              onChange={(e) => setTag(e.target.value)}
            >
              <option value="all">全部标签</option>
              {[...new Set(problems.flatMap((p) => p.tags))].map((t) => (
                <option key={t}>{t}</option>
              ))}
            </select>
          </div>
          {problems
            .filter(
              (p) =>
                (difficulty === "all" || difficulty === p.difficulty) &&
                (tag === "all" || p.tags.includes(tag)) &&
                `${p.id} ${p.title} ${p.tags.join(" ")}`
                  .toLowerCase()
                  .includes(query.toLowerCase()),
            )
            .map((p) => (
              <button
                className={`problem ${selected === p.id ? "active" : ""}`}
                key={p.id}
                disabled={busy}
                onClick={() => setSelected(p.id)}
              >
                <small>
                  {p.id} ·{" "}
                  {p.difficulty === "easy"
                    ? "简单"
                    : p.difficulty === "medium"
                      ? "中等"
                      : "困难"}
                </small>
                <strong>{p.title}</strong>
                <small>
                  {(["function", "acm"] as Mode[])
                    .map(
                      (m) =>
                        `${m === "function" ? "函数" : "ACM"} ${progress[keyOf(p.id, m)]?.status === "completed" ? "✓ 完成" : progress[keyOf(p.id, m)] ? "在做" : "未做"}`,
                    )
                    .join(" · ")}
                </small>
              </button>
            ))}
          <p className="muted">
            仅包含 4 道自行整理的题面和本地测试，尚未覆盖 Hot 100。
          </p>
        </aside>
        <article>
          {problem && (
            <>
              <h1>{problem.title}</h1>
              <div className="tags">
                {problem.tags.map((t) => (
                  <span key={t}>{t}</span>
                ))}
              </div>
              <p>{problem.statement}</p>
              <h3>约束</h3>
              <ul>
                {problem.constraints.map((c) => (
                  <li key={c}>{c}</li>
                ))}
              </ul>
              <h3>時間／空間複雜度要求</h3>
              <p>
                時間：{problem.complexity.time}
                <br />
                額外空間：{problem.complexity.space}
              </p>
              <p>{problem.complexity.officialRequirement}</p>
              <p className="muted">{problem.complexity.notes}</p>
              <p>
                <button
                  className="source-link"
                  title={problem.complexity.sourceUrl}
                  onClick={() => {
                    void invoke("open_problem_source", {
                      problemId: problem.id,
                    }).catch((e) => setError(String(e)));
                  }}
                >
                  官方題面參考（需聯網）
                </button>
              </p>
              <h3>本地執行上限</h3>
              <p>
                每例 {problem.limits.timeLimitMs} ms · 主進程記憶體{" "}
                {problem.limits.memoryLimitMb} MiB · 輸出{" "}
                {problem.limits.outputLimitKb} KiB
              </p>
              <p className="muted">
                此為本地設定，非 LeetCode
                官方資源限額。記憶體包含輸入、包裝器及執行期，每 5 ms
                採樣，可能漏計短暫峰值；不等同演算法額外空間。
              </p>
              <p className="muted">
                通過本地測試不會自動證明 Big-O
                或原地要求已滿足，仍需檢查程式的複雜度。
              </p>
              <h3>示例（逻辑数据）</h3>
              {problem.examples.map((ex, i) => (
                <pre key={i}>
                  输入：{ex.input}
                  {"\n"}输出：{ex.output}
                </pre>
              ))}
              <h3>ACM 输入输出协议</h3>
              <p>{problem.cpp.acmIoSpec}</p>
              {selected === "lc-206" && (
                <pre>
                  struct ListNode {"{"} int val; ListNode* next;{"\n"}{" "}
                  ListNode(int x=0, ListNode* n=nullptr); {"}"};
                </pre>
              )}
              <p className="muted">
                此程序以当前用户权限执行本人可信代码，不是安全沙箱。
              </p>
            </>
          )}
        </article>
        <section className="workspace">
          <div className="toolbar">
            <select
              aria-label="编程模式"
              disabled={busy}
              value={mode}
              onChange={(e) => setMode(e.target.value as Mode)}
            >
              <option value="function">LeetCode 核心函数</option>
              <option value="acm">ACM 完整程序</option>
            </select>
            <small>
              {saving ? "正在保存…" : "草稿已保存"} ·{" "}
              {mode === "function" ? "系统提供 main()" : "自行实现 main()"}
            </small>
          </div>
          <Editor
            height="390px"
            language="cpp"
            theme="vs-dark"
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
          <div className="actions">
            <button disabled={busy || !ready} onClick={() => run(true)}>
              运行样例
            </button>
            <button
              className="primary"
              disabled={busy || !ready}
              onClick={() => run(false)}
            >
              {busy ? "处理中…" : "提交本地测试"}
            </button>
          </div>
          <section className="results" aria-live="polite">
            {!result ? (
              <p className="muted">编译和运行结果将显示在这里。</p>
            ) : (
              <>
                <h3 className={result.status === "accepted" ? "pass" : "fail"}>
                  {labels[result.status] ?? result.status} · {result.passed}/
                  {result.total} 通过
                </h3>
                <small>
                  {result.compiler} ·
                  上次执行结果（样例或提交）；代码修改后不会自动重判
                  {result.limits &&
                    ` · 當次上限 ${result.limits.timeLimitMs} ms / ${result.limits.memoryLimitMb} MiB`}
                </small>
                {result.diagnostics && <pre>{result.diagnostics}</pre>}
                {result.cases.map((c) => (
                  <details key={c.id} open={c.status !== "accepted"}>
                    <summary>
                      {c.id} · {labels[c.status] ?? c.status} · {c.durationMs}{" "}
                      ms · exit {c.exitCode ?? "signal / killed"}
                      {c.peakMemoryBytes != null
                        ? ` · 採樣峰值 ${(c.peakMemoryBytes / 1024 / 1024).toFixed(1)} MiB`
                        : " · 記憶體未取得（短程式或舊記錄）"}
                    </summary>
                    <pre>
                      stdin：{"\n"}
                      {c.input}
                      {"\n"}预期（任意合法答案亦可）：
                      {JSON.stringify(c.expected)}
                      {"\n"}实际：{c.actual || "（空输出）"}
                      {c.stderr && `\nstderr：${c.stderr}`}
                    </pre>
                  </details>
                ))}
                {result.cases.length > 0 &&
                  result.cases.length < result.total && (
                    <p>超出時間、記憶體或輸出限制後已停止後續測試。</p>
                  )}
              </>
            )}
          </section>
        </section>
      </main>
    </>
  );
}
function App() {
  const [view, setView] = useState<"lc" | "deep_dive">("lc");
  const [visited, setVisited] = useState(false);
  return (
    <>
      <nav className="module-nav" aria-label="训练模块">
        <button
          className={view === "lc" ? "selected" : ""}
          onClick={() => setView("lc")}
        >
          LC 本地判题
        </button>
        <button
          className={view === "deep_dive" ? "selected" : ""}
          onClick={() => {
            setVisited(true);
            setView("deep_dive");
          }}
        >
          Technical Deep Dive
        </button>
      </nav>
      <div hidden={view !== "lc"}>
        <CodingApp />
      </div>
      {visited && (
        <div hidden={view !== "deep_dive"}>
          <InterviewApp />
        </div>
      )}
    </>
  );
}
createRoot(document.getElementById("root")!).render(<App />);
