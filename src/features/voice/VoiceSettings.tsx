import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { Button } from "@astryxdesign/core/Button";
import { TextInput } from "@astryxdesign/core/TextInput";
import { Banner } from "@astryxdesign/core/Banner";
import { useI18n } from "../../i18n";
import { localizeError } from "../../errors";

type Settings = { model: string; keyStatus: string };
export function VoiceSettings({ active }: { active: boolean }) {
  const { t, lang } = useI18n();
  const [settings, setSettings] = useState<Settings | null>(null);
  const [key, setKey] = useState("");
  const [working, setWorking] = useState(false);
  const [error, setError] = useState("");
  const [saved, setSaved] = useState(false);
  useEffect(() => {
    if (!active || !isTauri()) return;
    let current = true;
    invoke<Settings>("transcription_settings")
      .then((value) => {
        if (current) setSettings(value);
      })
      .catch((e) => {
        if (current) setError(String(e));
      });
    return () => {
      current = false;
      setKey("");
    };
  }, [active]);
  async function save() {
    setWorking(true);
    setError("");
    setSaved(false);
    try {
      await invoke("save_transcription_key", { apiKey: key.trim() });
      setKey("");
      setSettings(await invoke<Settings>("transcription_settings"));
      setSaved(true);
    } catch (e) {
      setError(String(e));
    } finally {
      setWorking(false);
    }
  }
  return (
    <section className="card settings-accounting">
      <h2>{t("voiceSettingsTitle")}</h2>
      <p className="muted">{t("voiceSettingsScope")}</p>
      <p>
        {t("voiceModel")} ·{" "}
        {settings?.keyStatus === "saved"
          ? t("keySaved")
          : settings?.keyStatus === "unavailable"
            ? t("keyUnavailable")
            : t("keyMissing")}
      </p>
      <div className="provider-fields">
        <TextInput
          type="password"
          autoComplete="off"
          label={t("voiceKeyLabel")}
          placeholder={t("apiKeyPlaceholder")}
          value={key}
          onChange={setKey}
          isDisabled={working}
          width={300}
        />
        <Button
          size="sm"
          variant="primary"
          label={t("saveSettingsShort")}
          isDisabled={working || !settings || !key.trim()}
          isLoading={working}
          onClick={() => void save()}
        />
      </div>
      {error && <Banner status="error" title={localizeError(error, lang)} />}
      {saved && <p role="status">{t("settingsSaved")}</p>}
      <p className="muted">{t("voiceAccounting")}</p>
    </section>
  );
}
