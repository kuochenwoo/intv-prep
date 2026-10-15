import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Button } from "@astryxdesign/core/Button";
import { Selector } from "@astryxdesign/core/Selector";
import { NumberInput } from "@astryxdesign/core/NumberInput";
import { Collapsible } from "@astryxdesign/core/Collapsible";
import { Banner } from "@astryxdesign/core/Banner";
import { useI18n } from "../../i18n";
import { localizeError } from "../../errors";

type Clock = {
  budgetMs: number;
  remainingMs: number;
  deadlineMs: number | null;
  startedAt: number;
  endedAt: number | null;
};
type Rates = {
  input: number;
  output: number;
  cacheRead: number;
  cacheWrite: number;
};
type Usage = {
  id: number;
  createdAt: string;
  provider: string;
  model: string;
  module: string;
  sessionId: string | null;
  operation: string;
  reviewer: boolean;
  status: string;
  inputTokens: number | null;
  outputTokens: number | null;
  cacheReadTokens: number | null;
  cacheWriteTokens: number | null;
  estimatedUsd: number | null;
};
type Accounting = {
  records: Usage[];
  prices: { provider: string; model: string; rates: Rates }[];
};
const clockText = (ms: number) => {
  const n = Math.ceil(ms / 1000);
  return `${Math.floor(n / 60)}:${String(n % 60).padStart(2, "0")}`;
};
function download(filename: string, text: string) {
  const url = URL.createObjectURL(
    new Blob([text], {
      type: filename.endsWith(".json")
        ? "application/json;charset=utf-8"
        : "text/markdown;charset=utf-8",
    }),
  );
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 10000);
}

export function SessionTools({
  module,
  sessionId,
  status,
  provider,
  model,
  busy,
  turns,
  stage,
}: {
  module: "p1" | "p2";
  sessionId: string;
  status: string;
  provider: string;
  model: string;
  busy: boolean;
  turns: number;
  stage?: string;
}) {
  const { t, lang } = useI18n();
  const [clock, setClock] = useState<Clock | null>(null),
    [minutes, setMinutes] = useState("off"),
    [now, setNow] = useState(Date.now()),
    [loaded, setLoaded] = useState(false),
    [working, setWorking] = useState(false),
    [error, setError] = useState(""),
    [notice, setNotice] = useState("");
  const args = { module, sessionId };
  useEffect(() => {
    let active = true;
    if (!isTauri()) return;
    setLoaded(false);
    invoke<Clock | null>("session_timing", { ...args, action: "get" })
      .then((c) => {
        if (active) {
          setClock(c);
          if (c) setMinutes(String(c.budgetMs / 60000));
          setLoaded(true);
        }
      })
      .catch((e) => {
        if (active) setError(String(e));
      });
    return () => {
      active = false;
    };
  }, [module, sessionId, status]);
  useEffect(() => {
    if (!clock?.deadlineMs) return;
    const id = setInterval(() => setNow(Date.now()), 250);
    return () => clearInterval(id);
  }, [clock?.deadlineMs]);
  async function timing(action: string) {
    setWorking(true);
    setError("");
    try {
      setClock(
        await invoke<Clock | null>("session_timing", {
          ...args,
          action,
          minutes: Number(minutes),
        }),
      );
      setNow(Date.now());
    } catch (e) {
      setError(String(e));
    } finally {
      setWorking(false);
    }
  }
  const left = clock
    ? clock.deadlineMs === null
      ? clock.remainingMs
      : Math.max(0, clock.deadlineMs - now)
    : 0;
  async function exportFile(format: string) {
    setWorking(true);
    setError("");
    try {
      const file = await invoke<{ filename: string; text: string }>(
        "export_interview",
        { ...args, format, lang },
      );
      download(file.filename, file.text);
      setNotice(t("exportReady"));
    } catch (e) {
      setError(String(e));
    } finally {
      setWorking(false);
    }
  }
  return (
    <section className="session-tools" aria-label={t("sessionTools")}>
      <div className="actions">
        {!clock ? (
          <>
            <Selector
              size="sm"
              label={t("timer")}
              value={minutes}
              isDisabled={!loaded || working || busy || status !== "active"}
              onChange={setMinutes}
              options={[
                { value: "off", label: t("timerOff") },
                ...[10, 20, 30].map((n) => ({
                  value: String(n),
                  label: t("timerMinutes", { n }),
                })),
              ]}
            />
            {minutes !== "off" && (
              <Button
                size="sm"
                variant="primary"
                label={t("timerStart")}
                isDisabled={!loaded || working || busy || status !== "active"}
                onClick={() => void timing("start")}
              />
            )}
          </>
        ) : (
          <>
            <span
              role="timer"
              className={`timer-clock ${left === 0 ? "urgent" : ""}`}
              aria-label={t("timeLeft", { t: clockText(left) })}
            >
              {clockText(left)} / {clockText(clock.budgetMs)}
            </span>
            <span>
              {clock.endedAt !== null
                ? t("mockFinished")
                : left === 0
                  ? t("timeUp")
                  : clock.deadlineMs === null
                    ? t("mockPaused")
                    : t("mockRunning")}
            </span>
            {clock.endedAt === null && status === "active" && left > 0 && (
              <Button
                size="sm"
                variant="ghost"
                label={t(
                  clock.deadlineMs === null ? "timerResume" : "timerPause",
                )}
                isDisabled={working}
                onClick={() =>
                  void timing(clock.deadlineMs === null ? "resume" : "pause")
                }
              />
            )}
            {clock.endedAt === null && status === "active" && (
              <Button
                size="sm"
                variant="ghost"
                label={t("timerReset")}
                isDisabled={working || busy}
                onClick={() => void timing("reset")}
              />
            )}
          </>
        )}
        <span className="muted">
          {t("mockProgress", { n: turns })}
          {stage
            ? ` · ${t(`sdStage_${stage}` as Parameters<typeof t>[0])}`
            : ""}
        </span>
        <span className="spacer" />
        <Button
          size="sm"
          label={t("exportMarkdown")}
          isDisabled={busy || working}
          onClick={() => void exportFile("md")}
        />
        <Button
          size="sm"
          variant="ghost"
          label={t("exportJson")}
          isDisabled={busy || working}
          onClick={() => void exportFile("json")}
        />
      </div>
      {clock && clock.endedAt === null && left === 0 && (
        <Banner
          status="warning"
          title={t("timeUp")}
          description={t("mockExpired")}
        />
      )}
      <small className="muted">{t("exportScope")}</small>
      {error && <Banner status="error" title={localizeError(error, lang)} />}
      {notice && <p role="status">{notice}</p>}
      <UsagePanel
        module={module}
        sessionId={sessionId}
        provider={provider}
        model={model}
        refreshKey={`${status}-${turns}-${busy}`}
      />
    </section>
  );
}
export function UsagePanel({
  module,
  sessionId,
  provider,
  model,
  refreshKey,
}: {
  module: "p1" | "p2";
  sessionId?: string;
  provider: string;
  model: string;
  refreshKey: string;
}) {
  const { t, lang } = useI18n();
  const [scope, setScope] = useState(sessionId ? "session" : "all"),
    [data, setData] = useState<Accounting | null>(null),
    [error, setError] = useState(""),
    [working, setWorking] = useState(false),
    [rates, setRates] = useState<Rates>({
      input: 0,
      output: 0,
      cacheRead: 0,
      cacheWrite: 0,
    }),
    [saved, setSaved] = useState(false),
    [dirtyRates, setDirtyRates] = useState(false);
  async function load() {
    const next = await invoke<Accounting>("api_usage", {
      module: scope === "session" ? module : null,
      sessionId: scope === "session" ? sessionId : null,
    });
    setData(next);
  }
  useEffect(() => {
    let active = true;
    if (!isTauri()) return;
    invoke<Accounting>("api_usage", {
      module: scope === "session" ? module : null,
      sessionId: scope === "session" ? sessionId : null,
    })
      .then((next) => {
        if (active) setData(next);
      })
      .catch((e) => {
        if (active) setError(String(e));
      });
    return () => {
      active = false;
    };
  }, [scope, sessionId, module, refreshKey]);
  useEffect(() => {
    setDirtyRates(false);
    setSaved(false);
  }, [provider, model]);
  useEffect(() => {
    const found = data?.prices.find(
      (p) => p.provider === provider && p.model === model,
    );
    if (!dirtyRates)
      setRates(
        found?.rates ?? { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 },
      );
  }, [data?.prices, provider, model, dirtyRates]);
  const rows = data?.records ?? [],
    unknown = rows.filter((r) => r.estimatedUsd === null).length;
  const sum = rows.reduce((n, r) => n + (r.estimatedUsd ?? 0), 0);
  const usd = (n: number) =>
    new Intl.NumberFormat(lang === "zh" ? "zh-CN" : "en-US", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 6,
    }).format(n);
  return (
    <Collapsible
      trigger={<strong>{t("usageTitle")}</strong>}
      defaultIsOpen={false}
    >
      <p className="muted">{t("usageExplanation")}</p>
      <div className="actions">
        {sessionId && (
          <Selector
            size="sm"
            label={t("usageScope")}
            value={scope}
            onChange={setScope}
            options={[
              { value: "session", label: t("usageSession") },
              { value: "all", label: t("usageAll") },
            ]}
          />
        )}
        <Button
          size="sm"
          variant="ghost"
          label={t("usageRefresh")}
          isDisabled={working}
          onClick={() => void load().catch((e) => setError(String(e)))}
        />
      </div>
      <p>{t("usageTotals", { n: rows.length, cost: usd(sum), unknown })}</p>
      <div className="usage-table-wrap">
        <table className="usage-table">
          <thead>
            <tr>
              {[
                "usageWhen",
                "usageModel",
                "usageTask",
                "usageTokens",
                "usageEstimate",
                "usageStatus",
              ].map((k) => (
                <th key={k}>{t(k as Parameters<typeof t>[0])}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.slice(0, 100).map((r) => (
              <tr key={r.id}>
                <td>
                  {new Date(r.createdAt).toLocaleString(
                    lang === "zh" ? "zh-CN" : "en",
                  )}
                </td>
                <td>
                  {r.module.toUpperCase()} · {r.provider}
                  <br />
                  {r.model}
                </td>
                <td>
                  {t(`usageOp_${r.operation}` as Parameters<typeof t>[0])}
                  {r.reviewer ? ` · ${t("usageReview")}` : ""}
                </td>
                <td>
                  {r.inputTokens ?? t("usageUnknown")} /{" "}
                  {r.outputTokens ?? t("usageUnknown")}
                  <br />
                  <small>
                    {t("usageCache")}: {r.cacheReadTokens ?? t("usageUnknown")}{" "}
                    / {r.cacheWriteTokens ?? t("usageUnknown")}
                  </small>
                </td>
                <td>
                  {r.estimatedUsd === null
                    ? t("usageUnknown")
                    : usd(r.estimatedUsd)}
                </td>
                <td>
                  {t(`usageState_${r.status}` as Parameters<typeof t>[0])}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {rows.length === 0 && <p>{t("usageEmpty")}</p>}
      {rows.length > 100 && <p>{t("usageRecent")}</p>}
      <Collapsible
        trigger={
          <span>
            {t("usagePrices")} · {provider} / {model}
          </span>
        }
        defaultIsOpen={false}
      >
        <p className="muted">{t("usagePriceHelp")}</p>
        <div className="usage-rates">
          {(["input", "output", "cacheRead", "cacheWrite"] as const).map(
            (key) => (
              <NumberInput
                key={key}
                label={t(`usageRate_${key}` as Parameters<typeof t>[0])}
                value={rates[key]}
                min={0}
                max={1000000}
                step={0.01}
                onChange={(v) => {
                  setSaved(false);
                  setDirtyRates(true);
                  setRates({ ...rates, [key]: v ?? 0 });
                }}
              />
            ),
          )}
        </div>
        <Button
          label={t("usageSavePrices")}
          size="sm"
          isDisabled={working}
          onClick={async () => {
            setWorking(true);
            setError("");
            try {
              await invoke("save_api_prices", { provider, model, rates });
              await load();
              setDirtyRates(false);
              setSaved(true);
            } catch (e) {
              setError(String(e));
            } finally {
              setWorking(false);
            }
          }}
        />
        {saved && <p role="status">{t("usagePricesSaved")}</p>}
      </Collapsible>
      {error && <Banner status="error" title={localizeError(error, lang)} />}
    </Collapsible>
  );
}
