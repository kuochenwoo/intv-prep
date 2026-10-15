import { useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import "./interview.css";
type Config = { provider: string; model: string; timeoutSeconds: number };
type Settings = { config: Config; keyStatus: string };
type Topic = {
  id: string;
  category: string;
  title: string;
  focus: string[];
  scenarioConstraints: string[];
};
type Question = {
  id: string;
  seedId: string;
  provider: string;
  model: string;
  question: string;
  background: string;
  constraints: string[];
  createdAt: string;
};
type Feedback = {
  summary: string;
  dimensions: {
    name: string;
    rating: string;
    evidence: string;
    improvement: string;
  }[];
  strengths: string[];
  gaps: string[];
  nextPracticeTopics: string[];
};
type Session = {
  id: string;
  question: Question;
  provider: string;
  model: string;
  status: string;
  draft: string;
  lastError: string | null;
  shouldEnd: boolean;
  turns: { role: string; text: string; timestamp: string }[];
  feedback: Feedback | null;
  createdAt: string;
};
type Summary = {
  id: string;
  seedId: string;
  title: string;
  status: string;
  provider: string;
  model: string;
  createdAt: string;
};
type Overview = {
  topics: Topic[];
  questions: Question[];
  sessions: Summary[];
  settings: Settings;
};
const states: Record<string, string> = {
  active: "进行中",
  awaiting_reply: "等待追问 / 可重试",
  feedback_pending: "等待反馈 / 可重试",
  finished: "已结束",
};
const ratings: Record<string, string> = {
  good: "良好",
  partial: "部分具备",
  missing: "遗漏",
  incorrect: "有误",
  uncertain: "待确认",
};
const dimensions: Record<string, string> = {
  principles: "原理理解",
  selection_rationale: "技术选型依据",
  tradeoffs: "替代方案权衡",
  failure_and_correctness: "故障边界与正确性",
  communication: "沟通清晰度",
};
export default function InterviewApp() {
  const [data, setData] = useState<Overview | null>(null),
    [topicId, setTopicId] = useState("cache-selection"),
    [questionId, setQuestionId] = useState("");
  const [session, setSession] = useState<Session | null>(null),
    [draft, setDraft] = useState("");
  const [config, setConfig] = useState<Config>({
      provider: "deepseek",
      model: "deepseek-flash",
      timeoutSeconds: 60,
    }),
    [apiKey, setApiKey] = useState("");
  const [busy, setBusy] = useState(false),
    [saving, setSaving] = useState(false),
    [error, setError] = useState(""),
    [connection, setConnection] = useState("API 尚未验证");
  const [online, setOnline] = useState(navigator.onLine),
    [confirm, setConfirm] = useState<"key" | "history" | "session" | null>(
      null,
    );
  const queue = useRef(Promise.resolve()),
    saveError = useRef(""),
    revision = useRef(0);
  function show(s: Session) {
    setSession(s);
    setDraft(s.draft);
  }
  async function overview() {
    const result = await invoke<Overview>("interview_overview");
    setData(result);
    setConfig(result.settings.config);
    return result;
  }
  useEffect(() => {
    const update = () => setOnline(navigator.onLine);
    window.addEventListener("online", update);
    window.addEventListener("offline", update);
    if (isTauri())
      overview()
        .then(async (result) => {
          const previous =
            result.sessions.find((s) => s.status !== "finished") ??
            result.sessions[0];
          if (previous)
            show(
              await invoke<Session>("get_interview_session", {
                sessionId: previous.id,
              }),
            );
        })
        .catch((e) => setError(String(e)));
    return () => {
      window.removeEventListener("online", update);
      window.removeEventListener("offline", update);
    };
  }, []);
  function save(value: string) {
    setDraft(value);
    if (!session || session.status !== "active") return;
    const id = session.id,
      version = ++revision.current;
    setSaving(true);
    queue.current = queue.current
      .then(() =>
        invoke("save_interview_draft", { sessionId: id, draft: value }),
      )
      .then(() => {
        saveError.current = "";
      })
      .catch((e) => {
        saveError.current = String(e);
        setError(`回答草稿保存失败：${e}`);
      })
      .then(() => {
        if (revision.current === version) setSaving(false);
      });
  }
  async function flush() {
    await queue.current;
    if (saveError.current) throw new Error(saveError.current);
  }
  async function perform(task: () => Promise<void>, network = false) {
    setBusy(true);
    setError("");
    if (network) setConnection("正在请求 DeepSeek…");
    try {
      await flush();
      await task();
      await overview();
      if (network) setConnection("上次 API 请求成功");
    } catch (e) {
      setError(String(e));
      if (network) setConnection("上次联网操作失败，可重试");
      if (session && !saveError.current) {
        try {
          show(
            await invoke<Session>("get_interview_session", {
              sessionId: session.id,
            }),
          );
        } catch {
          /* retain current local text on refresh failure */
        }
      }
    } finally {
      setBusy(false);
    }
  }
  const topic = data?.topics.find((t) => t.id === topicId),
    cached = data?.questions.filter((q) => q.seedId === topicId) ?? [];
  const preview = cached.find((q) => q.id === questionId) ?? cached[0];
  const pending =
    session?.status === "awaiting_reply" ||
    session?.status === "feedback_pending";
  return (
    <div className="interview-app">
      <header>
        <div>
          <strong>Technical Deep Dive</strong>
          <span>具体场景 → 方案 → 追问 → 反馈</span>
        </div>
        <span className="badge">P1 · DeepSeek</span>
      </header>
      <details
        className="provider-settings"
        open={!data || data.settings.keyStatus !== "saved"}
      >
        <summary>
          模型与密钥设置 · {config.model} ·{" "}
          {data?.settings.keyStatus === "saved"
            ? "密钥已存入系统凭据存储"
            : data?.settings.keyStatus === "unavailable"
              ? "凭据存储暂不可访问"
              : "尚未保存密钥"}
        </summary>
        <div className="provider-fields">
          <label>
            Provider
            <select aria-label="AI Provider" value="deepseek" disabled>
              <option>deepseek</option>
            </select>
          </label>
          <label>
            模型名称
            <input
              aria-label="AI 模型名称"
              value={config.model}
              disabled={busy}
              onChange={(e) => setConfig({ ...config, model: e.target.value })}
            />
          </label>
          <label>
            请求超时（秒）
            <input
              aria-label="API 超时秒数"
              type="number"
              min={15}
              max={120}
              value={config.timeoutSeconds}
              disabled={busy}
              onChange={(e) =>
                setConfig({ ...config, timeoutSeconds: Number(e.target.value) })
              }
            />
          </label>
          <label>
            API Key
            <input
              aria-label="DeepSeek API Key"
              type="password"
              autoComplete="off"
              placeholder="留空保留已存 Key"
              value={apiKey}
              disabled={busy}
              onChange={(e) => setApiKey(e.target.value)}
            />
          </label>
          <button
            disabled={busy || !data}
            onClick={() =>
              perform(async () => {
                await invoke("save_llm_settings", {
                  config,
                  apiKey: apiKey.trim() || null,
                });
                setApiKey("");
              })
            }
          >
            保存设置与密钥
          </button>
          <button disabled={busy || !data} onClick={() => setConfirm("key")}>
            删除当前 Provider 密钥
          </button>
        </div>
        <p className="muted">
          Key 仅存入操作系统凭据存储，留空不删除。不同 Provider
          的凭据独立保留，未来接入 Claude 时不会覆盖 DeepSeek；当前仅启用
          DeepSeek。新模型设置用于新会话，已有会话继续使用创建时的模型。
        </p>
      </details>
      <div className="privacy-note">
        生成题目、提交回答和生成反馈会将所选主题、题目及必要会话内容发送给
        DeepSeek，可能产生 API 费用。缓存查看、复用和草稿保存不联网。AI
        反馈仅作训练辅助。
      </div>
      <div className="connection-status" aria-live="polite">
        {online ? "系统报告网络可用" : "系统报告离线"} · {connection}
        {busy ? " · 处理中，请等待请求超时或完成" : ""}
      </div>
      {error && (
        <div role="alert" className="error">
          {error}
        </div>
      )}
      {confirm && (
        <div className="confirm-panel" role="alert">
          <span>
            {confirm === "key"
              ? "确认从系统凭据存储删除 DeepSeek Key？历史及其他 Provider 的 Key 会保留。"
              : confirm === "history"
                ? "确认清空全部 Deep Dive 会话、反馈和缓存题目？LC 数据和密钥会保留。"
                : "确认删除当前面试及反馈？缓存题目会保留。"}
          </span>
          <button
            disabled={busy}
            onClick={() =>
              perform(async () => {
                if (confirm === "key") {
                  await invoke("delete_llm_key", { provider: "deepseek" });
                  setApiKey("");
                } else if (confirm === "history") {
                  await invoke("clear_interview_history");
                  setSession(null);
                  setDraft("");
                  setQuestionId("");
                } else if (session) {
                  await invoke("delete_interview_session", {
                    sessionId: session.id,
                  });
                  setSession(null);
                  setDraft("");
                }
                setConfirm(null);
              })
            }
          >
            确认删除
          </button>
          <button disabled={busy} onClick={() => setConfirm(null)}>
            取消
          </button>
        </div>
      )}
      <main className="interview-main">
        <aside>
          <h2>
            训练主题 <small>{data?.topics.length ?? 0} 个</small>
          </h2>
          <button
            disabled={busy || !data}
            onClick={() => {
              const topics = data!.topics;
              setTopicId(topics[Math.floor(Math.random() * topics.length)].id);
              setQuestionId("");
            }}
          >
            随机选题主题
          </button>
          {data?.topics.map((t) => (
            <button
              className={`problem ${t.id === topicId ? "active" : ""}`}
              key={t.id}
              disabled={busy}
              onClick={() => {
                setTopicId(t.id);
                setQuestionId("");
              }}
            >
              <small>{t.category}</small>
              <strong>{t.title}</strong>
            </button>
          ))}
          <h3>面试历史（本地）</h3>
          {data?.sessions.length === 0 && (
            <p className="muted">还没有面试会话。</p>
          )}
          {data?.sessions.map((s) => (
            <button
              className={`history-item ${s.id === session?.id ? "active" : ""}`}
              key={s.id}
              disabled={busy}
              onClick={() =>
                perform(async () =>
                  show(
                    await invoke<Session>("get_interview_session", {
                      sessionId: s.id,
                    }),
                  ),
                )
              }
            >
              <strong>
                {s.title.slice(0, 50)}
                {s.title.length > 50 ? "…" : ""}
              </strong>
              <small>
                {states[s.status]} · {new Date(s.createdAt).toLocaleString()}
              </small>
            </button>
          ))}
          <button
            disabled={busy || !data}
            onClick={() => setConfirm("history")}
          >
            清空面试历史与题目缓存
          </button>
        </aside>
        <section className="interview-workspace">
          <section className="topic-panel">
            {topic && (
              <>
                <h1>{topic.title}</h1>
                <p>{topic.focus.join(" / ")}</p>
                <p className="muted">
                  种子约束：{topic.scenarioConstraints.join("；")}
                </p>
                <button
                  className="primary"
                  disabled={busy || !data}
                  onClick={() =>
                    perform(async () => {
                      const q = await invoke<Question>(
                        "generate_interview_question",
                        { topicId },
                      );
                      setQuestionId(q.id);
                      show(
                        await invoke<Session>("start_interview", {
                          questionId: q.id,
                        }),
                      );
                    }, true)
                  }
                >
                  {busy ? "处理中…" : "生成新题并开始（联网）"}
                </button>
                <h3>已缓存问题 · {cached.length}</h3>
                {cached.length > 0 ? (
                  <>
                    <select
                      aria-label="选择已缓存问题"
                      value={preview?.id ?? ""}
                      disabled={busy}
                      onChange={(e) => setQuestionId(e.target.value)}
                    >
                      {cached.map((q) => (
                        <option value={q.id} key={q.id}>
                          {new Date(q.createdAt).toLocaleDateString()} ·{" "}
                          {q.question.slice(0, 90)}
                        </option>
                      ))}
                    </select>
                    {preview && (
                      <>
                        <h3>{preview.question}</h3>
                        <p>{preview.background}</p>
                        <ul>
                          {preview.constraints.map((c, i) => (
                            <li key={i}>{c}</li>
                          ))}
                        </ul>
                        <small className="muted">
                          生成于 {new Date(preview.createdAt).toLocaleString()}{" "}
                          · {preview.model}
                        </small>
                        <p>
                          <button
                            disabled={busy}
                            onClick={() =>
                              perform(async () =>
                                show(
                                  await invoke<Session>("start_interview", {
                                    questionId: preview.id,
                                  }),
                                ),
                              )
                            }
                          >
                            复用此题开始新面试（离线）
                          </button>
                        </p>
                      </>
                    )}
                  </>
                ) : (
                  <p className="muted">
                    首次生成后保存在本地。再次练习同一题不必重复生成。
                  </p>
                )}
              </>
            )}
          </section>
          <section className="conversation-panel">
            {!session ? (
              <p className="muted">
                生成新题、复用缓存题目，或从历史恢复面试。
              </p>
            ) : (
              <>
                <div className="session-heading">
                  <h2>当前面试 · {states[session.status]}</h2>
                  <small>
                    {session.provider} / {session.model}
                  </small>
                  <button disabled={busy} onClick={() => setConfirm("session")}>
                    删除此会话
                  </button>
                </div>
                <div className="question-context">
                  <h3>{session.question.question}</h3>
                  <p>{session.question.background}</p>
                  <ul>
                    {session.question.constraints.map((c, i) => (
                      <li key={i}>{c}</li>
                    ))}
                  </ul>
                </div>
                <div className="turns">
                  {session.turns.map((t, i) => (
                    <section className={`turn ${t.role}`} key={i}>
                      <strong>
                        {t.role === "candidate" ? "你的回答" : "面试官追问"}
                      </strong>
                      <p>{t.text}</p>
                    </section>
                  ))}
                </div>
                {session.lastError && (
                  <div role="alert" className="error">
                    {session.lastError} · 内容已保存在本地。
                  </div>
                )}
                {pending && (
                  <div className="pending-request">
                    <p>
                      {session.status === "awaiting_reply"
                        ? "回答已保存，等待本轮追问。重试不会重复追加回答。"
                        : "已保存结束请求，等待生成反馈。重试不会新建会话。"}
                    </p>
                    <button
                      className="primary"
                      disabled={busy}
                      onClick={() =>
                        perform(
                          async () =>
                            show(
                              await invoke<Session>("retry_interview", {
                                sessionId: session.id,
                              }),
                            ),
                          true,
                        )
                      }
                    >
                      重试待处理请求（联网）
                    </button>
                  </div>
                )}
                {session.status === "active" && (
                  <>
                    <label className="answer-label" htmlFor="interview-answer">
                      你的回答（自动保存，最多 6000 字符）
                    </label>
                    <textarea
                      id="interview-answer"
                      aria-label="技术深挖回答"
                      maxLength={6000}
                      rows={8}
                      value={draft}
                      disabled={busy}
                      onChange={(e) => save(e.target.value)}
                      placeholder="说明你的方案、依据、替代方案与失败场景…"
                    />
                    <div className="answer-status">
                      {saving
                        ? "正在保存草稿…"
                        : saveError.current
                          ? "草稿保存失败，请重试"
                          : "草稿已保存"}{" "}
                      · {draft.length}/6000 · 已回答{" "}
                      {
                        session.turns.filter((t) => t.role === "candidate")
                          .length
                      }
                      /20 轮
                    </div>
                    {saveError.current && (
                      <button disabled={busy} onClick={() => save(draft)}>
                        重试保存草稿
                      </button>
                    )}
                    <div className="actions">
                      <button
                        className="primary"
                        disabled={
                          busy ||
                          !draft.trim() ||
                          session.turns.filter((t) => t.role === "candidate")
                            .length >= 20
                        }
                        onClick={() =>
                          perform(
                            async () =>
                              show(
                                await invoke<Session>(
                                  "submit_interview_answer",
                                  { sessionId: session.id, answer: draft },
                                ),
                              ),
                            true,
                          )
                        }
                      >
                        提交回答并追问（联网）
                      </button>
                      <button
                        disabled={
                          busy ||
                          !!draft.trim() ||
                          !session.turns.some((t) => t.role === "candidate")
                        }
                        onClick={() =>
                          perform(
                            async () =>
                              show(
                                await invoke<Session>("finish_interview", {
                                  sessionId: session.id,
                                }),
                              ),
                            true,
                          )
                        }
                      >
                        结束并生成反馈（联网）
                      </button>
                    </div>
                    {draft.trim() && (
                      <p className="muted">
                        结束前请先提交回答，或自行清空尚未提交的草稿。
                      </p>
                    )}
                    {session.shouldEnd && (
                      <p className="muted">
                        面试官建议可以结束并查看反馈，你也可以继续回答。
                      </p>
                    )}
                  </>
                )}
                {session.feedback && (
                  <section className="feedback">
                    <h2>面试反馈 · 定性评估</h2>
                    <p>{session.feedback.summary}</p>
                    {session.feedback.dimensions.map((d) => (
                      <div className="feedback-dimension" key={d.name}>
                        <h3>
                          {dimensions[d.name] ?? d.name} ·{" "}
                          {ratings[d.rating] ?? d.rating}
                        </h3>
                        <p>
                          <strong>依据：</strong>
                          {d.evidence}
                        </p>
                        <p>
                          <strong>建议：</strong>
                          {d.improvement}
                        </p>
                      </div>
                    ))}
                    {[
                      ["做得好的方面", session.feedback.strengths],
                      ["遗漏与风险", session.feedback.gaps],
                      ["后续训练方向", session.feedback.nextPracticeTopics],
                    ].map(([name, items]) => (
                      <section key={name as string}>
                        <h3>{name}</h3>
                        <ul>
                          {(items as string[]).map((s, i) => (
                            <li key={i}>{s}</li>
                          ))}
                        </ul>
                      </section>
                    ))}
                  </section>
                )}
              </>
            )}
          </section>
        </section>
      </main>
    </div>
  );
}
