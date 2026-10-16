/** Apply async navigation only while its owner is still current and mounted. */
export async function loadCurrent<T>(
  load: () => Promise<T>,
  apply: (value: T) => void,
  isCurrent: () => boolean,
): Promise<void> {
  if (!isCurrent()) return;
  const value = await load();
  if (isCurrent()) apply(value);
}
