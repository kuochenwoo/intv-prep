// Turns the partial JSON a model is streaming into readable text for the UI.
// Only candidate-facing fields are shown: followUpDirections, evaluationChecklist
// and other interviewer-only fields stay hidden, as they do in the finished view.
const SHOWN = new Set([
  "question",
  "background",
  "constraints",
  "observation",
  "followUpQuestion",
  "answer",
  "keyPoints",
  "summary",
  "evidence",
  "improvement",
  "strengths",
  "gaps",
  "nextPracticeTopics",
]);

function unescape(raw: string): string {
  // Drop an escape cut off at the end of the chunk, then let JSON decode the rest.
  const safe = raw.replace(/\\u[0-9a-fA-F]{0,3}$|\\$/, "");
  try {
    return JSON.parse(`"${safe}"`);
  } catch {
    return safe;
  }
}

/** `field` is the JSON key; the UI translates it for display. */
type StreamSection = { field: string; text: string };

export function readablePartialJson(partial: string): StreamSection[] {
  const sections: StreamSection[] = [];
  const stack: string[] = [];
  // The key each open container sits under, so nested fields know their parent.
  const parents: string[] = [];
  let key = "";
  let i = 0;
  while (i < partial.length) {
    const c = partial[i];
    if (c !== '"') {
      if (c === "{" || c === "[") {
        stack.push(c);
        parents.push(key);
      } else if (c === "}" || c === "]") {
        stack.pop();
        parents.pop();
      }
      i++;
      continue;
    }
    let j = i + 1;
    while (j < partial.length && partial[j] !== '"')
      j += partial[j] === "\\" ? 2 : 1;
    const value = unescape(partial.slice(i + 1, Math.min(j, partial.length)));
    const complete = j < partial.length;
    let k = j + 1;
    while (k < partial.length && /\s/.test(partial[k])) k++;
    let p = i - 1;
    while (p >= 0 && /\s/.test(partial[p])) p--;
    // Inside an object, a string after "{" or "," is a key, even before its ":" arrives.
    const isKey =
      (complete && partial[k] === ":") ||
      (stack[stack.length - 1] === "{" &&
        (partial[p] === "{" || partial[p] === ","));
    // System design gaps are {question, gap} objects: the question is the interviewer's
    // own text, so only the gap is shown, listed under "gaps".
    const inGaps = parents.includes("gaps");
    const field = inGaps && (key === "gap" || key === "gaps") ? "gaps" : key;
    const shown = inGaps ? field === "gaps" : SHOWN.has(key);
    if (isKey) {
      if (complete) key = value;
    } else if (shown && value.trim()) {
      const item =
        inGaps || partial[p] === "[" || partial[p] === "," ? `• ${value}` : value;
      const last = sections[sections.length - 1];
      if (last && last.field === field) last.text += `\n${item}`;
      else sections.push({ field, text: item });
    }
    i = j + 1;
  }
  return sections;
}
