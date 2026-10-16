import { useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@astryxdesign/core/Button";
import { loadCurrent } from "./navigation";
import { HistoryDrawer } from "./HistoryDrawer";
import { TextInput } from "@astryxdesign/core/TextInput";
import { TextArea } from "@astryxdesign/core/TextArea";
import { Selector } from "@astryxdesign/core/Selector";
import { Badge } from "@astryxdesign/core/Badge";
import { Banner } from "@astryxdesign/core/Banner";
import { Collapsible } from "@astryxdesign/core/Collapsible";
import { AlertDialog } from "@astryxdesign/core/AlertDialog";
import { lookup, useI18n } from "../../i18n";
import { localizeError } from "../../errors";
import { LiveResponse } from "./LiveResponse";
import "./interview.css";
import { SessionTools } from "./SessionTools";
import { VoiceInput } from "../voice/VoiceInput";
import { Markdown } from "../../Markdown";
import { EmptyState, FeedbackLists } from "../../EmptyState";
type Config = { provider: string; model: string; timeoutSeconds: number };
type Settings = { config: Config; keyStatus: string; credentialSource: string };
type Topic = {
  id: string;
  category: string;
  title: string;
  focus: string[];
  scenarioConstraints: string[];
  questionForm?: "foundation" | "scenario" | "troubleshooting";
  priority?: "core" | "advanced";
  sources?: string[];
  en?: { title: string; focus: string[]; scenarioConstraints: string[] };
};
const hasChinese = (text: string) => /[\u4e00-\u9fff]/.test(text);
// Topics come with English text; English mode shows it and the AI answers in English.
const localTopic = (x: Topic, lang: string): Topic =>
  lang === "en" && x.en ? { ...x, ...x.en } : x;
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
type Reference = { turnIndex: number; answer: string; keyPoints: string[] };
type Session = {
  id: string;
  question: Question;
  provider: string;
  model: string;
  status: string;
  draft: string;
  lastError: string | null;
  shouldEnd: boolean;
  turns: {
    role: string;
    text: string;
    timestamp: string;
    aiAssisted: boolean;
  }[];
  feedback: Feedback | null;
  references: Reference[];
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
type Connection =
  | { kind: "unverified" }
  | { kind: "requesting"; provider: string }
  | { kind: "ok" }
  | { kind: "failed" };
const ratingBadge: Record<
  string,
  "success" | "warning" | "error" | "info" | "neutral"
> = {
  good: "success",
  partial: "warning",
  missing: "neutral",
  incorrect: "error",
  uncertain: "info",
};
const stateBadge: Record<string, "info" | "warning" | "success"> = {
  active: "info",
  awaiting_reply: "warning",
  feedback_pending: "warning",
  finished: "success",
};
export default function InterviewApp({
  settingsVersion = 0,
  visible = true,
  focus,
}: {
  settingsVersion?: number;
  visible?: boolean;
  focus?: { id: string; session: boolean; n: number };
}) {
  const { t, lang } = useI18n();
  const [data, setData] = useState<Overview | null>(null),
    [topicId, setTopicId] = useState("mysql-index-structure"),
    [questionId, setQuestionId] = useState("");
  const [session, setSession] = useState<Session | null>(null),
    [draft, setDraft] = useState("");
  const [topicSearch, setTopicSearch] = useState(""),
    [topicCategory, setTopicCategory] = useState("");
  const [topicForm, setTopicForm] = useState("");
  const [topicPriority, setTopicPriority] = useState("core");
  const [config, setConfig] = useState<Config>({
    provider: "claude",
    model: "claude-haiku-5-5",
    timeoutSeconds: 60,
  });
  const [voiceBusy, setVoiceBusy] = useState(false);
  const [requestBusy, setBusy] = useState(false),
    [saving, setSaving] = useState(false),
    [error, setError] = useState(""),
    [connection, setConnection] = useState<Connection>({
      kind: "unverified",
    }),
    [live, setLive] = useState("");
  const busy = requestBusy || voiceBusy;
  const [online, setOnline] = useState(navigator.onLine),
    [confirm, setConfirm] = useState<"history" | "session" | null>(null);
  const queue = useRef(Promise.resolve()),
    saveError = useRef(""),
    revision = useRef(0);
  const locale = lang === "zh" ? "zh-CN" : "en";
  const categoryLabel = (category: string) =>
    lookup(t, "topicCategory_", category);
  const activeTopicFilters = [
    topicSearch.trim(),
    topicCategory,
    topicForm,
    topicPriority,
  ].filter(Boolean).length;
  const formOrder = { foundation: 0, scenario: 1, troubleshooting: 2 };
  const visibleTopics = (data?.topics ?? [])
    .filter((topic) => {
      const query = topicSearch.trim().toLocaleLowerCase();
      const localized = localTopic(topic, lang);
      return (
        (!topicCategory || topic.category === topicCategory) &&
        (!topicForm || (topic.questionForm ?? "scenario") === topicForm) &&
        (!topicPriority || (topic.priority ?? "core") === topicPriority) &&
        (!query ||
          [
            topic.id,
            topic.title,
            topic.en?.title ?? "",
            categoryLabel(topic.category),
            ...localized.focus,
            ...localized.scenarioConstraints,
          ]
            .join(" ")
            .toLocaleLowerCase()
            .includes(query))
      );
    })
    .sort(
      (a, b) =>
        Number((a.priority ?? "core") === "advanced") -
          Number((b.priority ?? "core") === "advanced") ||
        formOrder[a.questionForm ?? "scenario"] -
          formOrder[b.questionForm ?? "scenario"],
    );
  const [historyOpen, setHistoryOpen] = useState(false);
  const navigationVersion = useRef(0);
  const requestOwner = useRef<number | null>(null);
  useEffect(() => {
    if (isTauri()) void overview().catch((e) => setError(String(e)));
  }, [settingsVersion]);
  // Explicit review navigation wins over initial restoration and older requests.
  useEffect(() => {
    if (!focus || !isTauri()) return;
    const version = ++navigationVersion.current;
    setLive("");
    let disposed = false;
    const current = () => !disposed && navigationVersion.current === version;
    void (async () => {
      await queue.current;
      if (!current()) return;
      if (saveError.current) throw new Error(saveError.current);
      if (focus.session) {
        await loadCurrent(
          () =>
            invoke<Session>("get_interview_session", { sessionId: focus.id }),
          show,
          current,
        );
      } else {
        setTopicSearch("");
        setTopicCategory("");
        setTopicForm("");
        setTopicPriority("");
        setTopicId(focus.id);
        setQuestionId("");
        setSession(null);
        setDraft("");
      }
    })().catch((e) => {
      if (current()) setError(String(e));
    });
    return () => {
      disposed = true;
    };
  }, [focus?.n]);
  function show(s: Session) {
    navigationVersion.current += 1;
    setHistoryOpen(false);
    setSession(s);
    setDraft(s.draft);
    setTopicId(s.question.seedId);
    setQuestionId(s.question.id);
    if (!visibleTopics.some((topic) => topic.id === s.question.seedId)) {
      setTopicSearch("");
      setTopicCategory("");
      setTopicForm("");
      setTopicPriority("");
    }
  }
  async function overview() {
    const result = await invoke<Overview>("interview_overview");
    setData(result);
    setConfig(result.settings.config);
    return result;
  }
  useEffect(() => {
    if (!isTauri()) return;
    // The backend streams the model's partial JSON while a request runs.
    const stop = listen<string>("llm-stream", (e) => {
      if (requestOwner.current === navigationVersion.current)
        setLive(e.payload);
    });
    return () => {
      void stop.then((unlisten) => unlisten());
    };
  }, []);
  useEffect(() => {
    let disposed = false;
    const version = navigationVersion.current;
    const current = () =>
      !disposed && !focus && navigationVersion.current === version;
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
            await loadCurrent(
              () =>
                invoke<Session>("get_interview_session", {
                  sessionId: previous.id,
                }),
              show,
              current,
            );
        })
        .catch((e) => setError(String(e)));
    return () => {
      disposed = true;
      navigationVersion.current += 1;
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
        setError(t("draftSaveFailed", { e: String(e) }));
      })
      .then(() => {
        if (revision.current === version) setSaving(false);
      });
  }
  async function flush() {
    await queue.current;
    if (saveError.current) throw new Error(saveError.current);
  }
  async function perform(
    task: (current: () => boolean) => Promise<void>,
    network = false,
  ) {
    if (voiceBusy || requestOwner.current !== null) return;
    // Every explicit action supersedes a still-loading default restoration.
    const version = ++navigationVersion.current;
    requestOwner.current = version;
    const current = () => navigationVersion.current === version;
    setBusy(true);
    setError("");
    setLive("");
    if (network)
      setConnection({
        kind: "requesting",
        provider: session?.provider ?? config.provider,
      });
    try {
      await flush();
      if (!current()) return;
      await task(current);
      await overview();
      if (network) setConnection({ kind: "ok" });
    } catch (e) {
      if (!current()) return;
      setError(String(e));
      if (network) setConnection({ kind: "failed" });
      if (session && !saveError.current) {
        try {
          await loadCurrent(
            () =>
              invoke<Session>("get_interview_session", {
                sessionId: session.id,
              }),
            show,
            current,
          );
        } catch {
          /* retain current local text on refresh failure */
        }
      }
    } finally {
      requestOwner.current = null;
      setBusy(false);
      setLive("");
    }
  }
  function chooseTopic(id: string) {
    if (busy || requestOwner.current !== null || id === topicId) return;
    void perform(async (current) => {
      if (!current()) return;
      setTopicId(id);
      setQuestionId("");
      setSession(null);
      setDraft("");
    });
  }
  const found = data?.topics.find((x) => x.id === topicId),
    topic = found && localTopic(found, lang),
    // Cached questions are reused only in the language they were written in.
    cached =
      data?.questions.filter(
        (q) =>
          q.seedId === topicId && hasChinese(q.question) === (lang === "zh"),
      ) ?? [];
  const preview = cached.find((q) => q.id === questionId) ?? cached[0];
  const pending =
    session?.status === "awaiting_reply" ||
    session?.status === "feedback_pending";
  const answered =
    session?.turns.filter((turn) => turn.role === "candidate").length ?? 0;
  const stateLabel = (s: string) => lookup(t, "state_", s);
  // A reference shown for question k (0 = original question, i + 1 = follow-up
  // turns[i]) without being submitted as an answer.
  function shownReference(k: number) {
    if (!session || session.turns[k]?.aiAssisted) return null;
    const r = session.references.find((x) => x.turnIndex === k);
    return (
      r && (
        <div className="shown-reference">
          <Collapsible trigger={<strong>{t("refShown")}</strong>} defaultIsOpen>
            <Markdown text={r.answer} />
            <ul>
              {r.keyPoints.map((point, n) => (
                <li key={n}>
                  <Markdown text={point} />
                </li>
              ))}
            </ul>
            {session.status === "active" && k === session.turns.length && (
              <div className="actions">
                <Button
                  label={t("copyToAnswer")}
                  size="sm"
                  isDisabled={busy}
                  onClick={() => save(r.answer)}
                />
                <Button
                  label={t("submitRef")}
                  size="sm"
                  isDisabled={busy || !!draft.trim()}
                  tooltip={draft.trim() ? t("clearDraftFirst") : undefined}
                  onClick={() =>
                    perform(
                      async (current) =>
                        await loadCurrent(
                          async () =>
                            await invoke<Session>(
                              "submit_interview_reference",
                              {
                                sessionId: session.id,
                                turnIndex: k,
                                lang,
                              },
                            ),
                          show,
                          current,
                        ),
                      true,
                    )
                  }
                />
              </div>
            )}
          </Collapsible>
        </div>
      )
    );
  }
  const connectionText =
    connection.kind === "requesting"
      ? t("requesting", { p: connection.provider })
      : connection.kind === "ok"
        ? t("lastOk")
        : connection.kind === "failed"
          ? t("lastFailed")
          : t("apiUnverified");
  const referenceReady = !!session?.references.some(
    (r) => r.turnIndex === session.turns.length,
  );
  return (
    <div className="interview-app">
      <section className="dd-hero">
        <div>
          <h1>{t("moduleDeepDive")}</h1>
          <p className="muted">{t("ddTagline")}</p>
        </div>
        <div className="dd-status" aria-live="polite">
          <Badge
            variant={online ? "success" : "error"}
            label={online ? t("online") : t("offline")}
          />
          <Badge
            variant={
              connection.kind === "failed"
                ? "error"
                : connection.kind === "ok"
                  ? "success"
                  : "neutral"
            }
            label={
              voiceBusy
                ? t("voiceInProgress")
                : requestBusy
                  ? t("busyNote")
                  : connectionText
            }
          />
          <Badge
            variant="purple"
            label={`${config.provider} · ${config.model}`}
          />
        </div>
      </section>
      <p className="privacy-note">{t("privacyNote")}</p>
      {requestBusy && live && <LiveResponse text={live} />}
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
      <AlertDialog
        isOpen={confirm !== null}
        onOpenChange={(open) => !open && !busy && setConfirm(null)}
        title={
          confirm === "history"
            ? t("confirmHistoryTitle")
            : t("confirmSessionTitle")
        }
        description={
          confirm === "history" ? t("confirmHistory") : t("confirmSession")
        }
        actionLabel={t("confirmDelete")}
        cancelLabel={t("cancel")}
        isActionLoading={busy}
        onAction={async () => {
          // perform() reports failures in the error banner, so the dialog closes either way.
          await perform(async (current) => {
            if (confirm === "history") {
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
          });
          setConfirm(null);
        }}
      />
      <main className="interview-main">
        <aside>
          <Button
            label={t("openHistory", { n: data?.sessions.length ?? 0 })}
            width="100%"
            onClick={() => setHistoryOpen(true)}
          />
          <div className="list-head">
            <h2>{t("topics")}</h2>
            <Badge label={t("topicCount", { n: data?.topics.length ?? 0 })} />
          </div>
          <Collapsible
            trigger={
              <strong>{t("topicFilters", { n: activeTopicFilters })}</strong>
            }
            defaultIsOpen={false}
          >
            <div className="topic-filter-fields">
              <TextInput
                label={t("topicSearch")}
                placeholder={t("topicSearchHint")}
                size="sm"
                width="100%"
                value={topicSearch}
                onChange={setTopicSearch}
              />
              <Selector
                label={t("topicCategory")}
                size="sm"
                width="100%"
                value={topicCategory}
                onChange={setTopicCategory}
                options={[
                  { value: "", label: t("topicAllCategories") },
                  ...Array.from(
                    new Set(data?.topics.map((topic) => topic.category) ?? []),
                  ).map((category) => ({
                    value: category,
                    label: categoryLabel(category),
                  })),
                ]}
              />
              <Selector
                label={t("topicForm")}
                size="sm"
                width="100%"
                value={topicForm}
                onChange={setTopicForm}
                options={[
                  { value: "", label: t("topicAllForms") },
                  ...(
                    ["foundation", "scenario", "troubleshooting"] as const
                  ).map((form) => ({
                    value: form,
                    label: t(`topicForm_${form}`),
                  })),
                ]}
              />
              <Selector
                label={t("topicPriority")}
                size="sm"
                width="100%"
                value={topicPriority}
                onChange={setTopicPriority}
                options={[
                  { value: "", label: t("topicAllPriorities") },
                  { value: "core", label: t("topicPriority_core") },
                  { value: "advanced", label: t("topicPriority_advanced") },
                ]}
              />
              {activeTopicFilters > 0 && (
                <Button
                  label={t("topicClearFilters")}
                  size="sm"
                  variant="ghost"
                  onClick={() => {
                    setTopicSearch("");
                    setTopicCategory("");
                    setTopicForm("");
                    setTopicPriority("");
                  }}
                />
              )}
            </div>
          </Collapsible>
          <p className="muted topic-match-count" role="status">
            {t("topicMatches", {
              n: visibleTopics.length,
              total: data?.topics.length ?? 0,
            })}
          </p>
          <Button
            label={t("randomTopic")}
            variant="secondary"
            className="tint-btn tint-cyan"
            size="sm"
            width="100%"
            isDisabled={busy || !visibleTopics.length}
            onClick={() => {
              chooseTopic(
                visibleTopics[Math.floor(Math.random() * visibleTopics.length)]
                  .id,
              );
            }}
          />
          <div
            className="topic-list"
            role="region"
            aria-label={t("topics")}
            tabIndex={0}
          >
            {!visibleTopics.length && (
              <p className="muted">{t("topicNoMatches")}</p>
            )}
            {visibleTopics.map((x) => (
              <button
                className={`problem ${x.id === topicId ? "active" : ""}`}
                key={x.id}
                disabled={busy}
                aria-pressed={x.id === topicId}
                onClick={() => chooseTopic(x.id)}
              >
                <small>
                  {categoryLabel(x.category)} ·{" "}
                  {t(`topicForm_${x.questionForm ?? "scenario"}`)}
                </small>
                <strong>{localTopic(x, lang).title}</strong>
              </button>
            ))}
          </div>
        </aside>
        <HistoryDrawer
          open={historyOpen}
          onOpenChange={setHistoryOpen}
          title={t("history")}
        >
          {error && (
            <Banner status="error" title={localizeError(error, lang)} />
          )}
          {data?.sessions.length === 0 && (
            <EmptyState icon="🗂" text={t("noSessions")} />
          )}
          {data?.sessions.map((s) => (
            <button
              className={`problem ${s.id === session?.id ? "active" : ""}`}
              key={s.id}
              aria-current={s.id === session?.id ? "true" : undefined}
              disabled={busy}
              onClick={() =>
                perform(async (current) =>
                  loadCurrent(
                    async () =>
                      await invoke<Session>("get_interview_session", {
                        sessionId: s.id,
                      }),
                    show,
                    current,
                  ),
                )
              }
            >
              <strong>
                {s.title.slice(0, 50)}
                {s.title.length > 50 ? "…" : ""}
              </strong>
              <small>
                {stateLabel(s.status)} ·{" "}
                {new Date(s.createdAt).toLocaleString(locale)}
              </small>
            </button>
          ))}
          <Button
            label={t("clearHistory")}
            variant="destructive"
            size="sm"
            width="100%"
            isDisabled={busy || !data}
            onClick={() => {
              setHistoryOpen(false);
              setConfirm("history");
            }}
          />
        </HistoryDrawer>
        <section className="interview-workspace">
          <section className="topic-panel card">
            {topic && (
              <>
                <h2>{topic.title}</h2>
                <p className="muted">
                  {t(`topicForm_${topic.questionForm ?? "scenario"}`)} ·{" "}
                  {t(`topicPriority_${topic.priority ?? "core"}`)}
                </p>
                <div className="actions">
                  <Button
                    label={t("generateStart")}
                    variant="primary"
                    isDisabled={busy || !data}
                    isLoading={requestBusy}
                    onClick={() =>
                      perform(async (current) => {
                        const q = await invoke<Question>(
                          "generate_interview_question",
                          { topicId, lang },
                        );
                        if (!current()) return;
                        setQuestionId(q.id);
                        await loadCurrent(
                          async () =>
                            await invoke<Session>("start_interview", {
                              questionId: q.id,
                            }),
                          show,
                          current,
                        );
                      }, true)
                    }
                  />
                </div>
                <h3>{t("cachedQuestions", { n: cached.length })}</h3>
                {cached.length > 0 ? (
                  <>
                    <Selector
                      label={t("pickCached")}
                      isLabelHidden
                      width="100%"
                      value={preview?.id ?? ""}
                      isDisabled={busy}
                      onChange={setQuestionId}
                      options={cached.map((q) => ({
                        value: q.id,
                        label: `${new Date(q.createdAt).toLocaleDateString(locale)} · ${q.question.slice(0, 90)}`,
                      }))}
                    />
                    {preview && (
                      <div className="question-preview">
                        <div className="question-title">
                          <Markdown text={preview.question} />
                        </div>
                        <Markdown text={preview.background} />
                        <ul>
                          {preview.constraints.map((c, i) => (
                            <li key={i}>
                              <Markdown text={c} />
                            </li>
                          ))}
                        </ul>
                        <small className="muted">
                          {t("generatedAt", {
                            d: new Date(preview.createdAt).toLocaleString(
                              locale,
                            ),
                            m: preview.model,
                          })}
                        </small>
                        <div className="actions">
                          <Button
                            label={t("reuseStart")}
                            isDisabled={busy}
                            onClick={() =>
                              perform(
                                async (current) =>
                                  await loadCurrent(
                                    async () =>
                                      await invoke<Session>("start_interview", {
                                        questionId: preview.id,
                                      }),
                                    show,
                                    current,
                                  ),
                              )
                            }
                          />
                        </div>
                      </div>
                    )}
                  </>
                ) : (
                  <p className="muted">{t("cacheHint")}</p>
                )}
              </>
            )}
          </section>
          <section className="conversation-panel card">
            {!session ? (
              <EmptyState icon="💬" text={t("noSession")} />
            ) : (
              <>
                <div className="session-heading">
                  <h2>{t("interviewTitle")}</h2>
                  <Badge
                    variant={stateBadge[session.status] ?? "neutral"}
                    label={stateLabel(session.status)}
                  />
                  <small className="muted">
                    {session.provider} / {session.model}
                  </small>
                  <span className="spacer" />
                  <Button
                    label={t("deleteSession")}
                    variant="destructive"
                    size="sm"
                    isDisabled={busy}
                    onClick={() => setConfirm("session")}
                  />
                </div>
                <SessionTools
                  key={session.id}
                  module="p1"
                  sessionId={session.id}
                  status={session.status}
                  busy={busy || saving}
                  turns={answered}
                />
                <div className="question-context">
                  <div className="question-title">
                    <Markdown text={session.question.question} />
                  </div>
                  <Markdown text={session.question.background} />
                  <ul>
                    {session.question.constraints.map((c, i) => (
                      <li key={i}>
                        <Markdown text={c} />
                      </li>
                    ))}
                  </ul>
                  {shownReference(0)}
                </div>
                <div className="turns">
                  {session.turns.map((turn, i) => (
                    <section className={`turn ${turn.role}`} key={i}>
                      <strong>
                        {turn.role === "candidate"
                          ? turn.aiAssisted
                            ? t("roleAiAnswer")
                            : t("roleYou")
                          : t("roleInterviewer")}
                      </strong>
                      <Markdown text={turn.text} />
                      {turn.aiAssisted && (
                        <details className="reference-content">
                          <summary>{t("refKeyPoints")}</summary>
                          <ul>
                            {session.references
                              .find((r) => r.turnIndex === i)
                              ?.keyPoints.map((point, n) => (
                                <li key={n}>
                                  <Markdown text={point} />
                                </li>
                              ))}
                          </ul>
                        </details>
                      )}
                      {turn.role === "interviewer" && shownReference(i + 1)}
                    </section>
                  ))}
                </div>
                {session.lastError && (
                  <Banner
                    status="error"
                    title={t("savedLocally", {
                      e: localizeError(session.lastError, lang),
                    })}
                  />
                )}
                {pending && (
                  <div className="pending-request">
                    <p>
                      {session.status === "awaiting_reply"
                        ? t("pendingReply")
                        : t("pendingFeedback")}
                    </p>
                    <Button
                      label={t("retryPending")}
                      variant="primary"
                      isDisabled={busy}
                      isLoading={requestBusy}
                      onClick={() =>
                        perform(
                          async (current) =>
                            await loadCurrent(
                              async () =>
                                await invoke<Session>("retry_interview", {
                                  sessionId: session.id,
                                  lang,
                                }),
                              show,
                              current,
                            ),
                          true,
                        )
                      }
                    />
                  </div>
                )}
                {session.status === "active" && (
                  <>
                    <TextArea
                      label={t("yourAnswer")}
                      maxLength={6000}
                      rows={8}
                      value={draft}
                      isDisabled={busy}
                      onChange={(v) => save(v)}
                      placeholder={t("answerPlaceholder")}
                      width="100%"
                    />
                    <VoiceInput
                      key={session.id}
                      module="p1"
                      sessionId={session.id}
                      visible={visible}
                      disabled={requestBusy}
                      draft={draft}
                      onDraft={save}
                      onBusyChange={setVoiceBusy}
                    />
                    <div className="answer-status muted">
                      {saving
                        ? t("saving")
                        : saveError.current
                          ? t("draftFailed")
                          : t("saved")}{" "}
                      · {t("rounds", { n: answered })}
                    </div>
                    {saveError.current && (
                      <Button
                        label={t("retrySave")}
                        size="sm"
                        isDisabled={busy}
                        onClick={() => save(draft)}
                      />
                    )}
                    <div className="actions">
                      <Button
                        label={t("submitAnswer")}
                        variant="primary"
                        isDisabled={busy || !draft.trim() || answered >= 20}
                        isLoading={requestBusy}
                        onClick={() =>
                          perform(
                            async (current) =>
                              await loadCurrent(
                                async () =>
                                  await invoke<Session>(
                                    "submit_interview_answer",
                                    {
                                      sessionId: session.id,
                                      answer: draft,
                                      lang,
                                    },
                                  ),
                                show,
                                current,
                              ),
                            true,
                          )
                        }
                      />
                      <Button
                        label={
                          referenceReady ? t("refReady") : t("aiReference")
                        }
                        variant="secondary"
                        className="tint-btn tint-purple"
                        isDisabled={busy || referenceReady}
                        onClick={() =>
                          perform(
                            async (current) =>
                              await loadCurrent(
                                async () =>
                                  await invoke<Session>(
                                    "show_interview_reference",
                                    {
                                      sessionId: session.id,
                                      turnIndex: session.turns.length,
                                      lang,
                                    },
                                  ),
                                show,
                                current,
                              ),
                            true,
                          )
                        }
                      />
                      <Button
                        label={t("finishFeedback")}
                        variant="secondary"
                        className="tint-btn tint-orange"
                        isDisabled={busy || !!draft.trim() || answered === 0}
                        onClick={() =>
                          perform(
                            async (current) =>
                              await loadCurrent(
                                async () =>
                                  await invoke<Session>("finish_interview", {
                                    sessionId: session.id,
                                    lang,
                                  }),
                                show,
                                current,
                              ),
                            true,
                          )
                        }
                      />
                    </div>
                    <p className="muted">{t("refHint")}</p>
                    {draft.trim() && (
                      <p className="muted">{t("finishNeedsClear")}</p>
                    )}
                    {session.shouldEnd && (
                      <p className="muted">{t("suggestEnd")}</p>
                    )}
                  </>
                )}
                {session.feedback && (
                  <section className="feedback">
                    <h2>{t("feedbackTitle")}</h2>
                    <p role="status" className="muted">
                      {t("reviewAdded")}
                    </p>
                    <Markdown text={session.feedback.summary} />
                    <div className="feedback-grid">
                      {session.feedback.dimensions.map((d) => (
                        <div className="feedback-dimension" key={d.name}>
                          <div className="dimension-head">
                            <strong>{lookup(t, "dim_", d.name)}</strong>
                            <Badge
                              variant={ratingBadge[d.rating] ?? "neutral"}
                              label={lookup(t, "rating_", d.rating)}
                            />
                          </div>
                          <div>
                            <strong>{t("evidence")}</strong>
                            <Markdown text={d.evidence} />
                          </div>
                          <div>
                            <strong>{t("advice")}</strong>
                            <Markdown text={d.improvement} />
                          </div>
                        </div>
                      ))}
                    </div>
                    <FeedbackLists
                      strengths={session.feedback.strengths}
                      gaps={session.feedback.gaps}
                      next={session.feedback.nextPracticeTopics}
                    />
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
