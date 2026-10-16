import { Badge } from "@astryxdesign/core/Badge";
import { Banner } from "@astryxdesign/core/Banner";
import { useI18n } from "./i18n";

type Props = { status?: string; provider: string };

export function ApiKeyBadge({ status, provider }: Props) {
  const { t } = useI18n();
  if (!status) return null;
  const label = status === "saved"
    ? t("keySaved")
    : status === "unsaved"
      ? t("providerUnsaved")
      : status === "unavailable"
        ? t("keyUnavailable")
        : t("keyMissing");
  return (
    <span role="status" className="api-key-badge">
      <Badge
        variant={status === "saved" ? "success" : status === "unsaved" ? "neutral" : "warning"}
        label={`${provider} · ${label}`}
      />
    </span>
  );
}

export function ApiKeyWarning({ status, provider }: Props) {
  const { t } = useI18n();
  if (!status || status === "saved" || status === "unsaved") return null;
  return (
    <div className="api-key-warning">
      <Banner
        status="warning"
        title={status === "unavailable"
          ? t("apiKeyUnreadableWarning", { p: provider })
          : t("apiKeyMissingWarning", { p: provider })}
      />
    </div>
  );
}
