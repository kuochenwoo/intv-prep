/** Unknown provider costs must remain distinct from a reported zero. */
export function summarizeCosts(costs: readonly { usd: number | null }[]) {
  return costs.reduce(
    (summary, cost) =>
      cost.usd === null
        ? { ...summary, unknown: summary.unknown + 1 }
        : {
            ...summary,
            known: summary.known + 1,
            total: summary.total + cost.usd,
          },
    { total: 0, known: 0, unknown: 0 },
  );
}
