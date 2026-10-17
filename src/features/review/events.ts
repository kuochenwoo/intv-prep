/** Local practice mutations invalidate the derived Review overview. */
const REVIEW_CHANGED = "swe-review-changed";
export function notifyReviewChanged() {
  window.dispatchEvent(new Event(REVIEW_CHANGED));
}
export function nextMidnight(now: number): number {
  const next = new Date(now);
  next.setHours(24, 0, 0, 0);
  return next.getTime();
}
export function watchReviewChanges(refresh: () => void): () => void {
  let timer: ReturnType<typeof setTimeout>;
  const schedule = () => {
    clearTimeout(timer);
    timer = setTimeout(
      () => {
        refresh();
        schedule();
      },
      nextMidnight(Date.now()) - Date.now() + 100,
    );
  };
  const resume = () => {
    refresh();
    schedule();
  };
  window.addEventListener(REVIEW_CHANGED, refresh);
  window.addEventListener("focus", resume);
  document.addEventListener("visibilitychange", resume);
  schedule();
  return () => {
    clearTimeout(timer);
    window.removeEventListener(REVIEW_CHANGED, refresh);
    window.removeEventListener("focus", resume);
    document.removeEventListener("visibilitychange", resume);
  };
}
