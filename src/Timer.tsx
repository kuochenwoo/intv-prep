import { useEffect, useState } from "react";
import { Selector } from "@astryxdesign/core/Selector";
import { Button } from "@astryxdesign/core/Button";
import { Badge } from "@astryxdesign/core/Badge";
import { useI18n } from "./i18n";

const MINUTES = [10, 20, 30, 40, 60];

const clock = (ms: number) => {
  const s = Math.ceil(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

/** Optional countdown for a coding attempt. Off by default; a picked length waits for Start. */
export function Timer() {
  const { t } = useI18n();
  const [minutes, setMinutes] = useState("off"),
    // While running: the moment time runs out. While paused: the time left.
    [deadline, setDeadline] = useState(0),
    [pausedLeft, setPausedLeft] = useState<number | null>(null),
    [started, setStarted] = useState(false),
    [now, setNow] = useState(Date.now());
  const on = minutes !== "off";
  const left = pausedLeft ?? Math.max(0, deadline - now);
  const running = on && pausedLeft === null && left > 0;

  useEffect(() => {
    if (!running) return;
    const id = setInterval(() => setNow(Date.now()), 250);
    return () => clearInterval(id);
  }, [running]);

  // Picking a length (or Reset) shows the full time, held until Start.
  function choose(value: string) {
    setMinutes(value);
    setStarted(false);
    setPausedLeft(value === "off" ? null : Number(value) * 60_000);
  }
  function pauseOrResume() {
    if (pausedLeft === null) setPausedLeft(left);
    else {
      const at = Date.now();
      setNow(at);
      setDeadline(at + pausedLeft);
      setPausedLeft(null);
    }
  }

  return (
    <div className="timer">
      <Selector
        label={t("timer")}
        isLabelHidden
        size="sm"
        value={minutes}
        onChange={choose}
        options={[
          { value: "off", label: t("timerOff") },
          ...MINUTES.map((n) => ({
            value: String(n),
            label: t("timerMinutes", { n }),
          })),
        ]}
      />
      {on &&
        (left === 0 ? (
          <Badge variant="red" label={t("timeUp")} />
        ) : (
          <span
            className={`timer-clock ${left <= 60_000 ? "urgent" : ""} ${started && pausedLeft !== null ? "paused" : ""}`}
            role="timer"
            aria-label={t("timeLeft", { t: clock(left) })}
          >
            {clock(left)}
          </span>
        ))}
      {on && left > 0 && (
        <Button
          size="sm"
          variant={started ? "ghost" : "primary"}
          label={
            !started
              ? t("timerStart")
              : pausedLeft === null
                ? t("timerPause")
                : t("timerResume")
          }
          onClick={() => {
            setStarted(true);
            pauseOrResume();
          }}
        />
      )}
      {on && started && (
        <Button
          size="sm"
          variant="ghost"
          label={t("timerReset")}
          onClick={() => choose(minutes)}
        />
      )}
    </div>
  );
}
