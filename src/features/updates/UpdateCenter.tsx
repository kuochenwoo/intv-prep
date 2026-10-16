import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { Button } from "@astryxdesign/core/Button";
import { Switch } from "@astryxdesign/core/Switch";
import { Banner } from "@astryxdesign/core/Banner";
import { AlertDialog } from "@astryxdesign/core/AlertDialog";
import { Dialog } from "@astryxdesign/core/Dialog";
import { Markdown } from "../../Markdown";
import { useI18n, type Key } from "../../i18n";
import { localizeError } from "../../errors";
import { prepareFrontendUpdate } from "./guards";
import { UPDATE_CHECK_INTERVAL_MS, downloadPercent } from "./policy";

type Info = { version: string; enabled: boolean; autoCheckAllowed: boolean };
type Phase =
  | "idle"
  | "checking"
  | "available"
  | "current"
  | "error"
  | "downloading"
  | "installing"
  | "restarting"
  | "restartRequired";
function storedAutomaticChecks() {
  try {
    return localStorage.getItem("automaticUpdateChecks") !== "false";
  } catch {
    return true;
  }
}
function useUpdateState() {
  const [info, setInfo] = useState<Info | null>(null);
  const [automatic, setAutomatic] = useState(storedAutomaticChecks);
  const [phase, setPhase] = useState<Phase>("idle");
  const [update, setUpdate] = useState<Update | null>(null);
  const [error, setError] = useState("");
  const [showGlobalError, setShowGlobalError] = useState(false);
  const [percent, setPercent] = useState<number | undefined>();
  const [lastChecked, setLastChecked] = useState<number | null>(null);
  const [confirm, setConfirm] = useState(false);
  const [dismissed, setDismissed] = useState(false);
  const active = useRef(true),
    operation = useRef(false),
    installedUpdate = useRef(false),
    resource = useRef<Update | null>(null);
  const busy = ["downloading", "installing", "restarting"].includes(phase);
  useEffect(() => {
    active.current = true;
    if (isTauri())
      invoke<Info>("app_update_info")
        .then((value) => {
          if (active.current) setInfo(value);
        })
        .catch(() => {
          if (active.current) setError("updateInfoFailed");
        });
    return () => {
      active.current = false;
      void resource.current?.close().catch(() => {});
    };
  }, []);
  const checkNow = useCallback(async () => {
    if (!info?.enabled || operation.current || installedUpdate.current) return;
    operation.current = true;
    setPhase("checking");
    setError("");
    setShowGlobalError(false);
    try {
      await resource.current?.close();
      resource.current = null;
      setUpdate(null);
      const next = await check({ timeout: 15_000 });
      if (!active.current) {
        await next?.close();
        return;
      }
      resource.current = next;
      setUpdate(next);
      setDismissed(false);
      setLastChecked(Date.now());
      setPhase(next ? "available" : "current");
    } catch {
      // A missing/unpublished feed or network failure must never look like "up to date".
      if (active.current) {
        setError("updateCheckFailed");
        setPhase("error");
      }
    } finally {
      operation.current = false;
    }
  }, [info?.enabled]);
  useEffect(() => {
    if (!automatic || !info?.autoCheckAllowed) return;
    const startup = window.setTimeout(() => void checkNow(), 5_000);
    const periodic = window.setInterval(
      () => void checkNow(),
      UPDATE_CHECK_INTERVAL_MS,
    );
    return () => {
      window.clearTimeout(startup);
      window.clearInterval(periodic);
    };
  }, [automatic, info?.autoCheckAllowed, checkNow]);
  function changeAutomatic(value: boolean) {
    setAutomatic(value);
    try {
      localStorage.setItem("automaticUpdateChecks", String(value));
    } catch {
      /* Session-only preference. */
    }
  }
  async function restart() {
    if (operation.current) return;
    operation.current = true;
    setError("");
    setPhase("restarting");
    try {
      await relaunch();
    } catch {
      setShowGlobalError(true);
      setError("updateRestartFailed");
      setPhase("restartRequired");
    } finally {
      operation.current = false;
    }
  }
  async function install() {
    const next = resource.current;
    if (!next || operation.current) return;
    operation.current = true;
    setConfirm(false);
    setError("");
    setPercent(undefined);
    setPhase("downloading");
    let installed = false;
    try {
      await prepareFrontendUpdate();
      await invoke("prepare_app_update");
      let received = 0,
        total: number | undefined;
      await next.download(
        (event) => {
          if (event.event === "Started") total = event.data.contentLength;
          if (event.event === "Progress") received += event.data.chunkLength;
          setPercent(downloadPercent(received, total));
        },
        { timeout: 300_000 },
      );
      // Signature verification finishes in download(); do not install before it succeeds.
      await prepareFrontendUpdate();
      await invoke("prepare_app_update");
      setPhase("installing");
      await next.install();
      installed = true;
      installedUpdate.current = true;
      resource.current = null;
      setUpdate(null);
      setPhase("restarting");
      await relaunch();
    } catch (e) {
      setShowGlobalError(true);
      const message = e instanceof Error ? e.message : String(e);
      const known = [
        "updatePracticeBusy",
        "updateDraftFailed",
        "updateVoicePending",
        "当前运行环境不支持应用内更新，请安装正式应用包",
        "请等待判题、面试请求或语音转写完成后再更新",
        "无法保存更新前的数据，请稍后重试",
      ];
      setError(
        installed
          ? "updateRestartFailed"
          : known.includes(message)
            ? message
            : "updateInstallFailed",
      );
      setPhase(installed ? "restartRequired" : "error");
    } finally {
      operation.current = false;
    }
  }
  return {
    info,
    automatic,
    phase,
    update,
    error,
    showGlobalError,
    percent,
    lastChecked,
    confirm,
    dismissed,
    busy,
    changeAutomatic,
    checkNow,
    install,
    restart,
    setConfirm,
    setDismissed,
  };
}
const UpdateContext = createContext<ReturnType<typeof useUpdateState> | null>(
  null,
);
export function UpdateProvider({ children }: { children: ReactNode }) {
  const state = useUpdateState();
  return (
    <UpdateContext.Provider value={state}>{children}</UpdateContext.Provider>
  );
}
function useUpdates() {
  const value = useContext(UpdateContext);
  if (!value) throw Error("UpdateProvider is required");
  return value;
}
const phaseText: Record<Phase, Key> = {
  idle: "updateIdle",
  checking: "updateChecking",
  available: "updateAvailable",
  current: "updateCurrent",
  error: "updateFailed",
  downloading: "updateDownloading",
  installing: "updateInstalling",
  restarting: "updateRestarting",
  restartRequired: "updateRestartRequired",
};
function UpdateError({ error }: { error: string }) {
  const { t, lang } = useI18n();
  const known: Record<string, Key> = {
    updateInfoFailed: "updateInfoFailed",
    updateCheckFailed: "updateCheckFailed",
    updateRestartFailed: "updateRestartFailed",
    updateInstallFailed: "updateInstallFailed",
    updatePracticeBusy: "updatePracticeBusy",
    updateDraftFailed: "updateDraftFailed",
    updateVoicePending: "updateVoicePending",
  };
  return (
    <Banner
      status="warning"
      title={known[error] ? t(known[error]) : localizeError(error, lang)}
    />
  );
}
export function UpdateNotice() {
  const u = useUpdates(),
    { t } = useI18n();
  return (
    <>
      {u.update && !u.dismissed && !u.busy && (
        <div className="update-notice">
          <Banner
            status="info"
            title={t("updateFound", { v: u.update.version })}
            isDismissable
            onDismiss={() => u.setDismissed(true)}
          />
          <Button
            label={t("updateInstall")}
            size="sm"
            onClick={() => u.setConfirm(true)}
          />
        </div>
      )}
      {u.phase === "restartRequired" && (
        <div className="update-notice">
          <Banner status="warning" title={t("updateRestartRequired")} />
          <Button label={t("updateRestart")} onClick={() => void u.restart()} />
        </div>
      )}
      {u.error && u.showGlobalError && !u.busy && (
        <div className="update-notice">
          <UpdateError error={u.error} />
        </div>
      )}
      <AlertDialog
        isOpen={u.confirm}
        onOpenChange={u.setConfirm}
        title={t("updateConfirmTitle", { v: u.update?.version ?? "" })}
        description={t("updateConfirmNote")}
        actionVariant="primary"
        actionLabel={t("updateInstall")}
        cancelLabel={t("cancel")}
        onAction={() => void u.install()}
      />
      <Dialog
        isOpen={u.busy}
        purpose="required"
        onOpenChange={() => {}}
        aria-label={t(phaseText[u.phase])}
      >
        <div className="update-progress" role="status" aria-live="polite">
          <h2>{t(phaseText[u.phase])}</h2>
          {u.phase === "downloading" && (
            <progress
              max={100}
              value={u.percent}
              aria-label={t("updateDownloading")}
            />
          )}
          <p>
            {t("updateKeepOpen")}
            {u.phase === "downloading" && u.percent !== undefined
              ? ` ${u.percent}%`
              : ""}
          </p>
        </div>
      </Dialog>
    </>
  );
}
export function UpdateSettings() {
  const u = useUpdates(),
    { t, lang } = useI18n();
  return (
    <section className="card settings-accounting">
      <h2>{t("updateTitle")}</h2>
      <p>{t("updateVersion", { v: u.info?.version ?? "—" })}</p>
      <Switch
        label={t("updateAutomatic")}
        description={t("updateAutomaticNote")}
        value={u.automatic}
        onChange={u.changeAutomatic}
        isDisabled={!u.info?.enabled || u.busy}
      />
      <div className="actions">
        <Button
          label={t("updateCheck")}
          onClick={() => void u.checkNow()}
          isLoading={u.phase === "checking"}
          isDisabled={
            !u.info?.enabled ||
            u.busy ||
            u.phase === "checking" ||
            u.phase === "restartRequired"
          }
        />
        {u.update && (
          <Button
            label={t("updateInstall")}
            variant="primary"
            onClick={() => u.setConfirm(true)}
            isDisabled={u.busy}
          />
        )}
        {u.phase === "restartRequired" && (
          <Button label={t("updateRestart")} onClick={() => void u.restart()} />
        )}
      </div>
      <p role="status">{t(phaseText[u.phase])}</p>
      {u.lastChecked && (
        <p className="muted">
          {t("updateLastChecked", {
            time: new Date(u.lastChecked).toLocaleString(
              lang === "zh" ? "zh-CN" : "en",
            ),
          })}
        </p>
      )}
      {!u.info?.enabled && <p className="muted">{t("updateInstalledOnly")}</p>}
      {u.error && <UpdateError error={u.error} />}
      {u.update?.body && (
        <div className="update-release-notes">
          <h3>{t("updateReleaseNotes")}</h3>
          <Markdown text={u.update.body} />
        </div>
      )}
    </section>
  );
}
