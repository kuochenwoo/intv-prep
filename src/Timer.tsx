import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Selector } from "@astryxdesign/core/Selector";
import { Button } from "@astryxdesign/core/Button";
import { Badge } from "@astryxdesign/core/Badge";
import { useI18n } from "./i18n";
import { localizeError } from "./errors";

const MINUTES = [10, 20, 30];
type CodingClock = {
  budgetMs: number;
  remainingMs: number;
  deadlineMs: number | null;
  startedAt: number;
  endedAt: number | null;
};
const clockText = (ms: number) => {
  const s = Math.ceil(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

/** Saved per problem, shared across coding modes. Selecting a duration never starts it. */
export function Timer({ problemId }: { problemId: string }) {
  const { t, lang } = useI18n();
  const [minutes, setMinutes] = useState("off"),
    [clock, setClock] = useState<CodingClock | null>(null),
    [loaded, setLoaded] = useState(false),
    [reload, setReload] = useState(0),
    [working, setWorking] = useState(false),
    [error, setError] = useState(""),
    [now, setNow] = useState(Date.now());

  useEffect(() => {
    let active = true;
    if (!isTauri()) return;
    setLoaded(false);
    setClock(null);
    setMinutes("off");
    setError("");
    invoke<CodingClock | null>("coding_timing", { problemId, action: "get" })
      .then((saved) => {
        if (!active) return;
        setClock(saved);
        if (saved) setMinutes(String(saved.budgetMs / 60000));
        setNow(Date.now());
        setLoaded(true);
      })
      .catch((e) => {
        if (active) setError(String(e));
      });
    return () => {
      active = false;
    };
  }, [problemId, reload]);

  const left = clock
    ? clock.deadlineMs === null
      ? clock.remainingMs
      : Math.max(0, clock.deadlineMs - now)
    : minutes === "off"
      ? 0
      : Number(minutes) * 60000;
  const running = clock?.deadlineMs != null && left > 0;
  useEffect(() => {
    if (!running) return;
    const id = setInterval(() => setNow(Date.now()), 250);
    return () => clearInterval(id);
  }, [running]);

  async function timing(action: string, nextMinutes?: string) {
    setWorking(true);
    setError("");
    try {
      const saved = await invoke<CodingClock | null>("coding_timing", {
        problemId,
        action,
        minutes: Number(minutes),
      });
      setClock(saved);
      setNow(Date.now());
      if (nextMinutes !== undefined) setMinutes(nextMinutes);
    } catch (e) {
      setError(String(e));
    } finally {
      setWorking(false);
    }
  }
  function choose(value: string) {
    if (clock) void timing("reset", value);
    else setMinutes(value);
  }

  return (
    <div>
      <div className="timer" title={t("codingTimerHint")}>
        <Selector
          label={t("timer")}
          isLabelHidden
          size="sm"
          value={minutes}
          isDisabled={!loaded || working}
          onChange={choose}
          options={[
            { value: "off", label: t("timerOff") },
            ...MINUTES.map((n) => ({
              value: String(n),
              label: t("timerMinutes", { n }),
            })),
          ]}
        />
        {minutes !== "off" &&
          (left === 0 ? (
            <Badge variant="red" label={t("timeUp")} />
          ) : (
            <span
              className={`timer-clock ${left <= 60_000 ? "urgent" : ""} ${clock && clock.deadlineMs === null ? "paused" : ""}`}
              role="timer"
              aria-label={t("timeLeft", { t: clockText(left) })}
            >
              {clockText(left)}
            </span>
          ))}
        {minutes !== "off" && left > 0 && (
          <Button
            size="sm"
            variant={clock ? "ghost" : "primary"}
            isDisabled={!loaded || working}
            label={
              !clock
                ? t("timerStart")
                : clock.deadlineMs === null
                  ? t("timerResume")
                  : t("timerPause")
            }
            onClick={() =>
              void timing(
                !clock
                  ? "start"
                  : clock.deadlineMs === null
                    ? "resume"
                    : "pause",
              )
            }
          />
        )}
        {clock && (
          <Button
            size="sm"
            variant="ghost"
            label={t("timerReset")}
            isDisabled={working}
            onClick={() => void timing("reset")}
          />
        )}
      </div>
      {error && !loaded && (
        <Button
          size="sm"
          variant="secondary"
          label={t("retryLoadTimer")}
          isDisabled={working}
          onClick={() => setReload((n) => n + 1)}
        />
      )}
      {error && <p role="alert">{localizeError(error, lang)}</p>}
    </div>
  );
}
