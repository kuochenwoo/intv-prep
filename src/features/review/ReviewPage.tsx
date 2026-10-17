import { useEffect, useMemo, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Button } from "@astryxdesign/core/Button";
import { Banner } from "@astryxdesign/core/Banner";
import { lookup, useI18n } from "../../i18n";
import { localizeError } from "../../errors";
import { EmptyState } from "../../EmptyState";
import "../interview/interview.css";
import "./review.css";
import { watchReviewChanges } from "./events";
import { summarizeCosts } from "./costs";

type Module = "lc" | "p1" | "p2";
type Title = { zh: string; en: string };
type Activity = {
  at: string;
  module: Module;
  refId: string;
  title: Title;
  status: string;
  passed: number | null;
  total: number | null;
  firstTry: boolean;
  good: number;
  weak: number;
  stages: number;
};
type Evidence = {
  at: string;
  kind: "answer" | "feedback" | "judge";
  text: string;
  label: string;
  sessionId: string | null;
  source: Title;
  aiAssisted: boolean;
};
type Status = "open" | "improving" | "resolved";
type WeakPoint = {
  concept: string;
  module: Module;
  title: Title;
  practiceId: string;
  sessionId: string | null;
  status: Status;
  stage: number;
  weakCount: number;
  firstWeakMs: number;
  lastMs: number;
  dueMs: number | null;
  evidence: Evidence[];
};
type Overview = {
  nowMs: number;
  activity: Activity[];
  weakPoints: WeakPoint[];
  timed: { startedMs: number; usedMs: number }[];
  costs: { at: string; usd: number | null }[];
};
/** Where "Practice this" or "Open session" should take the user. */
export type ReviewTarget = { module: Module; id: string; session: boolean };

const DAY = 86_400_000;
const WEEKS = 26;
const pad = (n: number) => String(n).padStart(2, "0");
/** Local calendar day, so "today" follows the user's clock rather than UTC. */
const dayKey = (ms: number) => {
  const d = new Date(ms);
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
};
const startOfDay = (ms: number) => {
  const d = new Date(ms);
  d.setHours(0, 0, 0, 0);
  return d.getTime();
};
const addDays = (ms: number, n: number) => {
  const d = new Date(ms);
  d.setDate(d.getDate() + n);
  return d.getTime();
};
const moduleTint: Record<Module, string> = {
  lc: "tint-blue",
  p1: "tint-purple",
  p2: "tint-teal",
};
const statusTint: Record<Status, string> = {
  open: "tint-red",
  improving: "tint-orange",
  resolved: "tint-green",
};
const heatLevel = (n: number) =>
  n === 0 ? 0 : n === 1 ? 1 : n <= 3 ? 2 : n <= 6 ? 3 : 4;

export function ReviewPage({
  active,
  onNavigate,
  onDue,
}: {
  active: boolean;
  onNavigate: (target: ReviewTarget) => void;
  onDue: (count: number) => void;
}) {
  const { t, lang } = useI18n();
  const locale = lang === "zh" ? "zh-CN" : "en";
  const [data, setData] = useState<Overview | null>(null),
    [error, setError] = useState(""),
    [day, setDay] = useState<string | null>(null),
    [statusFilter, setStatusFilter] = useState<Status | "all">("all"),
    [moduleFilter, setModuleFilter] = useState<Module | "all">("all"),
    [open, setOpen] = useState(""),
    [dismissing, setDismissing] = useState("");
  const loadVersion = useRef(0);
  async function load() {
    if (!isTauri()) return;
    const version = ++loadVersion.current;
    try {
      const result = await invoke<Overview>("review_overview");
      if (version === loadVersion.current) {
        setData(result);
        setError("");
      }
    } catch (e) {
      if (version === loadVersion.current) setError(String(e));
    }
  }
  useEffect(() => {
    if (active) void load();
  }, [active]);
  useEffect(() => {
    void load();
    return watchReviewChanges(() => void load());
  }, []);

  const now = data?.nowMs ?? Date.now();
  const today = dayKey(now);
  const endToday = addDays(startOfDay(now), 1);
  const title = (x: Title) => (lang === "en" ? x.en : x.zh);
  const conceptTitle = (w: WeakPoint) => {
    if (w.module === "p2")
      return lookup(t, "sdAssessment_", w.concept.slice(3));
    if (w.module === "lc")
      return `#${w.practiceId.replace(/^lc-/, "")} ${title(w.title)}`;
    return title(w.title);
  };
  const time = (iso: string) =>
    new Date(iso).toLocaleTimeString(locale, {
      hour: "2-digit",
      minute: "2-digit",
    });
  const date = (ms: number) =>
    new Date(ms).toLocaleDateString(locale, { month: "short", day: "numeric" });
  const ago = (ms: number) => {
    const days = Math.round((startOfDay(now) - startOfDay(ms)) / DAY);
    return new Intl.RelativeTimeFormat(locale, { numeric: "auto" }).format(
      -days,
      "day",
    );
  };
  const usd = (n: number) =>
    `$${n > 0 && n < 0.01 ? n.toFixed(4) : n.toFixed(2)}`;

  const perDay = useMemo(() => {
    const m = new Map<string, number>();
    for (const a of data?.activity ?? []) {
      const k = dayKey(Date.parse(a.at));
      m.set(k, (m.get(k) ?? 0) + 1);
    }
    return m;
  }, [data]);

  const due = (data?.weakPoints ?? [])
    .filter((w) => w.dueMs !== null && w.dueMs < endToday)
    .sort((a, b) => a.dueMs! - b.dueMs!);
  const tomorrow = (data?.weakPoints ?? []).filter(
    (w) =>
      w.dueMs !== null && w.dueMs >= endToday && w.dueMs < addDays(endToday, 1),
  );
  useEffect(() => onDue(due.length), [due.length]);

  // The day card: today unless a heatmap day is picked.
  const shown = day ?? today;
  const onDay = (ms: number) => dayKey(ms) === shown;
  const dayActivity = (data?.activity ?? []).filter((a) =>
    onDay(Date.parse(a.at)),
  );
  const lcDay = dayActivity.filter((a) => a.module === "lc");
  const accepted = new Set(
    lcDay.filter((a) => a.status === "accepted").map((a) => a.refId),
  );
  const firstTry = lcDay.filter((a) => a.firstTry).length;
  const dd = dayActivity.filter((a) => a.module === "p1").length;
  const sd = dayActivity.filter((a) => a.module === "p2").length;
  const timedMs = (data?.timed ?? [])
    .filter((x) => onDay(x.startedMs))
    .reduce((s, x) => s + x.usedMs, 0);
  const newWeak = (data?.weakPoints ?? []).filter((w) =>
    onDay(w.firstWeakMs),
  ).length;
  const cost = summarizeCosts(
    (data?.costs ?? []).filter((c) => onDay(Date.parse(c.at))),
  );
  const costText = !data
    ? t("usageUnknown")
    : cost.unknown === 0
      ? usd(cost.total)
      : cost.known === 0
        ? t("usageUnknown")
        : t("reviewPartialCost", { cost: usd(cost.total) });
  // One row per problem (latest result and attempt count), one per interview.
  const rows = useMemo(() => {
    const byProblem = new Map<string, { last: Activity; tries: number }>();
    const out: { last: Activity; tries: number }[] = [];
    for (const a of dayActivity) {
      if (a.module !== "lc") {
        out.push({ last: a, tries: 1 });
        continue;
      }
      const seen = byProblem.get(a.refId);
      if (seen) seen.tries += 1;
      else {
        const row = { last: a, tries: 1 };
        byProblem.set(a.refId, row);
        out.push(row);
      }
    }
    return out;
  }, [data, shown]);
  const minutes = Math.round(timedMs / 60000);
  const timedText =
    minutes >= 60
      ? t("reviewHoursMinutes", {
          h: Math.floor(minutes / 60),
          m: minutes % 60,
        })
      : t("reviewMinutes", { m: minutes });

  // Heatmap: WEEKS columns ending with the current week, Sunday on top.
  const start = addDays(
    startOfDay(now),
    -((WEEKS - 1) * 7 + new Date(now).getDay()),
  );
  const cells: { ms: number; n: number; future: boolean }[] = [];
  for (let i = 0; i < WEEKS * 7; i++) {
    const ms = addDays(start, i);
    cells.push({ ms, n: perDay.get(dayKey(ms)) ?? 0, future: ms >= endToday });
  }
  let streak = 0;
  for (
    let d = perDay.get(today) ? startOfDay(now) : addDays(startOfDay(now), -1);
    perDay.get(dayKey(d));
    d = addDays(d, -1)
  )
    streak += 1;
  const windowStats = (fromDays: number) => {
    const lo = addDays(startOfDay(now), -fromDays - 6),
      hi = addDays(startOfDay(now), -fromDays + 1);
    const inside = (data?.activity ?? []).filter((a) => {
      const ms = Date.parse(a.at);
      return ms >= lo && ms < hi;
    });
    return {
      problems: new Set(
        inside
          .filter((a) => a.module === "lc" && a.status === "accepted")
          .map((a) => a.refId),
      ).size,
      sessions: inside.filter((a) => a.module !== "lc").length,
    };
  };
  const thisWeek = windowStats(0),
    lastWeek = windowStats(7);
  const compare = (a: number, b: number) =>
    a > b
      ? t("reviewUpFrom", { n: b })
      : a < b
        ? t("reviewDownFrom", { n: b })
        : t("reviewSameAs");

  const weak = (data?.weakPoints ?? []).filter(
    (w) =>
      (statusFilter === "all" || w.status === statusFilter) &&
      (moduleFilter === "all" || w.module === moduleFilter),
  );
  const count = (s: Status) =>
    (data?.weakPoints ?? []).filter((w) => w.status === s).length;
  const evidenceLabel = (w: WeakPoint, e: Evidence) => {
    if (e.kind === "judge")
      return `${lookup(t, "status_", e.label)} · ${t("reviewPassed", { n: e.text })}`;
    if (e.kind === "answer") return t("reviewYourAnswer");
    return `${t("reviewFeedback")} · ${lookup(t, w.module === "p2" ? "sdAssessment_" : "dim_", e.label)}`;
  };
  async function dismiss(w: WeakPoint) {
    setDismissing(w.concept);
    try {
      await invoke("review_dismiss", { concept: w.concept });
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setDismissing("");
    }
  }
  const practice = (w: WeakPoint) =>
    onNavigate({ module: w.module, id: w.practiceId, session: false });

  return (
    <div className="interview-app review-app">
      <section className="dd-hero">
        <div>
          <h1>{t("moduleReview")}</h1>
          <p className="muted">{t("reviewTagline")}</p>
        </div>
      </section>
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
      <div className="review-grid">
        <section className="card review-today">
          <div className="review-card-head">
            <h2>
              📅{" "}
              {shown === today
                ? t("reviewToday")
                : date(Date.parse(`${shown}T12:00:00`))}
            </h2>
            {shown === today ? (
              <span className="muted">
                {new Date(now).toLocaleDateString(locale, {
                  weekday: "short",
                  month: "short",
                  day: "numeric",
                })}
              </span>
            ) : (
              <Button
                size="sm"
                variant="ghost"
                label={t("reviewBackToToday")}
                onClick={() => setDay(null)}
              />
            )}
          </div>
          <div className="usage-stats review-stats">
            <div className="usage-stat tint-green">
              <span>{t("reviewStatAccepted", { n: firstTry })}</span>
              <strong>{accepted.size}</strong>
            </div>
            <div className="usage-stat tint-blue">
              <span>{t("reviewStatSessions", { dd, sd })}</span>
              <strong>{dd + sd}</strong>
            </div>
            <div className="usage-stat tint-teal">
              <span>{t("reviewStatTimed")}</span>
              <strong>{timedText}</strong>
            </div>
            <div className="usage-stat tint-orange">
              <span>{t("reviewStatNewWeak")}</span>
              <strong>{newWeak}</strong>
            </div>
            <div className="usage-stat tint-purple">
              <span>{t("reviewStatCost")}</span>
              <strong>{costText}</strong>
              {cost.unknown > 0 && (
                <small>{t("reviewUnknownCosts", { n: cost.unknown })}</small>
              )}
            </div>
          </div>
          {rows.length ? (
            <ul className="review-rows">
              {rows.map(({ last: a, tries }) => (
                <li key={`${a.module}-${a.refId}-${a.at}`}>
                  {a.module === "lc" ? (
                    <span
                      className={`usage-pill ${a.status === "accepted" ? (tries > 1 ? "tint-orange" : "tint-green") : "tint-red"}`}
                    >
                      {a.status === "accepted" && tries > 1
                        ? t("reviewTries", { n: tries })
                        : lookup(t, "status_", a.status)}
                    </span>
                  ) : (
                    <span className={`usage-pill ${moduleTint[a.module]}`}>
                      {t(
                        a.module === "p1"
                          ? "moduleDeepDive"
                          : "moduleSystemDesign",
                      )}
                    </span>
                  )}
                  <span className="review-row-title">
                    {a.module === "lc"
                      ? `#${a.refId.replace(/^lc-/, "")} `
                      : ""}
                    {title(a.title)}
                    <span className="muted">
                      {" · "}
                      {a.module === "lc"
                        ? a.firstTry
                          ? t("reviewFirstTry")
                          : t("reviewPassed", { n: `${a.passed}/${a.total}` })
                        : a.status !== "finished"
                          ? t("reviewInProgress")
                          : a.module === "p1"
                            ? t("reviewRatings", { good: a.good, weak: a.weak })
                            : t("reviewStages", { n: a.stages })}
                    </span>
                  </span>
                  <span className="muted">{time(a.at)}</span>
                </li>
              ))}
            </ul>
          ) : (
            <EmptyState
              compact
              icon="🌱"
              text={
                timedMs > 0
                  ? t("reviewTimedOnly")
                  : shown === today
                    ? t("reviewNothingToday")
                    : t("reviewNothingThatDay")
              }
            />
          )}
        </section>

        <section className="card review-due">
          <div className="review-card-head">
            <h2>🔁 {t("reviewDueToday")}</h2>
            <span className="muted">{t("reviewDueHint")}</span>
          </div>
          {due.length ? (
            <ul className="review-rows">
              {due.map((w) => (
                <li key={w.concept}>
                  <span className="review-row-title">
                    <strong>{conceptTitle(w)}</strong>
                    <br />
                    <span className={`usage-pill ${moduleTint[w.module]}`}>
                      {t(`reviewModule_${w.module}`)}
                    </span>{" "}
                    <span className="muted">
                      {t("reviewNth", { n: w.stage + 1 })} ·{" "}
                      {t("reviewLastSeen", { when: ago(w.lastMs) })}
                      {w.dueMs! < startOfDay(now)
                        ? ` · ${t("reviewOverdue")}`
                        : ""}
                    </span>
                  </span>
                  <Button
                    size="sm"
                    variant="secondary"
                    className={`tint-btn ${w.module === "lc" ? "tint-blue" : "tint-orange"}`}
                    label={
                      w.module === "lc"
                        ? t("reviewResolve")
                        : t("reviewPractice")
                    }
                    onClick={() => practice(w)}
                  />
                </li>
              ))}
            </ul>
          ) : (
            <EmptyState compact icon="🎉" text={t("reviewNothingDue")} />
          )}
          {tomorrow.length > 0 && (
            <p className="muted review-tomorrow">
              {t("reviewTomorrow", {
                items: tomorrow
                  .map(conceptTitle)
                  .join(lang === "zh" ? "、" : ", "),
              })}
            </p>
          )}
        </section>

        <section className="card review-streak">
          <div className="review-card-head">
            <h2>🔥 {t("reviewStreak", { n: streak })}</h2>
            <span className="muted">{t("reviewHeatHint")}</span>
          </div>
          <div className="review-heat-wrap">
            <div
              className="review-heat"
              role="grid"
              aria-label={t("reviewHeatLabel")}
            >
              {cells.map((c) => (
                <button
                  key={c.ms}
                  type="button"
                  disabled={c.future}
                  className={`heat-${heatLevel(c.n)} ${dayKey(c.ms) === shown ? "picked" : ""}`}
                  title={`${date(c.ms)} · ${t("reviewActivities", { n: c.n })}`}
                  aria-label={`${date(c.ms)} · ${t("reviewActivities", { n: c.n })}`}
                  onClick={() =>
                    setDay(dayKey(c.ms) === today ? null : dayKey(c.ms))
                  }
                />
              ))}
            </div>
            <div className="review-week">
              <div className="usage-stat tint-green">
                <span>
                  {t("reviewWeekProblems", {
                    cmp: compare(thisWeek.problems, lastWeek.problems),
                  })}
                </span>
                <strong>{thisWeek.problems}</strong>
              </div>
              <div className="usage-stat tint-blue">
                <span>
                  {t("reviewWeekSessions", {
                    cmp: compare(thisWeek.sessions, lastWeek.sessions),
                  })}
                </span>
                <strong>{thisWeek.sessions}</strong>
              </div>
            </div>
          </div>
        </section>

        <section className="card review-weak">
          <div className="review-card-head">
            <h2>🎯 {t("reviewWeakPoints")}</h2>
            <span className="muted">{t("reviewWeakHint")}</span>
          </div>
          <div className="review-filters">
            {(["all", "open", "improving", "resolved"] as const).map((s) => (
              <button
                key={s}
                type="button"
                className={`review-chip ${statusFilter === s ? "on" : ""}`}
                aria-pressed={statusFilter === s}
                onClick={() => setStatusFilter(s)}
              >
                {t(`reviewStatus_${s}`)} ·{" "}
                {s === "all" ? (data?.weakPoints.length ?? 0) : count(s)}
              </button>
            ))}
            <span className="review-filter-gap" />
            {(["all", "lc", "p1", "p2"] as const).map((m) => (
              <button
                key={m}
                type="button"
                className={`review-chip ${moduleFilter === m ? "on" : ""}`}
                aria-pressed={moduleFilter === m}
                onClick={() => setModuleFilter(m)}
              >
                {m === "all" ? t("reviewAllModules") : t(`reviewModule_${m}`)}
              </button>
            ))}
          </div>
          {weak.length ? (
            <div className="review-cards">
              {weak.map((w) => {
                const expanded = open === w.concept;
                return (
                  <article
                    key={w.concept}
                    className={`review-wp ${statusTint[w.status]} ${expanded ? "open" : ""}`}
                  >
                    <button
                      type="button"
                      className="review-wp-head"
                      aria-expanded={expanded}
                      onClick={() => setOpen(expanded ? "" : w.concept)}
                    >
                      <strong>{conceptTitle(w)}</strong>
                      <span className={`usage-pill ${statusTint[w.status]}`}>
                        {t(`reviewStatus_${w.status}`)}
                      </span>
                    </button>
                    <p className="muted">
                      {t(`reviewModule_${w.module}`)} ·{" "}
                      {t("reviewSeen", { n: w.weakCount })} ·{" "}
                      {t("reviewProgress", { n: w.stage, of: 3 })}
                    </p>
                    {expanded && (
                      <>
                        <div className="review-evidence">
                          {w.evidence.map((e, i) => (
                            <div key={i} className="review-quote">
                              <div className="review-quote-who">
                                {evidenceLabel(w, e)} · {date(Date.parse(e.at))}{" "}
                                · {title(e.source)}
                                {e.aiAssisted && (
                                  <span className="usage-pill tint-purple">
                                    {t("reviewAiAssisted")}
                                  </span>
                                )}
                              </div>
                              <p>
                                {e.kind === "judge"
                                  ? t("reviewJudgeEvidence")
                                  : e.text}
                                {e.aiAssisted && (
                                  <span className="muted">
                                    {" "}
                                    {t("reviewAiNote")}
                                  </span>
                                )}
                              </p>
                            </div>
                          ))}
                        </div>
                        <div className="review-wp-actions">
                          <Button
                            size="sm"
                            variant="secondary"
                            className="tint-btn tint-orange"
                            label={t("reviewPracticeThis")}
                            onClick={() => practice(w)}
                          />
                          {w.sessionId && (
                            <Button
                              size="sm"
                              variant="secondary"
                              className="tint-btn tint-blue"
                              label={t("reviewOpenSession")}
                              onClick={() =>
                                onNavigate({
                                  module: w.module,
                                  id: w.sessionId!,
                                  session: true,
                                })
                              }
                            />
                          )}
                          <Button
                            size="sm"
                            variant="ghost"
                            label={t("reviewDismiss")}
                            tooltip={t("reviewDismissTip")}
                            isDisabled={dismissing === w.concept}
                            onClick={() => void dismiss(w)}
                          />
                          <span className="muted review-next">
                            {w.dueMs === null
                              ? t("reviewResolvedNote")
                              : t("reviewNextReview", { when: date(w.dueMs) })}
                          </span>
                        </div>
                      </>
                    )}
                  </article>
                );
              })}
            </div>
          ) : (
            <EmptyState
              icon="🎯"
              text={
                data?.weakPoints.length ? t("reviewNoMatch") : t("reviewNoWeak")
              }
            />
          )}
          <p className="muted review-footnote">{t("reviewFootnote")}</p>
        </section>
      </div>
    </div>
  );
}
