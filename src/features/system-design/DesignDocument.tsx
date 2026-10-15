import { useRef, useState } from "react";
import { Button } from "@astryxdesign/core/Button";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { useI18n } from "../../i18n";

export function Markdown({ text }: { text: string }) {
  return (
    <div className="sd-markdown">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        components={{
          // Notes never fetch images or open user/model supplied external destinations.
          img: () => null,
          a: ({ children }) => <span>{children}</span>,
        }}
      >
        {text}
      </ReactMarkdown>
    </div>
  );
}

export function Diagram({ source, dark }: { source: string; dark: boolean }) {
  const { t } = useI18n();
  const [preview, setPreview] = useState<{
    source: string;
    svg: string;
    dark: boolean;
  } | null>(null);
  const [error, setError] = useState(false),
    [busy, setBusy] = useState(false);
  const sequence = useRef(0);
  async function render() {
    if (busy || !source.trim()) return;
    setBusy(true);
    setError(false);
    try {
      const { default: mermaid } = await import("mermaid");
      mermaid.initialize({
        startOnLoad: false,
        securityLevel: "strict",
        suppressErrorRendering: true,
        maxTextSize: 12000,
        maxEdges: 150,
        flowchart: { htmlLabels: false },
        theme: dark ? "dark" : "neutral",
      });
      const { svg } = await mermaid.render(
        `sd-diagram-${Date.now()}-${++sequence.current}`,
        source,
      );
      setPreview({ source, svg, dark });
    } catch {
      setPreview(null);
      setError(true);
    } finally {
      setBusy(false);
    }
  }
  return (
    <div className="sd-diagram">
      <Button
        label={busy ? t("sdRendering") : t("sdRender")}
        size="sm"
        isDisabled={busy || !source.trim()}
        onClick={render}
      />
      {error && <p role="alert">{t("sdDiagramError")}</p>}
      {!preview && !error && <p className="muted">{t("sdDiagramEmpty")}</p>}
      {preview && (
        <>
          {(preview.source !== source || preview.dark !== dark) && (
            <p className="muted">{t("sdDiagramStale")}</p>
          )}
          {/* Rendered SVG is isolated: no scripts, navigation, forms or network access. */}
          <iframe
            title={t("sdDiagramPreview")}
            sandbox=""
            srcDoc={`<!doctype html><meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'"><style>:root{color-scheme:${preview.dark ? "dark" : "light"}}body{margin:12px}svg{max-width:100%;height:auto}</style>${preview.svg}`}
          />
        </>
      )}
    </div>
  );
}
