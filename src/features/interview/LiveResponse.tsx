import { useState } from "react";
import { Button } from "@astryxdesign/core/Button";
import { lookup, useI18n, type T } from "../../i18n";
import { Markdown } from "../../Markdown";
import { readablePartialJson } from "./stream";

/**
 * The model's streamed output, shown while a request runs. It floats in the bottom
 * corner so it stays visible next to the answer box, and can be minimized.
 */
export function LiveResponse({ text }: { text: string }) {
  const { t } = useI18n();
  const [minimized, setMinimized] = useState(false);
  const sections = readablePartialJson(text);
  return (
    <div
      className={`live-response ${minimized ? "minimized" : ""}`}
      aria-live="polite"
    >
      <div className="live-head">
        <strong>{t("aiWriting")}</strong>
        <Button
          size="sm"
          variant="ghost"
          label={minimized ? t("liveShow") : t("liveHide")}
          onClick={() => setMinimized(!minimized)}
        />
      </div>
      {!minimized &&
        sections.map((s, i) => (
          <section key={i}>
            <span>{label(t, s.field)}</span>
            <Markdown text={s.text} />
          </section>
        ))}
    </div>
  );
}

// lookup() falls back to the raw field name; a field without a label shows no heading.
function label(t: T, field: string) {
  const text = lookup(t, "stream_", field);
  return text === field ? "" : text;
}
