// Turns the partial JSON a model is streaming into readable text for the UI.
// Only candidate-facing fields are shown: followUpDirections, evaluationChecklist
// and other interviewer-only fields stay hidden, as they do in the finished view.
const LABELS: Record<string, string> = {
  question: "题目",
  background: "背景",
  constraints: "约束",
  observation: "面试官观察",
  followUpQuestion: "追问",
  answer: "参考回答",
  keyPoints: "要点",
  summary: "总结",
  evidence: "依据",
  improvement: "改进建议",
  strengths: "优势",
  gaps: "不足",
  nextPracticeTopics: "后续练习",
};

function unescape(raw: string): string {
  // Drop an escape cut off at the end of the chunk, then let JSON decode the rest.
  const safe = raw.replace(/\\u[0-9a-fA-F]{0,3}$|\\$/, "");
  try {
    return JSON.parse(`"${safe}"`);
  } catch {
    return safe;
  }
}

export type StreamSection = { label: string; text: string };

export function readablePartialJson(partial: string): StreamSection[] {
  const sections: StreamSection[] = [];
  const stack: string[] = [];
  let key = "";
  let i = 0;
  while (i < partial.length) {
    const c = partial[i];
    if (c !== '"') {
      if (c === "{" || c === "[") stack.push(c);
      else if (c === "}" || c === "]") stack.pop();
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
    if (isKey) {
      if (complete) key = value;
    } else if (LABELS[key] && value.trim()) {
      const item =
        partial[p] === "[" || partial[p] === "," ? `• ${value}` : value;
      const label = LABELS[key];
      const last = sections[sections.length - 1];
      if (last && last.label === label) last.text += `\n${item}`;
      else sections.push({ label, text: item });
    }
    i = j + 1;
  }
  return sections;
}
