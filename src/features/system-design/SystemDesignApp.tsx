import { useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@astryxdesign/core/Button";
import { TextArea } from "@astryxdesign/core/TextArea";
import { TextInput } from "@astryxdesign/core/TextInput";
import { NumberInput } from "@astryxdesign/core/NumberInput";
import { Selector } from "@astryxdesign/core/Selector";
import { Badge } from "@astryxdesign/core/Badge";
import { Banner } from "@astryxdesign/core/Banner";
import { Collapsible } from "@astryxdesign/core/Collapsible";
import { lookup, useI18n, type Lang } from "../../i18n";
import { localizeError } from "../../errors";
import { readablePartialJson } from "../interview/stream";
import { Diagram, Markdown } from "./DesignDocument";
import "../interview/interview.css";
import "./system-design.css";

const stages = [
  "requirements",
  "estimation",
  "high_level",
  "deep_dive",
  "tradeoffs",
] as const;
type Stage = (typeof stages)[number] | "feedback";
type ScenarioText = {
  title: string;
  description: string;
  functionalRequirements: string[];
  scaleRanges: string[];
  reliabilityConstraints: string[];
  deepDiveDirections: string[];
};
type Scenario = ScenarioText & { id: string; en: ScenarioText };
type Question = {
  id: string;
  seedId: string;
  lang: Lang;
  provider: string;
  model: string;
  createdAt: string;
  content: { question: string; background: string; constraints: string[] };
};
type Workspace = {
  drafts: Partial<Record<Stage, string>>;
  notes: string;
  diagram: string;
};
type Assessment = { rating: string; evidence: string; improvement: string };
type Session = {
  id: string;
  question: Question;
  provider: string;
  model: string;
  stage: Stage;
  status: string;
  workspace: Workspace;
  turns: {
    role: string;
    text: string;
    stage: Stage;
    observation: string | null;
  }[];
  suggestedStage: Stage | null;
  lastError: string | null;
  feedback: {
    summary: string;
    assessment: Record<string, Assessment>;
    strengths: string[];
    gaps: string[];
    nextPracticeTopics: string[];
  } | null;
  createdAt: string;
};
type Summary = {
  id: string;
  title: string;
  stage: Stage;
  status: string;
  lang: Lang;
  createdAt: string;
};
type Config = { provider: string; model: string; timeoutSeconds: number };
type Overview = {
  scenarios: Scenario[];
  questions: Question[];
  sessions: Summary[];
  settings: { config: Config; keyStatus: string };
};
const emptyWorkspace = (): Workspace => ({
  drafts: {},
  notes: "",
  diagram: "",
});

export default function SystemDesignApp({ dark }: { dark: boolean }) {
  const { t, lang } = useI18n();
  const [data, setData] = useState<Overview | null>(null),
    [scenarioId, setScenarioId] = useState("url-shortener"),
    [questionId, setQuestionId] = useState("");
  const [session, setSession] = useState<Session | null>(null),
    [workspace, setWorkspace] = useState<Workspace>(emptyWorkspace);
  const [busy, setBusy] = useState(false),
    [saving, setSaving] = useState(false),
    [error, setError] = useState(""),
    [live, setLive] = useState(""),
    [saveFailed, setSaveFailed] = useState(false);
  const [online, setOnline] = useState(navigator.onLine),
    [apiKey, setApiKey] = useState("");
  const [config, setConfig] = useState<Config>({
    provider: "claude",
    model: "claude-haiku-5-5",
    timeoutSeconds: 120,
  });
  const queue = useRef(Promise.resolve()),
    saveError = useRef(""),
    workRef = useRef(workspace),
    revision = useRef(0),
    running = useRef(false),
    activeId = useRef("");
  function show(s: Session) {
    activeId.current = s.id;
    setScenarioId(s.question.seedId);
    setQuestionId(s.question.id);
    setSession(s);
    workRef.current = s.workspace;
    setWorkspace(s.workspace);
    saveError.current = "";
    setSaveFailed(false);
  }
  async function overview() {
    const d = await invoke<Overview>("sd_overview");
    setData(d);
    setConfig(d.settings.config);
    return d;
  }
  useEffect(() => {
    if (!isTauri()) return;
    let disposed = false;
    const stop = listen<string>("sd-stream", (e) => setLive(e.payload));
    overview()
      .then(async (d) => {
        const previous =
          d.sessions.find((s) => s.status !== "finished") ?? d.sessions[0];
        if (previous) {
          const s = await invoke<Session>("sd_session", {
            sessionId: previous.id,
          });
          if (!disposed) show(s);
        }
      })
      .catch((e) => setError(String(e)));
    const update = () => setOnline(navigator.onLine);
    window.addEventListener("online", update);
    window.addEventListener("offline", update);
    return () => {
      disposed = true;
      void stop.then((fn) => fn());
      window.removeEventListener("online", update);
      window.removeEventListener("offline", update);
    };
  }, []);
  function updateWorkspace(next: Workspace) {
    workRef.current = next;
    setWorkspace(next);
    if (!session || session.status !== "active") return;
    const id = session.id,
      version = ++revision.current;
    setSaving(true);
    queue.current = queue.current
      .then(() =>
        invoke("sd_save_workspace", { sessionId: id, workspace: next }),
      )
      .then(() => {
        saveError.current = "";
        setSaveFailed(false);
      })
      .catch((e) => {
        saveError.current = String(e);
        setSaveFailed(true);
        setError(String(e));
      })
      .then(() => {
        if (version === revision.current) setSaving(false);
      });
  }
  async function perform(task: () => Promise<void>) {
    if (running.current) return;
    running.current = true;
    setBusy(true);
    setError("");
    setLive("");
    try {
      await queue.current;
      if (saveError.current) throw new Error(saveError.current);
      await task();
      await overview();
    } catch (e) {
      setError(String(e));
      if (activeId.current && !saveError.current) {
        try {
          show(
            await invoke<Session>("sd_session", {
              sessionId: activeId.current,
            }),
          );
        } catch {
          /* Keep visible local text if recovery itself fails. */
        }
      }
    } finally {
      running.current = false;
      setBusy(false);
      setLive("");
    }
  }
  const raw = data?.scenarios.find((s) => s.id === scenarioId),
    seed = raw && (lang === "en" ? raw.en : raw);
  const cached =
    data?.questions.filter((q) => q.seedId === scenarioId && q.lang === lang) ??
    [];
  const preview = cached.find((q) => q.id === questionId) ?? cached[0];
  const active = session?.status === "active",
    pending =
      session?.status === "awaiting_reply" ||
      session?.status === "feedback_pending";
  const draft = session ? (workspace.drafts[session.stage] ?? "") : "";
  const stageLabel = (stage: string) => lookup(t, "sdStage_", stage);
  const date = (value: string) =>
    new Date(value).toLocaleString(lang === "zh" ? "zh-CN" : "en");
  const orderedAssessment = session?.feedback
    ? [
        "requirements",
        "estimation",
        "architecture",
        "scalability",
        "availability",
        "correctness",
        "tradeoffs",
        "communication",
      ].map((name) => [name, session.feedback!.assessment[name]] as const)
    : [];
  const unfinishedDrafts = Object.entries(workspace.drafts).filter(([, v]) =>
    v?.trim(),
  );
  function request(action: "answer" | "feedback" | "retry") {
    if (!session) return;
    void perform(async () =>
      show(
        await invoke<Session>("sd_request", {
          sessionId: session.id,
          action,
          answer: action === "answer" ? draft : null,
        }),
      ),
    );
  }
  const list = (values: string[]) => (
    <ul>
      {values.map((v, i) => (
        <li key={i}>{v}</li>
      ))}
    </ul>
  );
  return (
    <div className="interview-app sd-app">
      <section className="dd-hero">
        <div>
          <h1>{t("moduleSystemDesign")}</h1>
          <p className="muted">{t("sdTagline")}</p>
        </div>
        <div className="dd-status">
          <Badge
            variant={online ? "success" : "warning"}
            label={online ? t("online") : t("offline")}
          />
          <Badge
            variant="purple"
            label={`${session?.provider ?? config.provider} · ${session?.model ?? config.model}`}
          />
        </div>
      </section>
      <p className="privacy-note">
        {t("sdScope")} {t("privacyNote")}
      </p>
      <div className="provider-settings">
        <Collapsible
          defaultIsOpen={false}
          trigger={
            <strong>
              {t("sdModelSettings")} ·{" "}
              {data?.settings.keyStatus === "saved"
                ? t("keySaved")
                : t("keyMissing")}
            </strong>
          }
        >
          <div className="provider-fields">
            <Selector
              label={t("sdProvider")}
              size="sm"
              width={150}
              value={config.provider}
              isDisabled={busy}
              options={[
                { value: "claude", label: "Claude" },
                { value: "deepseek", label: "DeepSeek" },
              ]}
              onChange={(v) => {
                setApiKey("");
                setConfig({
                  ...config,
                  provider: v,
                  model: v === "claude" ? "claude-haiku-5-5" : "deepseek-flash",
                });
              }}
            />
            <TextInput
              label={t("modelName")}
              size="sm"
              value={config.model}
              onChange={(v) => setConfig({ ...config, model: v })}
              isDisabled={busy}
            />
            <NumberInput
              label={t("timeoutSeconds")}
              size="sm"
              min={15}
              max={120}
              value={config.timeoutSeconds}
              onChange={(v) => setConfig({ ...config, timeoutSeconds: v })}
              isDisabled={busy}
            />
            <TextInput
              label={`${config.provider} API Key`}
              type="password"
              autoComplete="off"
              size="sm"
              value={apiKey}
              onChange={setApiKey}
              placeholder={t("apiKeyPlaceholder")}
              isDisabled={busy}
            />
            <Button
              label={t("saveSettings")}
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
          </div>
          <p className="muted">{t("settingsNote")}</p>
        </Collapsible>
      </div>
      {!isTauri() && <Banner status="warning" title={t("needDesktop")} />}
      {error && (
        <div className="sd-message">
          <Banner status="error" title={localizeError(error, lang)} />
        </div>
      )}
      {busy && (
        <div className="live-response" aria-live="polite">
          <strong>{t("busyNote")}</strong>
          {readablePartialJson(live).map((s, i) => (
            <section key={i}>
              <p>{s.text}</p>
            </section>
          ))}
        </div>
      )}
      {!data && isTauri() && <p className="sd-message">{t("sdLoading")}</p>}
      <div className="interview-main">
        <aside>
          <h2>{t("sdScenarios")}</h2>
          {data?.scenarios.map((s) => (
            <Button
              key={s.id}
              label={lang === "en" ? s.en.title : s.title}
              variant={s.id === scenarioId ? "primary" : "ghost"}
              isDisabled={busy}
              onClick={() => {
                setScenarioId(s.id);
                setQuestionId("");
              }}
            />
          ))}
          <h3>{t("sdHistory")}</h3>
          {data?.sessions.map((s) => (
            <Button
              key={s.id}
              label={`${s.title.slice(0, 70)} · ${stageLabel(s.stage)} · ${lookup(t, "state_", s.status)}`}
              tooltip={`${s.title} · ${date(s.createdAt)}`}
              variant={session?.id === s.id ? "primary" : "ghost"}
              size="sm"
              isDisabled={busy}
              onClick={() =>
                perform(async () =>
                  show(
                    await invoke<Session>("sd_session", { sessionId: s.id }),
                  ),
                )
              }
            />
          ))}
        </aside>
        <main className="interview-workspace">
          {seed && (
            <section className="topic-panel">
              <h2>{seed.title}</h2>
              <p>{seed.description}</p>
              <Collapsible trigger={t("sdSeed")} defaultIsOpen={false}>
                <h3>{t("sdFunctions")}</h3>
                {list(seed.functionalRequirements)}
                <h3>{t("sdScale")}</h3>
                {list(seed.scaleRanges)}
                <h3>{t("sdReliability")}</h3>
                {list(seed.reliabilityConstraints)}
              </Collapsible>
              <div className="actions">
                <Button
                  label={t("sdGenerate")}
                  variant="primary"
                  isDisabled={busy || !online || !data}
                  onClick={() =>
                    perform(async () => {
                      const q = await invoke<Question>("sd_generate", {
                        scenarioId,
                        lang,
                      });
                      setQuestionId(q.id);
                      show(
                        await invoke<Session>("sd_start", { questionId: q.id }),
                      );
                    })
                  }
                />
              </div>
              <h3>{t("sdCached")}</h3>
              {preview ? (
                <div className="sd-cache">
                  <Selector
                    label={t("pickCached")}
                    value={preview.id}
                    options={cached.map((q) => ({
                      value: q.id,
                      label: `${date(q.createdAt)} · ${q.content.question}`,
                    }))}
                    isDisabled={busy}
                    onChange={setQuestionId}
                  />
                  <Button
                    label={t("sdCache")}
                    size="sm"
                    isDisabled={busy}
                    onClick={() =>
                      perform(async () =>
                        show(
                          await invoke<Session>("sd_start", {
                            questionId: preview.id,
                          }),
                        ),
                      )
                    }
                  />
                </div>
              ) : (
                <p className="muted">{t("sdNoCache")}</p>
              )}
            </section>
          )}
          {!session ? (
            <p>{t("sdEmpty")}</p>
          ) : (
            <section className="conversation-panel">
              <div className="session-heading">
                <h2>{t("sdCurrent")}</h2>
                <Badge
                  variant={
                    pending
                      ? "warning"
                      : session.status === "finished"
                        ? "success"
                        : "info"
                  }
                  label={lookup(t, "state_", session.status)}
                />
              </div>
              <div className="question-context">
                <h3>{session.question.content.question}</h3>
                <p>{session.question.content.background}</p>
                {list(session.question.content.constraints)}
                <p className="muted">
                  {t("sdSavedLanguage", {
                    language:
                      session.question.lang === "zh"
                        ? t("sdLangZh")
                        : t("sdLangEn"),
                  })}
                </p>
              </div>
              <nav className="sd-stages" aria-label={t("sdStages")}>
                {stages.map((stage, i) => (
                  <Button
                    key={stage}
                    label={`${i + 1}. ${stageLabel(stage)}`}
                    variant={session.stage === stage ? "primary" : "secondary"}
                    size="sm"
                    aria-pressed={session.stage === stage}
                    isDisabled={busy || !active}
                    onClick={() =>
                      perform(async () =>
                        show(
                          await invoke<Session>("sd_change_stage", {
                            sessionId: session.id,
                            stage,
                          }),
                        ),
                      )
                    }
                  />
                ))}
              </nav>
              <p className="muted">{t("sdStageTip")}</p>
              {session.suggestedStage &&
                active &&
                session.suggestedStage !== session.stage && (
                  <p className="sd-suggestion">
                    {t("sdSuggest", {
                      stage: stageLabel(session.suggestedStage),
                    })}
                  </p>
                )}
              <div className="sd-workspace">
                <Collapsible
                  trigger={<strong>{t("sdWorkspace")}</strong>}
                  defaultIsOpen={false}
                >
                  <TextArea
                    label={t("sdNotes")}
                    description={t("sdNotesHint")}
                    value={workspace.notes}
                    placeholder={t("sdNotesPlaceholder")}
                    rows={8}
                    maxLength={20000}
                    isDisabled={busy}
                    isReadOnly={!active}
                    onChange={(notes) =>
                      updateWorkspace({ ...workRef.current, notes })
                    }
                  />

                  <Collapsible
                    trigger={t("sdNotesPreview")}
                    defaultIsOpen={false}
                  >
                    <Markdown text={workspace.notes || t("sdNoNotes")} />
                  </Collapsible>
                  <TextArea
                    label={t("sdDiagram")}
                    description={t("sdDiagramHint")}
                    value={workspace.diagram}
                    rows={5}
                    maxLength={12000}
                    isDisabled={busy}
                    isReadOnly={!active}
                    onChange={(diagram) =>
                      updateWorkspace({ ...workRef.current, diagram })
                    }
                  />
                  <Diagram
                    key={session.id}
                    source={workspace.diagram}
                    dark={dark}
                  />
                </Collapsible>
              </div>
              <h3>{t("sdThread")}</h3>
              <div className="turns">
                {session.turns.map((turn, i) => (
                  <article className={`turn ${turn.role}`} key={i}>
                    <strong>
                      {turn.role === "candidate"
                        ? t("sdCandidate")
                        : t("sdInterviewer")}{" "}
                      · {stageLabel(turn.stage)}
                    </strong>
                    {turn.observation && (
                      <p className="muted">{turn.observation}</p>
                    )}
                    <Markdown text={turn.text} />
                  </article>
                ))}
              </div>
              {pending && (
                <div className="pending-request">
                  <p>{t("sdPending")}</p>
                  {session.lastError && (
                    <p role="alert">{localizeError(session.lastError, lang)}</p>
                  )}
                  <Button
                    label={t("retryPending")}
                    variant="primary"
                    isDisabled={busy || !online}
                    onClick={() => request("retry")}
                  />
                </div>
              )}
              {active && (
                <>
                  <TextArea
                    label={`${stageLabel(session.stage)} · ${t("sdAnswer")}`}
                    description={t("sdAnswerHint")}
                    rows={7}
                    maxLength={6000}
                    value={draft}
                    placeholder={t("sdAnswerPlaceholder")}
                    isDisabled={busy}
                    onChange={(answer) =>
                      updateWorkspace({
                        ...workRef.current,
                        drafts: {
                          ...workRef.current.drafts,
                          [session.stage]: answer,
                        },
                      })
                    }
                  />
                  <p className="muted" aria-live="polite">
                    {busy
                      ? t("busyNote")
                      : saving
                        ? t("saving")
                        : saveFailed
                          ? t("draftFailed")
                          : t("saved")}
                  </p>
                  {saveFailed && (
                    <Button
                      label={t("sdRetrySave")}
                      isDisabled={busy}
                      onClick={() => updateWorkspace(workRef.current)}
                    />
                  )}
                  <div className="actions">
                    <Button
                      label={t("sdSubmit")}
                      variant="primary"
                      isDisabled={
                        busy || !online || !draft.trim() || saveFailed
                      }
                      onClick={() => request("answer")}
                    />
                    <Button
                      label={t("sdFinish")}
                      isDisabled={
                        busy ||
                        !online ||
                        unfinishedDrafts.length > 0 ||
                        saveFailed
                      }
                      onClick={() => request("feedback")}
                    />
                  </div>
                  {unfinishedDrafts.length > 0 && (
                    <p className="muted">
                      {t("sdUnsubmitted")}{" "}
                      {unfinishedDrafts
                        .map(([stage]) => stageLabel(stage))
                        .join(" / ")}
                    </p>
                  )}
                </>
              )}
              {session.feedback && (
                <section className="feedback">
                  <h2>{t("sdStage_feedback")}</h2>
                  <Markdown text={session.feedback.summary} />
                  <div className="feedback-grid">
                    {orderedAssessment.map(([name, d]) => (
                      <article key={name} className="feedback-dimension">
                        <div className="dimension-head">
                          <strong>{lookup(t, "sdAssessment_", name)}</strong>
                          <Badge
                            label={lookup(t, "rating_", d.rating)}
                            variant={
                              d.rating === "good"
                                ? "success"
                                : d.rating === "incorrect"
                                  ? "error"
                                  : d.rating === "partial"
                                    ? "warning"
                                    : "neutral"
                            }
                          />
                        </div>
                        <p>
                          <strong>{t("evidence")}</strong> {d.evidence}
                        </p>
                        <p>
                          <strong>{t("advice")}</strong> {d.improvement}
                        </p>
                      </article>
                    ))}
                  </div>
                  <h3>{t("strengths")}</h3>
                  {list(session.feedback.strengths)}
                  <h3>{t("gaps")}</h3>
                  {list(session.feedback.gaps)}
                  <h3>{t("nextTopics")}</h3>
                  {list(session.feedback.nextPracticeTopics)}
                </section>
              )}
            </section>
          )}
        </main>
      </div>
    </div>
  );
}
