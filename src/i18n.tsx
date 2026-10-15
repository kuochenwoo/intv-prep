import { createContext, useContext, useState, type ReactNode } from "react";

export type Lang = "zh" | "en";

// UI strings. Problem and topic text has its own `en` blocks in the content files,
// and backend errors are translated in errors.ts.
const strings = {
  // App shell
  appTitle: ["SWE 面试准备", "SWE Interview Prep"],
  moduleNav: ["训练模块", "Training modules"],
  moduleLc: ["LC 本地判题", "LeetCode Judge"],
  moduleDeepDive: ["技术深挖", "Technical Deep Dive"],
  switchLang: ["English", "中文"],
  switchLangTip: ["切换到英文界面", "Switch to Chinese"],
  themeLight: ["浅色", "Light"],
  themeDark: ["深色", "Dark"],
  themeMode: ["外观", "Appearance"],

  // Coding
  needDesktop: [
    "请通过 npm run tauri dev 打开桌面应用。浏览器预览无法调用本地编译器。",
    "Open the desktop app with npm run tauri dev. The browser preview cannot reach the local compiler.",
  ],
  draftSaveFailed: ["草稿保存失败：{e}", "Could not save the draft: {e}"],
  compilerDetected: [
    "已检测到编译器：{path}；判题时使用 -std=c++20 编译。",
    "Compiler found: {path}. Judging compiles with -std=c++20.",
  ],
  offlineJudge: ["C++20 · 离线判题", "C++20 · offline judge"],
  compiler: ["编译器", "Compiler"],
  compilerPlaceholder: [
    "自动探测 clang++ / g++，或填写完整路径",
    "Auto-detect clang++ / g++, or enter a full path",
  ],
  detectSave: ["检测并保存", "Detect and save"],
  problems: ["练习题", "Problems"],
  problemCount: ["{n} 道", "{n} problems"],
  timer: ["计时", "Timer"],
  timerOff: ["不计时", "No timer"],
  timerMinutes: ["{n} 分钟", "{n} min"],
  timerPause: ["暂停", "Pause"],
  timerResume: ["继续", "Resume"],
  timerRestart: ["重新计时", "Restart"],
  timeLeft: ["剩余 {t}", "{t} left"],
  timeUp: ["时间到", "Time's up"],
  searchProblems: ["搜索题目", "Search problems"],
  searchPlaceholder: ["标题 / 编号 / 标签", "Title, number or tag"],
  difficulty: ["难度", "Difficulty"],
  allDifficulty: ["全部难度", "All difficulties"],
  easy: ["简单", "Easy"],
  medium: ["中等", "Medium"],
  hard: ["困难", "Hard"],
  tag: ["标签", "Tag"],
  allTags: ["全部标签", "All tags"],
  modeFunction: ["函数", "Function"],
  modeAcm: ["ACM", "ACM"],
  progressDone: ["完成", "done"],
  progressDoing: ["在做", "started"],
  progressTodo: ["未做", "not started"],
  noMatch: ["没有符合条件的题目。", "No problems match these filters."],
  constraints: ["约束", "Constraints"],
  complexity: ["时间 / 空间复杂度要求", "Time / space requirements"],
  timeLabel: ["时间", "Time"],
  spaceLabel: ["额外空间", "Extra space"],
  officialSource: ["官方题面（需联网）", "Official problem (online)"],
  localLimits: ["本地执行上限", "Local run limits"],
  limitsLine: [
    "每例 {t} ms · 内存 {m} MiB · 输出 {o} KiB",
    "{t} ms per case · {m} MiB memory · {o} KiB output",
  ],
  limitsNote: [
    "这是本地设置，不是 LeetCode 官方资源限额。内存包含输入、包装器和运行时，每 5 ms 采样一次，可能漏掉短暂峰值，不等同于算法额外空间。",
    "These are local settings, not LeetCode's official limits. Memory includes input, harness and runtime, sampled every 5 ms, so short peaks can be missed; it is not the algorithm's extra space.",
  ],
  bigONote: [
    "通过本地测试不代表已满足 Big-O 或原地要求，仍需自行检查复杂度。",
    "Passing local tests does not prove the Big-O or in-place requirement; check the complexity yourself.",
  ],
  examples: ["示例（逻辑数据）", "Examples (logical data)"],
  exampleIO: ["输入：{i}\n输出：{o}", "Input: {i}\nOutput: {o}"],
  acmProtocol: ["ACM 输入输出协议", "ACM input/output protocol"],
  notSandbox: [
    "程序以当前用户权限运行你自己的代码，不是安全沙箱。",
    "Your code runs with your user's permissions. This is not a security sandbox.",
  ],
  codingMode: ["编程模式", "Coding mode"],
  modeFunctionFull: ["LeetCode 核心函数", "LeetCode function"],
  modeAcmFull: ["ACM 完整程序", "ACM full program"],
  saving: ["正在保存…", "Saving…"],
  saved: ["草稿已保存", "Draft saved"],
  mainProvided: ["系统提供 main()", "main() is provided"],
  mainYours: ["自行实现 main()", "Write your own main()"],
  runSamples: ["运行样例", "Run samples"],
  submitTests: ["提交本地测试", "Submit all tests"],
  resultsPlaceholder: [
    "编译和运行结果将显示在这里。",
    "Compile and run results appear here.",
  ],
  passedOf: ["{p}/{t} 通过", "{p}/{t} passed"],
  lastRunNote: [
    "上次执行结果（样例或提交）；修改代码后不会自动重判",
    "Last run (samples or submit); editing code does not re-run it",
  ],
  runLimits: ["本次上限 {t} ms / {m} MiB", "Limits {t} ms / {m} MiB"],
  killed: ["信号 / 被终止", "signal / killed"],
  peakMemory: ["采样峰值 {m} MiB", "sampled peak {m} MiB"],
  noMemory: [
    "未取得内存（程序太短或旧记录）",
    "no memory sample (short run or old record)",
  ],
  caseStdin: ["stdin：", "stdin:"],
  caseExpected: [
    "预期（任意合法答案亦可）：",
    "Expected (any valid answer is accepted):",
  ],
  caseActual: ["实际：", "Actual:"],
  emptyOutput: ["（空输出）", "(empty output)"],
  stoppedEarly: [
    "超出时间、内存或输出限制后已停止后续测试。",
    "Remaining tests were skipped after a time, memory or output limit was hit.",
  ],
  status_accepted: ["通过", "Accepted"],
  status_wrong_answer: ["答案错误", "Wrong answer"],
  status_compile_error: ["编译错误", "Compile error"],
  status_runtime_error: ["运行错误", "Runtime error"],
  status_timeout: ["运行超时", "Time limit exceeded"],
  status_output_limit: ["输出超限", "Output limit exceeded"],
  status_memory_limit: ["内存超限", "Memory limit exceeded"],
  status_compile_timeout: ["编译超时", "Compile timeout"],
  status_compile_output_limit: ["编译输出超限", "Compiler output too long"],

  // Deep dive
  ddTagline: [
    "具体场景 → 方案 → 追问 → 反馈",
    "Scenario → solution → follow-ups → feedback",
  ],
  settingsTitle: ["模型与密钥", "Model and API key"],
  providerUnsaved: ["Provider 更改尚未保存", "Provider change not saved"],
  keySaved: ["密钥已保存到 .env", "Key saved in .env"],
  keyUnavailable: [
    ".env 暂不可读或配置不完整",
    ".env unreadable or incomplete",
  ],
  keyMissing: ["尚未保存密钥", "No key saved"],
  modelName: ["模型名称", "Model"],
  timeoutSeconds: ["请求超时（秒）", "Request timeout (s)"],
  apiKeyPlaceholder: [
    "留空则保留已存 Key",
    "Leave empty to keep the saved key",
  ],
  saveSettings: ["保存设置与密钥", "Save settings and key"],
  deleteKey: ["删除当前 Provider 密钥", "Delete this provider's key"],
  settingsNote: [
    "密钥只保存在项目 .env 中。Claude 和 DeepSeek 的密钥分开保存。新设置用于新会话，已有会话继续使用创建时的模型。",
    "Keys are stored only in the project's .env, separately for Claude and DeepSeek. New settings apply to new sessions; existing sessions keep their original model.",
  ],
  privacyNote: [
    "生成题目、提交回答和生成反馈会把主题、题目和必要的会话内容发送给 Provider，可能产生 API 费用。查看缓存、复用题目和保存草稿不联网。AI 反馈仅供练习参考。",
    "Generating questions, submitting answers and getting feedback send the topic, question and needed conversation to the provider and may cost API credits. Viewing cache, reusing questions and saving drafts stay offline. AI feedback is for practice only.",
  ],
  online: ["网络可用", "Online"],
  offline: ["离线", "Offline"],
  apiUnverified: ["API 尚未验证", "API not checked yet"],
  requesting: ["正在请求 {p}…", "Calling {p}…"],
  lastOk: ["上次 API 请求成功", "Last API call succeeded"],
  lastFailed: [
    "上次联网操作失败，可重试",
    "Last online action failed; you can retry",
  ],
  busyNote: [
    "处理中，请等待完成或超时",
    "Working; wait for it to finish or time out",
  ],
  aiWriting: ["AI 正在生成…", "AI is writing…"],
  confirmKey: [
    "确认从 .env 删除 {p} 的密钥？历史记录和其他 Provider 的密钥会保留。",
    "Delete the {p} key from .env? History and the other provider's key are kept.",
  ],
  confirmHistory: [
    "确认清空所有技术深挖会话、反馈和缓存题目？LC 数据和密钥会保留。",
    "Clear all deep-dive sessions, feedback and cached questions? LeetCode data and keys are kept.",
  ],
  confirmSession: [
    "确认删除当前面试及其反馈？缓存题目会保留。",
    "Delete this interview and its feedback? Cached questions are kept.",
  ],
  confirmDelete: ["确认删除", "Delete"],
  cancel: ["取消", "Cancel"],
  topics: ["训练主题", "Topics"],
  topicCount: ["{n} 个", "{n}"],
  randomTopic: ["随机选主题", "Random topic"],
  history: ["面试历史（本地）", "Interview history (local)"],
  noSessions: ["还没有面试会话。", "No interviews yet."],
  clearHistory: [
    "清空面试历史与题目缓存",
    "Clear history and cached questions",
  ],
  seedConstraints: ["种子约束：{c}", "Seed constraints: {c}"],
  working: ["处理中…", "Working…"],
  generateStart: [
    "生成新题并开始（联网）",
    "Generate a question and start (online)",
  ],
  cachedQuestions: ["已缓存问题 · {n}", "Cached questions · {n}"],
  pickCached: ["选择已缓存问题", "Pick a cached question"],
  generatedAt: ["生成于 {d} · {m}", "Generated {d} · {m}"],
  reuseStart: [
    "复用此题开始新面试（离线）",
    "Start a new interview with this question (offline)",
  ],
  cacheHint: [
    "首次生成后会保存在本地，再练同一题不必重新生成。",
    "Questions are saved locally after the first generation, so you can practice them again without regenerating.",
  ],
  noSession: [
    "生成新题、复用缓存题目，或从历史中恢复面试。",
    "Generate a question, reuse a cached one, or resume an interview from history.",
  ],
  interviewTitle: ["当前面试", "Interview"],
  deleteSession: ["删除此会话", "Delete interview"],
  roleAiAnswer: ["AI 参考代答", "AI answer (submitted)"],
  roleYou: ["你的回答", "Your answer"],
  roleInterviewer: ["面试官追问", "Interviewer"],
  refKeyPoints: ["参考作答要点", "Key points"],
  refShown: [
    "AI 参考回答（未提交，仅供参考）",
    "AI reference answer (not submitted)",
  ],
  copyToAnswer: [
    "复制到我的回答（可修改后提交）",
    "Copy into my answer (edit, then submit)",
  ],
  submitRef: [
    "直接提交此回答并追问（联网）",
    "Submit this answer and continue (online)",
  ],
  clearDraftFirst: ["请先清空回答草稿", "Clear your draft first"],
  savedLocally: [
    "{e} · 内容已保存在本地。",
    "{e} · Your content is saved locally.",
  ],
  pendingReply: [
    "回答已保存，等待本轮追问。重试不会重复追加回答。",
    "Answer saved; waiting for the follow-up. Retrying will not add the answer twice.",
  ],
  pendingFeedback: [
    "已保存结束请求，等待生成反馈。重试不会新建会话。",
    "Finish request saved; waiting for feedback. Retrying will not create a new session.",
  ],
  retryPending: ["重试待处理请求（联网）", "Retry pending request (online)"],
  yourAnswer: [
    "你的回答（自动保存，最多 6000 字符）",
    "Your answer (auto-saved, up to 6000 characters)",
  ],
  answerPlaceholder: [
    "说明你的方案、依据、替代方案与失败场景…",
    "Explain your approach, reasoning, alternatives and failure cases…",
  ],
  draftFailed: ["草稿保存失败，请重试", "Draft not saved; try again"],
  rounds: ["已回答 {n}/20 轮", "{n}/20 rounds answered"],
  retrySave: ["重试保存草稿", "Retry saving draft"],
  submitAnswer: ["提交回答并追问（联网）", "Submit and get follow-up (online)"],
  refReady: ["已生成参考回答（见上方）", "Reference answer shown above"],
  aiReference: ["AI 参考回答（联网）", "AI reference answer (online)"],
  finishFeedback: [
    "结束并生成反馈（联网）",
    "Finish and get feedback (online)",
  ],
  refHint: [
    "AI 参考回答只显示一份简洁答案，不会自动提交或追问。",
    "The AI reference shows one concise answer; it is not submitted and asks no follow-up.",
  ],
  finishNeedsClear: [
    "结束前请先提交或清空当前草稿。",
    "Submit or clear your draft before finishing.",
  ],
  suggestEnd: [
    "面试官建议可以结束并查看反馈，你也可以继续回答。",
    "The interviewer suggests wrapping up for feedback; you can also keep answering.",
  ],
  feedbackTitle: ["面试反馈 · 定性评估", "Feedback · qualitative"],
  evidence: ["依据：", "Evidence: "],
  advice: ["建议：", "Advice: "],
  strengths: ["做得好的方面", "Strengths"],
  gaps: ["遗漏与风险", "Gaps and risks"],
  nextTopics: ["后续训练方向", "What to practice next"],
  state_active: ["进行中", "In progress"],
  state_awaiting_reply: ["等待追问 / 可重试", "Awaiting follow-up / retry"],
  state_feedback_pending: ["等待反馈 / 可重试", "Awaiting feedback / retry"],
  state_finished: ["已结束", "Finished"],
  rating_good: ["良好", "Good"],
  rating_partial: ["部分具备", "Partial"],
  rating_missing: ["遗漏", "Missing"],
  rating_incorrect: ["有误", "Incorrect"],
  rating_uncertain: ["待确认", "Uncertain"],
  dim_principles: ["原理理解", "Principles"],
  dim_selection_rationale: ["技术选型依据", "Selection rationale"],
  dim_tradeoffs: ["替代方案权衡", "Trade-offs"],
  dim_failure_and_correctness: ["故障边界与正确性", "Failure and correctness"],
  dim_communication: ["沟通清晰度", "Communication"],
  stream_question: ["题目", "Question"],
  stream_background: ["背景", "Background"],
  stream_constraints: ["约束", "Constraints"],
  stream_observation: ["面试官观察", "Observation"],
  stream_followUpQuestion: ["追问", "Follow-up"],
  stream_answer: ["参考回答", "Answer"],
  stream_keyPoints: ["要点", "Key points"],
  stream_summary: ["总结", "Summary"],
  stream_evidence: ["依据", "Evidence"],
  stream_improvement: ["改进建议", "Improvement"],
  stream_strengths: ["优势", "Strengths"],
  stream_gaps: ["不足", "Gaps"],
  stream_nextPracticeTopics: ["后续练习", "Practice next"],
} satisfies Record<string, [string, string]>;

export type Key = keyof typeof strings;
export type T = (key: Key, vars?: Record<string, string | number>) => string;

function translate(lang: Lang): T {
  return (key, vars) => {
    let s = strings[key][lang === "zh" ? 0 : 1];
    for (const [k, v] of Object.entries(vars ?? {}))
      s = s.split(`{${k}}`).join(String(v));
    return s;
  };
}

/** Looks up a dynamic key such as `status_${x}`; falls back to the raw value. */
export function lookup(t: T, prefix: string, value: string): string {
  const key = `${prefix}${value}`;
  return key in strings ? t(key as Key) : value;
}

const Ctx = createContext<{ lang: Lang; setLang: (l: Lang) => void; t: T }>({
  lang: "zh",
  setLang: () => {},
  t: translate("zh"),
});

function stored(): Lang {
  try {
    return localStorage.getItem("lang") === "en" ? "en" : "zh";
  } catch {
    return "zh";
  }
}

export function LangProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Lang>(stored);
  const setLang = (l: Lang) => {
    setLangState(l);
    try {
      localStorage.setItem("lang", l);
    } catch {
      /* the choice just isn't remembered */
    }
  };
  return (
    <Ctx.Provider value={{ lang, setLang, t: translate(lang) }}>
      {children}
    </Ctx.Provider>
  );
}

export const useI18n = () => useContext(Ctx);
