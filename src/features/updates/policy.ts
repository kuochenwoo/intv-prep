export const UPDATE_CHECK_INTERVAL_MS = 6 * 60 * 60 * 1000;

export function downloadPercent(
  received: number,
  total?: number,
): number | undefined {
  if (!total || total <= 0) return undefined;
  return Math.max(0, Math.min(99, Math.floor((received / total) * 100)));
}
