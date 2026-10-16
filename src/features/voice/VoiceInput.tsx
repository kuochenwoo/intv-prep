import { registerUpdateGuard } from "../updates/guards";
import { useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Button } from "@astryxdesign/core/Button";
import { IconButton } from "@astryxdesign/core/IconButton";
import { TextArea } from "@astryxdesign/core/TextArea";
import { Banner } from "@astryxdesign/core/Banner";
import { useI18n } from "../../i18n";
import { localizeError } from "../../errors";
import "./voice.css";
import { appendTranscript } from "./draft";

type Phase =
  "idle" | "permission" | "recording" | "stopping" | "ready" | "transcribing";
type Clip = { blob: Blob; mime: string };
const MAX_BYTES = 20 * 1024 * 1024;
const MAX_MS = 5 * 60_000;
const displayTime = (ms: number) => {
  const s = Math.floor(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

/** Audio and the editable preview stay in memory; only explicit insertion saves text. */
export function VoiceInput({
  module,
  sessionId,
  visible,
  disabled,
  draft,
  onDraft,
  onBusyChange,
}: {
  module: "p1" | "p2";
  sessionId: string;
  visible: boolean;
  disabled: boolean;
  draft: string;
  onDraft: (text: string) => void;
  onBusyChange: (busy: boolean) => void;
}) {
  const { t, lang } = useI18n();
  const [phase, setPhase] = useState<Phase>("idle");
  const [clip, setClip] = useState<Clip | null>(null);
  const [playback, setPlayback] = useState("");
  const [transcript, setTranscript] = useState("");
  const [transcribed, setTranscribed] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [usageWarning, setUsageWarning] = useState(false);
  const [elapsed, setElapsed] = useState(0);
  const [online, setOnline] = useState(navigator.onLine);
  const recorder = useRef<MediaRecorder | null>(null);
  const stream = useRef<MediaStream | null>(null);
  const mounted = useRef(true);
  const generation = useRef(0);
  const operation = useRef(false);
  const context = useRef({ visible, disabled });
  context.current = { visible, disabled };
  const pending = [
    "permission",
    "recording",
    "stopping",
    "transcribing",
  ].includes(phase);

  useEffect(() => registerUpdateGuard(() => {
    if (pending || clip || transcript.trim()) throw new Error("updateVoicePending");
  }), [pending, clip, transcript]);

  function releaseMicrophone() {
    stream.current?.getTracks().forEach((track) => track.stop());
    stream.current = null;
  }
  function stop() {
    const active = recorder.current;
    if (active && active.state !== "inactive") {
      setPhase("stopping");
      active.stop();
    }
    releaseMicrophone();
  }
  useEffect(() => {
    onBusyChange(pending);
  }, [pending, onBusyChange]);
  useEffect(() => {
    mounted.current = true;
    const update = () => setOnline(navigator.onLine);
    window.addEventListener("online", update);
    window.addEventListener("offline", update);
    return () => {
      mounted.current = false;
      generation.current++;
      const active = recorder.current;
      if (active && active.state !== "inactive") active.stop();
      releaseMicrophone();
      onBusyChange(false);
      window.removeEventListener("online", update);
      window.removeEventListener("offline", update);
    };
  }, [onBusyChange]);
  useEffect(() => {
    if (!visible || disabled) stop();
  }, [visible, disabled]);
  useEffect(() => {
    if (!clip) {
      setPlayback("");
      return;
    }
    const url = URL.createObjectURL(clip.blob);
    setPlayback(url);
    return () => URL.revokeObjectURL(url);
  }, [clip]);

  async function record() {
    if (operation.current || !visible || disabled) return;
    if (
      !isTauri() ||
      !navigator.mediaDevices?.getUserMedia ||
      typeof MediaRecorder === "undefined"
    ) {
      setError(t("voiceUnsupported"));
      return;
    }
    const mime = [
      "audio/webm;codecs=opus",
      "audio/mp4",
      "audio/ogg;codecs=opus",
      "audio/webm",
    ].find((x) => MediaRecorder.isTypeSupported(x));
    if (!mime) {
      setError(t("voiceUnsupported"));
      return;
    }
    operation.current = true;
    const current = ++generation.current;
    setError("");
    setNotice("");
    setUsageWarning(false);
    setPhase("permission");
    try {
      const media = await navigator.mediaDevices.getUserMedia({
        audio: true,
        video: false,
      });
      if (
        !mounted.current ||
        generation.current !== current ||
        !context.current.visible ||
        context.current.disabled
      ) {
        media.getTracks().forEach((track) => track.stop());
        if (mounted.current && generation.current === current)
          setPhase(clip ? "ready" : "idle");
        operation.current = false;
        return;
      }
      stream.current = media;
      const active = new MediaRecorder(media, { mimeType: mime });
      recorder.current = active;
      const chunks: Blob[] = [];
      let size = 0,
        failed = false;
      const started = Date.now();
      const ticker = window.setInterval(() => {
        if (mounted.current && generation.current === current)
          setElapsed(Date.now() - started);
      }, 250);
      const limit = window.setTimeout(() => {
        if (mounted.current && generation.current === current) {
          setNotice(t("voiceLimit"));
          stop();
        }
      }, MAX_MS);
      const clearTimers = () => {
        clearInterval(ticker);
        clearTimeout(limit);
      };
      active.ondataavailable = (event) => {
        size += event.data.size;
        if (size > MAX_BYTES) {
          failed = true;
          if (mounted.current && generation.current === current)
            setError(t("voiceTooLarge"));
          if (active.state !== "inactive") active.stop();
          releaseMicrophone();
        } else if (event.data.size) chunks.push(event.data);
      };
      active.onerror = () => {
        failed = true;
        clearTimers();
        releaseMicrophone();
        if (active.state !== "inactive") active.stop();
        if (mounted.current && generation.current === current) {
          setPhase("stopping");
          setError(t("voiceRecordFailed"));
        }
      };
      active.onstop = () => {
        clearTimers();
        releaseMicrophone();
        if (!mounted.current || generation.current !== current) return;
        recorder.current = null;
        operation.current = false;
        const blob = new Blob(chunks, { type: active.mimeType || mime });
        setElapsed(Math.min(MAX_MS, Date.now() - started));
        if (failed || blob.size === 0) {
          setClip(null);
          setPhase("idle");
          if (!failed) setError(t("voiceEmpty"));
          return;
        }
        setClip({ blob, mime: (active.mimeType || mime).split(";")[0] });
        setPhase("ready");
      };
      // Drop the prior temporary preview only after microphone access succeeds.
      try {
        active.start(1000);
      } catch (e) {
        clearTimers();
        throw e;
      }
      setClip(null);
      setTranscript("");
      setTranscribed(false);
      setElapsed(0);
      setPhase("recording");
    } catch (e) {
      releaseMicrophone();
      recorder.current = null;
      operation.current = false;
      if (!mounted.current || generation.current !== current) return;
      setPhase(clip ? "ready" : "idle");
      const name = e instanceof DOMException ? e.name : "";
      setError(
        t(
          name === "NotAllowedError"
            ? "voicePermission"
            : name === "NotFoundError"
              ? "voiceNoDevice"
              : name === "NotReadableError"
                ? "voiceDeviceBusy"
                : "voiceRecordFailed",
        ),
      );
    }
  }
  async function transcribe() {
    if (!clip || operation.current || !online || !visible || disabled) return;
    operation.current = true;
    const current = generation.current;
    setPhase("transcribing");
    setError("");
    setNotice("");
    try {
      const audio = Array.from(new Uint8Array(await clip.blob.arrayBuffer()));
      if (
        !mounted.current ||
        generation.current !== current ||
        !context.current.visible
      )
        return;
      const result = await invoke<{ text: string; usageWarning: boolean }>(
        "transcribe_audio",
        {
          module,
          sessionId,
          audio,
          mime: clip.mime,
          lang,
        },
      );
      if (!mounted.current || generation.current !== current) return;
      setTranscript(result.text);
      setTranscribed(true);
      setUsageWarning(result.usageWarning);
    } catch (e) {
      if (mounted.current && generation.current === current)
        setError(localizeError(String(e), lang));
    } finally {
      if (mounted.current && generation.current === current) {
        operation.current = false;
        setPhase("ready");
      }
    }
  }
  const combined = appendTranscript(draft, transcript);
  const tooLong = !!transcript.trim() && combined === null;
  return (
    <section className="voice-input" aria-label={t("voiceTitle")}>
      <div className="voice-toolbar">
        {pending && (
          <span role="status" className="muted">
            {phase === "permission"
              ? t("voiceRequesting")
              : phase === "transcribing"
                ? t("voiceTranscribing")
                : phase === "stopping"
                  ? t("voiceStopping")
                  : t("voiceRecording", { time: displayTime(elapsed) })}
          </span>
        )}
        <IconButton
          size="sm"
          variant={phase === "recording" ? "primary" : "ghost"}
          className="voice-trigger"
          label={
            phase === "recording"
              ? t("voiceStop")
              : clip || transcribed
                ? t("voiceRerecord")
                : t("voiceRecord")
          }
          tooltip={
            phase === "recording" ? t("voiceStop") : t("voiceTriggerHint")
          }
          aria-pressed={phase === "recording"}
          isLoading={pending && phase !== "recording"}
          isDisabled={
            (pending && phase !== "recording") ||
            disabled ||
            !visible ||
            !isTauri()
          }
          icon={
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.8"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              {phase === "recording" ? (
                <rect x="6" y="6" width="12" height="12" rx="2" />
              ) : (
                <>
                  <rect x="9" y="2" width="6" height="12" rx="3" />
                  <path d="M5 10v2a7 7 0 0 0 14 0v-2M12 19v3M8 22h8" />
                </>
              )}
            </svg>
          }
          onClick={() => (phase === "recording" ? stop() : void record())}
        />
      </div>
      {(pending || clip || transcribed || error || usageWarning) && (
        <div className="voice-panel">
          <div className="voice-controls">
            {clip && !pending && !transcribed && (
              <Button
                size="sm"
                variant="primary"
                label={t("voiceTranscribe")}
                isDisabled={disabled || !online || !visible}
                onClick={() => void transcribe()}
              />
            )}
            {(clip || transcribed) && !pending && (
              <Button
                size="sm"
                variant="ghost"
                label={t("voiceDiscard")}
                isDisabled={disabled}
                onClick={() => {
                  generation.current++;
                  setClip(null);
                  setTranscript("");
                  setTranscribed(false);
                  setError("");
                  setNotice("");
                  setUsageWarning(false);
                  setPhase("idle");
                }}
              />
            )}
          </div>
          <p className="muted">{t("voicePrivacy")}</p>
          {playback && (
            <audio
              controls
              preload="metadata"
              src={playback}
              aria-label={t("voicePlayback")}
            />
          )}
          {!online && clip && !transcribed && (
            <p className="muted">{t("voiceOffline")}</p>
          )}
          {error && <Banner status="error" title={error} />}
          {usageWarning && (
            <Banner status="warning" title={t("voiceUsageWarning")} />
          )}
          {transcribed && (
            <>
              <TextArea
                label={t("voiceTranscript")}
                description={t("voiceEditHint")}
                value={transcript}
                rows={5}
                maxLength={20000}
                width="100%"
                isDisabled={disabled}
                onChange={setTranscript}
              />
              {tooLong && (
                <p role="status" className="muted">
                  {t("voiceDraftLimit")}
                </p>
              )}
              <Button
                size="sm"
                variant="primary"
                label={t("voiceInsert")}
                isDisabled={
                  disabled ||
                  pending ||
                  !visible ||
                  !transcript.trim() ||
                  tooLong
                }
                onClick={() => {
                  if (combined === null) return;
                  onDraft(combined);
                  setClip(null);
                  setTranscript("");
                  setTranscribed(false);
                  setPhase("idle");
                  setNotice(t("voiceInserted"));
                }}
              />
            </>
          )}
          <small className="muted">{t("voiceTemporary")}</small>
        </div>
      )}
      {notice && (
        <p role="status" className="muted voice-notice">
          {notice}
        </p>
      )}
    </section>
  );
}
