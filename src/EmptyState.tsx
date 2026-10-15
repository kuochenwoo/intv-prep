import { useI18n } from "./i18n";

/** A small centered card for "nothing here yet" messages. */
export function EmptyState({
  icon,
  text,
  compact = false,
}: {
  icon: string;
  text: string;
  compact?: boolean;
}) {
  return (
    <div className={`empty-state ${compact ? "compact" : ""}`}>
      <span className="empty-icon" aria-hidden="true">
        {icon}
      </span>
      <p>{text}</p>
    </div>
  );
}

/** Feedback lists as colored cards: strengths green, gaps orange, practice blue. */
export function FeedbackLists({
  strengths,
  gaps,
  next,
}: {
  strengths: string[];
  gaps: string[];
  next: string[];
}) {
  const { t } = useI18n();
  const cards = [
    ["strengths", "tint-green", strengths],
    ["gaps", "tint-orange", gaps],
    ["nextTopics", "tint-blue", next],
  ] as const;
  return (
    <div className="feedback-lists">
      {cards.map(([key, tint, items]) => (
        <section key={key} className={`feedback-list-card ${tint}`}>
          <h3>{t(key)}</h3>
          {items.length ? (
            <ul>
              {items.map((s, i) => (
                <li key={i}>{s}</li>
              ))}
            </ul>
          ) : (
            <p className="muted">{t("feedbackListEmpty")}</p>
          )}
        </section>
      ))}
    </div>
  );
}
