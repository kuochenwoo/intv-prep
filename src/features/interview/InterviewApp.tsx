import { useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@astryxdesign/core/Button";
import { TextInput } from "@astryxdesign/core/TextInput";
import { TextArea } from "@astryxdesign/core/TextArea";
import { NumberInput } from "@astryxdesign/core/NumberInput";
import { Selector } from "@astryxdesign/core/Selector";
import { Badge } from "@astryxdesign/core/Badge";
import { Banner } from "@astryxdesign/core/Banner";
import { Collapsible } from "@astryxdesign/core/Collapsible";
import { AlertDialog } from "@astryxdesign/core/AlertDialog";
import { lookup, useI18n } from "../../i18n";
import { localizeError } from "../../errors";
import { LiveResponse } from "./LiveResponse";
import "./interview.css";
type Config = { provider: string; model: string; timeoutSeconds: number };
type Settings = { config: Config; keyStatus: string; credentialSource: string };
type Topic = {
  id: string;
  category: string;
  title: string;
  focus: string[];
  scenarioConstraints: string[];
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
export default function InterviewApp() {
  const { t, lang } = useI18n();
  const [data, setData] = useState<Overview | null>(null),
    [topicId, setTopicId] = useState("cache-selection"),
    [questionId, setQuestionId] = useState("");
  const [session, setSession] = useState<Session | null>(null),
    [draft, setDraft] = useState("");
  const [topicSearch, setTopicSearch] = useState(""),
    [topicCategory, setTopicCategory] = useState("");
  const [config, setConfig] = useState<Config>({
      provider: "claude",
      model: "claude-haiku-5-5",
      timeoutSeconds: 60,
    }),
    [apiKey, setApiKey] = useState("");
  const [busy, setBusy] = useState(false),
    [saving, setSaving] = useState(false),
    [error, setError] = useState(""),
    [connection, setConnection] = useState<Connection>({
      kind: "unverified",
    }),
    [live, setLive] = useState("");
  const [online, setOnline] = useState(navigator.onLine),
    [confirm, setConfirm] = useState<"key" | "history" | "session" | null>(
      null,
    );
  const queue = useRef(Promise.resolve()),
    saveError = useRef(""),
    revision = useRef(0);
  const locale = lang === "zh" ? "zh-CN" : "en";
  const categoryLabel = (category: string) =>
    lookup(t, "topicCategory_", category);
  const visibleTopics = (data?.topics ?? []).filter((topic) => {
    const query = topicSearch.trim().toLocaleLowerCase();
    const localized = localTopic(topic, lang);
    return (
      (!topicCategory || topic.category === topicCategory) &&
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
  });
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
    if (!isTauri()) return;
    // The backend streams the model's partial JSON while a request runs.
    const stop = listen<string>("llm-stream", (e) => setLive(e.payload));
    return () => {
      void stop.then((unlisten) => unlisten());
    };
  }, []);
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
  async function perform(task: () => Promise<void>, network = false) {
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
      await task();
      await overview();
      if (network) setConnection({ kind: "ok" });
    } catch (e) {
      setError(String(e));
      if (network) setConnection({ kind: "failed" });
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
      setLive("");
    }
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
            <p>{r.answer}</p>
            <ul>
              {r.keyPoints.map((point, n) => (
                <li key={n}>{point}</li>
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
                      async () =>
                        show(
                          await invoke<Session>("submit_interview_reference", {
                            sessionId: session.id,
                            turnIndex: k,
                            lang,
                          }),
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
  const keyStatus =
    data && config.provider !== data.settings.config.provider
      ? t("providerUnsaved")
      : data?.settings.keyStatus === "saved"
        ? t("keySaved")
        : data?.settings.keyStatus === "unavailable"
          ? t("keyUnavailable")
          : t("keyMissing");
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
            label={busy ? t("busyNote") : connectionText}
          />
          <Badge
            variant="purple"
            label={`${config.provider} · ${config.model}`}
          />
        </div>
      </section>
      <div className="provider-settings">
        <Collapsible
          trigger={
            <span>
              <strong>{t("settingsTitle")}</strong>{" "}
              <span className="muted">· {keyStatus}</span>
            </span>
          }
          defaultIsOpen={!data || data.settings.keyStatus !== "saved"}
        >
          <div className="provider-fields">
            <Selector
              label="Provider"
              size="sm"
              width={160}
              value={config.provider}
              isDisabled={busy}
              onChange={(v) => {
                setApiKey("");
                setConfig({
                  ...config,
                  provider: v,
                  model: v === "claude" ? "claude-haiku-5-5" : "deepseek-flash",
                });
              }}
              options={[
                { value: "claude", label: "Claude" },
                { value: "deepseek", label: "DeepSeek" },
              ]}
            />
            <TextInput
              label={t("modelName")}
              size="sm"
              width={220}
              value={config.model}
              isDisabled={busy}
              onChange={(v) => setConfig({ ...config, model: v })}
            />
            <NumberInput
              label={t("timeoutSeconds")}
              size="sm"
              min={15}
              max={120}
              value={config.timeoutSeconds}
              isDisabled={busy}
              onChange={(v) => setConfig({ ...config, timeoutSeconds: v })}
            />
            <TextInput
              label={`${config.provider} API Key`}
              type="password"
              autoComplete="off"
              size="sm"
              width={240}
              placeholder={t("apiKeyPlaceholder")}
              value={apiKey}
              isDisabled={busy}
              onChange={setApiKey}
            />
            <Button
              label={t("saveSettings")}
              variant="primary"
              size="sm"
              isDisabled={busy || !data}
              onClick={() =>
                perform(async () => {
                  await invoke("save_llm_settings", {
                    config,
                    apiKey: apiKey.trim() || null,
                  });
                  setApiKey("");
                })
              }
            />
            <Button
              label={t("deleteKey")}
              variant="ghost"
              size="sm"
              isDisabled={busy || !data}
              onClick={() => setConfirm("key")}
            />
          </div>
          <p className="muted">{t("settingsNote")}</p>
        </Collapsible>
      </div>
      <p className="privacy-note">{t("privacyNote")}</p>
      {busy && live && <LiveResponse text={live} />}
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
          confirm === "key"
            ? t("confirmKeyTitle", { p: config.provider })
            : confirm === "history"
              ? t("confirmHistoryTitle")
              : t("confirmSessionTitle")
        }
        description={
          confirm === "key"
            ? t("confirmKey")
            : confirm === "history"
              ? t("confirmHistory")
              : t("confirmSession")
        }
        actionLabel={t("confirmDelete")}
        cancelLabel={t("cancel")}
        isActionLoading={busy}
        onAction={async () => {
          // perform() reports failures in the error banner, so the dialog closes either way.
          await perform(async () => {
            if (confirm === "key") {
              await invoke("delete_llm_key", {
                provider: config.provider,
              });
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
          });
          setConfirm(null);
        }}
      />
      <main className="interview-main">
        <aside>
          <div className="list-head">
            <h2>{t("topics")}</h2>
            <Badge label={t("topicCount", { n: data?.topics.length ?? 0 })} />
          </div>
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
          <p className="muted topic-match-count" role="status">
            {t("topicMatches", {
              n: visibleTopics.length,
              total: data?.topics.length ?? 0,
            })}
          </p>
          {(topicSearch || topicCategory) && (
            <Button
              label={t("topicClearFilters")}
              size="sm"
              variant="ghost"
              onClick={() => {
                setTopicSearch("");
                setTopicCategory("");
              }}
            />
          )}
          <Button
            label={t("randomTopic")}
            size="sm"
            width="100%"
            isDisabled={busy || !visibleTopics.length}
            onClick={() => {
              setTopicId(
                visibleTopics[Math.floor(Math.random() * visibleTopics.length)]
                  .id,
              );
              setQuestionId("");
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
                onClick={() => {
                  setTopicId(x.id);
                  setQuestionId("");
                }}
              >
                <small>{categoryLabel(x.category)}</small>
                <strong>{localTopic(x, lang).title}</strong>
              </button>
            ))}
          </div>
          <h3>{t("history")}</h3>
          {data?.sessions.length === 0 && (
            <p className="muted">{t("noSessions")}</p>
          )}
          {data?.sessions.map((s) => (
            <button
              className={`problem ${s.id === session?.id ? "active" : ""}`}
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
                {stateLabel(s.status)} ·{" "}
                {new Date(s.createdAt).toLocaleString(locale)}
              </small>
            </button>
          ))}
          <Button
            label={t("clearHistory")}
            variant="ghost"
            size="sm"
            width="100%"
            isDisabled={busy || !data}
            onClick={() => setConfirm("history")}
          />
        </aside>
        <section className="interview-workspace">
          <section className="topic-panel card">
            {topic && (
              <>
                <h2>{topic.title}</h2>
                <p>{topic.focus.join(" / ")}</p>
                <p className="muted">
                  {t("seedConstraints", {
                    c: topic.scenarioConstraints.join(
                      lang === "zh" ? "；" : "; ",
                    ),
                  })}
                </p>
                <div className="actions">
                  <Button
                    label={t("generateStart")}
                    variant="primary"
                    isDisabled={!data}
                    isLoading={busy}
                    onClick={() =>
                      perform(async () => {
                        const q = await invoke<Question>(
                          "generate_interview_question",
                          { topicId, lang },
                        );
                        setQuestionId(q.id);
                        show(
                          await invoke<Session>("start_interview", {
                            questionId: q.id,
                          }),
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
                        <h3>{preview.question}</h3>
                        <p>{preview.background}</p>
                        <ul>
                          {preview.constraints.map((c, i) => (
                            <li key={i}>{c}</li>
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
                              perform(async () =>
                                show(
                                  await invoke<Session>("start_interview", {
                                    questionId: preview.id,
                                  }),
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
              <p className="muted">{t("noSession")}</p>
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
                    variant="ghost"
                    size="sm"
                    isDisabled={busy}
                    onClick={() => setConfirm("session")}
                  />
                </div>
                <div className="question-context">
                  <h3>{session.question.question}</h3>
                  <p>{session.question.background}</p>
                  <ul>
                    {session.question.constraints.map((c, i) => (
                      <li key={i}>{c}</li>
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
                      <p>{turn.text}</p>
                      {turn.aiAssisted && (
                        <details className="reference-content">
                          <summary>{t("refKeyPoints")}</summary>
                          <ul>
                            {session.references
                              .find((r) => r.turnIndex === i)
                              ?.keyPoints.map((point, n) => (
                                <li key={n}>{point}</li>
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
                      isLoading={busy}
                      onClick={() =>
                        perform(
                          async () =>
                            show(
                              await invoke<Session>("retry_interview", {
                                sessionId: session.id,
                                lang,
                              }),
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
                        isDisabled={!draft.trim() || answered >= 20}
                        isLoading={busy}
                        onClick={() =>
                          perform(
                            async () =>
                              show(
                                await invoke<Session>(
                                  "submit_interview_answer",
                                  {
                                    sessionId: session.id,
                                    answer: draft,
                                    lang,
                                  },
                                ),
                              ),
                            true,
                          )
                        }
                      />
                      <Button
                        label={
                          referenceReady ? t("refReady") : t("aiReference")
                        }
                        isDisabled={busy || referenceReady}
                        onClick={() =>
                          perform(
                            async () =>
                              show(
                                await invoke<Session>(
                                  "show_interview_reference",
                                  {
                                    sessionId: session.id,
                                    turnIndex: session.turns.length,
                                    lang,
                                  },
                                ),
                              ),
                            true,
                          )
                        }
                      />
                      <Button
                        label={t("finishFeedback")}
                        variant="ghost"
                        isDisabled={busy || !!draft.trim() || answered === 0}
                        onClick={() =>
                          perform(
                            async () =>
                              show(
                                await invoke<Session>("finish_interview", {
                                  sessionId: session.id,
                                  lang,
                                }),
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
                    <p>{session.feedback.summary}</p>
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
                          <p>
                            <strong>{t("evidence")}</strong>
                            {d.evidence}
                          </p>
                          <p>
                            <strong>{t("advice")}</strong>
                            {d.improvement}
                          </p>
                        </div>
                      ))}
                    </div>
                    {(
                      [
                        [t("strengths"), session.feedback.strengths],
                        [t("gaps"), session.feedback.gaps],
                        [t("nextTopics"), session.feedback.nextPracticeTopics],
                      ] as const
                    ).map(([name, items]) => (
                      <section key={name}>
                        <h3>{name}</h3>
                        <ul>
                          {items.map((s, i) => (
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
