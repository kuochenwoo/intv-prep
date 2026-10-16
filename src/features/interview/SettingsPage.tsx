import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Button } from "@astryxdesign/core/Button";
import { TextInput } from "@astryxdesign/core/TextInput";
import { NumberInput } from "@astryxdesign/core/NumberInput";
import { Selector } from "@astryxdesign/core/Selector";
import { Collapsible } from "@astryxdesign/core/Collapsible";
import { AlertDialog } from "@astryxdesign/core/AlertDialog";
import { Banner } from "@astryxdesign/core/Banner";
import { useI18n } from "../../i18n";
import { localizeError } from "../../errors";
import { UsagePanel } from "./SessionTools";
import { ApiKeyBadge, ApiKeyWarning } from "../../ApiKeyStatus";
import { UpdateSettings } from "../updates/UpdateCenter";
import { VoiceSettings } from "../voice/VoiceSettings";
type Config = { provider: string; model: string; timeoutSeconds: number };
type Summary = { id: string; title: string; provider?: string; model?: string };
type Overview = {
  settings: { config: Config; keyStatus: string; credentialPath: string };
  sessions: Summary[];
};
export default function SettingsPage({
  active,
  onSaved,
}: {
  active: boolean;
  onSaved: () => void;
}) {
  const { t, lang } = useI18n();
  const [data, setData] = useState<Overview | null>(null),
    [design, setDesign] = useState<Summary[]>([]),
    [config, setConfig] = useState<Config>({
      provider: "claude",
      model: "claude-haiku-5-5",
      timeoutSeconds: 60,
    }),
    [apiKey, setApiKey] = useState("");
  const [busy, setBusy] = useState(false),
    [error, setError] = useState(""),
    [notice, setNotice] = useState(""),
    [confirm, setConfirm] = useState<"key" | null>(null),
    [scope, setScope] = useState("all"),
    [revision, setRevision] = useState(0),
    [compiler, setCompiler] = useState("");
  async function load() {
    const [p1, p2, compiler] = await Promise.all([
      invoke<Overview>("interview_overview"),
      invoke<{ sessions: Summary[] }>("sd_overview"),
      invoke<string>("get_compiler"),
    ]);
    setData(p1);
    setDesign(p2.sessions);
    setConfig(p1.settings.config);
    setApiKey("");
    setCompiler(compiler);
  }
  useEffect(() => {
    if (active && isTauri()) void load().catch((e) => setError(String(e)));
  }, [active]);
  async function perform(task: () => Promise<void>) {
    setBusy(true);
    setError("");
    setNotice("");
    try {
      await task();
      await load();
      setRevision((n) => n + 1);
      onSaved();
      setNotice(t("settingsSaved"));
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  const keyStatus = data
    ? config.provider !== data.settings.config.provider
      ? "unsaved"
      : data.settings.keyStatus
    : undefined;
  const [scopeModule, scopeId] = scope.split(":");
  return (
    <main className="settings-page">
      <header>
        <h1>{t("settingsPage")}</h1>
        <p className="muted">{t("settingsPageNote")}</p>
      </header>
      {error && <Banner status="error" title={localizeError(error, lang)} />}
      {notice && <p role="status">{notice}</p>}
      <div className="provider-settings">
        <Collapsible
          trigger={
            <span className="settings-key-title">
              <strong>{t("settingsTitle")}</strong>{" "}
              <ApiKeyBadge status={keyStatus} provider={config.provider} />
            </span>
          }
          defaultIsOpen={!data || data.settings.keyStatus !== "saved"}
        >
          <ApiKeyWarning status={keyStatus} provider={config.provider} />
          <div className="provider-fields settings-provider-row">
            <Selector
              label={t("providerLabel")}
              size="sm"
              width={140}
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
              width={200}
              value={config.model}
              isDisabled={busy}
              onChange={(v) => setConfig({ ...config, model: v })}
            />
            <NumberInput
              label={t("timeoutSeconds")}
              size="sm"
              width={120}
              min={15}
              max={120}
              value={config.timeoutSeconds}
              isDisabled={busy}
              onChange={(v) => setConfig({ ...config, timeoutSeconds: v })}
            />
            <TextInput
              label={t("providerKeyLabel", { p: config.provider })}
              type="password"
              autoComplete="off"
              size="sm"
              width={220}
              placeholder={t("apiKeyPlaceholder")}
              value={apiKey}
              isDisabled={busy}
              onChange={setApiKey}
            />
            <Button
              label={t("saveSettingsShort")}
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
              label={t("deleteKeyShort")}
              variant="destructive"
              size="sm"
              isDisabled={busy || !data}
              onClick={() => setConfirm("key")}
            />
          </div>
          <p className="muted">{t("settingsNote")}</p>
          {data && (
            <p className="muted">
              {t("credentialPath", { path: data.settings.credentialPath })}
            </p>
          )}
        </Collapsible>
      </div>

      <UpdateSettings />
      <VoiceSettings active={active} />
      <section className="card settings-accounting">
        <h2>{t("compiler")}</h2>
        <div className="provider-fields">
          <TextInput
            label={t("compiler")}
            value={compiler}
            onChange={setCompiler}
            placeholder={t("compilerPlaceholder")}
            isDisabled={busy}
            width={420}
          />
          <Button
            label={t("detectSave")}
            variant="secondary"
            className="tint-btn tint-green"
            isDisabled={!data || busy}
            onClick={() =>
              perform(async () => {
                const path = await invoke<string>("set_compiler", {
                  path: compiler,
                });
                setCompiler(path);
              })
            }
          />
        </div>
      </section>
      <section className="card settings-accounting">
        <h2>{t("usageTitle")}</h2>
        <Selector
          label={t("usageScope")}
          value={scope}
          onChange={setScope}
          options={[
            { value: "all", label: t("usageAll") },
            ...(data?.sessions ?? []).map((s) => ({
              value: `p1:${s.id}`,
              label: `P1 · ${s.title.slice(0, 65)}`,
            })),
            ...design.map((s) => ({
              value: `p2:${s.id}`,
              label: `P2 · ${s.title.slice(0, 65)}`,
            })),
          ]}
        />
        <UsagePanel
          key={scope}
          module={scopeModule === "p2" ? "p2" : "p1"}
          sessionId={scopeId}
          provider={config.provider}
          model={config.model}
          refreshKey={`${revision}-${active}`}
        />
      </section>
      <AlertDialog
        isOpen={confirm === "key"}
        onOpenChange={(open) => {
          if (!open && !busy) setConfirm(null);
        }}
        title={t("confirmKeyTitle", { p: config.provider })}
        description={t("confirmKey")}
        actionLabel={t("confirmDelete")}
        cancelLabel={t("cancel")}
        isActionLoading={busy}
        onAction={async () => {
          await perform(async () => {
            await invoke("delete_llm_key", { provider: config.provider });
            setApiKey("");
          });
          setConfirm(null);
        }}
      />
    </main>
  );
}
