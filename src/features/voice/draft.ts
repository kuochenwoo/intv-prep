/** Preserve the typed draft; never truncate either part to make a transcript fit. */
export function appendTranscript(
  draft: string,
  transcript: string,
  limit = 6000,
): string | null {
  if (!transcript.trim()) return null;
  const combined =
    draft.trimEnd() + (draft.trim() ? "\n\n" : "") + transcript.trim();
  return combined.length <= limit ? combined : null;
}
