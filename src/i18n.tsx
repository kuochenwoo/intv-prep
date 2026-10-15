import { createContext, useContext, useState, type ReactNode } from "react";

export type Lang = "zh" | "en";

// UI strings. Problem and topic text has its own `en` blocks in the content files,
// and backend errors are translated in errors.ts.
const strings = {
  settingsPage: ["设置", "Settings"],
  settingsPageNote: [
    "管理模型、密钥、API 用量和费用。已有面试继续使用创建时的模型。",
    "Manage models, credentials, API usage and costs. Existing interviews keep their original model.",
  ],
  settingsSaved: ["设置已保存。", "Settings saved."],
  providerLabel: ["供应商", "Provider"],
  providerKeyLabel: ["{p} API 密钥", "{p} API key"],
  openHistory: ["面试历史 · {n}", "Interview history · {n}"],
  confirmExportTitle: ["确认导出面试记录？", "Export this interview?"],
  confirmExportDescription: [
    "将下载 {format} 文件，包含已保存的题目、对话、AI 辅助标记、参考回答、反馈、设计笔记及用量。文件可能包含你的个人练习内容；不包含未提交草稿、密钥或私有考点。",
    "Download a {format} file containing saved questions, conversation, AI-assistance labels, references, feedback, design notes and usage. It may contain personal practice content. Unsubmitted drafts, keys and private checkpoints are excluded.",
  ],
  confirmExportAction: ["确认导出", "Export file"],

  sessionTools: ["会话工具", "Session tools"],
  exportMarkdown: ["导出 Markdown", "Export Markdown"],
  exportJson: ["导出 JSON", "Export JSON"],
  exportReady: [
    "已创建导出文件，请查看下载目录。",
    "Export created. Check your downloads folder.",
  ],
  exportScope: [
    "导出已保存的题目、对话、参考回答、反馈及设计笔记；不含未提交草稿和私有考点。",
    "Exports saved questions, conversation, references, feedback and design notes; excludes unsubmitted drafts and private checkpoints.",
  ],
  mockFinished: ["计时已结束", "Timer ended"],
  mockPaused: ["已暂停", "Paused"],
  mockRunning: ["计时中", "Running"],
  mockProgress: ["已回答 {n} 轮", "{n} answers submitted"],
  mockExpired: [
    "时间到。草稿已保留，你可以继续作答，或提交后点击结束并生成反馈。不会自动发送回答。",
    "Time is up. Your draft is preserved. Continue answering, or submit it and finish for feedback. Nothing is sent automatically.",
  ],
  usageTitle: ["API 用量与费用", "API usage and costs"],
  usageExplanation: [
    "逐次记录真实 HTTP 请求，含审阅、修复和重试。token 由供应商返回；未知不代表零。费用按你填写的费率估算，不是账单。中断请求可能仍产生费用。",
    "Records each actual HTTP request, including reviews, repairs and retries. Tokens come from the provider; unknown is not zero. Costs use your rates and are estimates, not invoices. Interrupted requests may still be charged.",
  ],
  usageScope: ["记录范围", "Record scope"],
  usageSession: ["当前面试", "This interview"],
  usageAll: ["全部 API 请求", "All API requests"],
  usageRefresh: ["刷新记录", "Refresh records"],
  usageTotals: [
    "{n} 次请求 · 已知费用合计 {cost} · {unknown} 次费用未知",
    "{n} requests · Known estimates total {cost} · {unknown} with unknown cost",
  ],
  usageWhen: ["时间", "Time"],
  usageModel: ["模块 / 模型", "Module / model"],
  usageTask: ["用途", "Purpose"],
  usageTokens: ["输入 / 输出 token", "Input / output tokens"],
  usageEstimate: ["费用估算", "Estimated cost"],
  usageStatus: ["请求状态", "Request status"],
  usageUnknown: ["未知", "Unknown"],
  usageCache: ["缓存读 / 写", "Cache read / write"],
  usageReview: ["质量审阅", "Quality review"],
  usageEmpty: [
    "尚无请求记录。旧请求不会补算。",
    "No request records yet. Past requests are not backfilled.",
  ],
  usageRecent: [
    "显示最近 100 条；合计包含当前范围的所有记录。",
    "Showing the latest 100; totals include all records in this scope.",
  ],
  usagePrices: ["设置模型费率", "Set model rates"],
  usagePriceHelp: [
    "请参考供应商当前价格，填写每百万 token 的美元费率（支持缓存读写）。只有保存后发起的请求使用新费率，历史估算保留原费率；未设置时费用为未知。",
    "Enter current provider rates in USD per million tokens, including cache reads/writes. New rates apply only to future requests; historical estimates keep their original rates. Unconfigured costs remain unknown.",
  ],
  usageRate_input: ["输入（非缓存）", "Input (uncached)"],
  usageRate_output: ["输出", "Output"],
  usageRate_cacheRead: ["缓存读取", "Cache read"],
  usageRate_cacheWrite: ["缓存写入", "Cache write"],
  usageSavePrices: ["保存费率", "Save rates"],
  usagePricesSaved: [
    "费率已保存，仅用于后续请求。",
    "Rates saved for future requests.",
  ],
  usageOp_generation: ["出题", "Generation"],
  usageOp_follow_up: ["追问", "Follow-up"],
  usageOp_feedback: ["反馈", "Feedback"],
  usageOp_retry: ["重试待处理请求", "Retry pending request"],
  usageOp_reference: ["参考回答", "Reference answer"],
  usageOp_reference_submit: ["参考回答 / 提交追问", "Reference / follow-up"],
  usageOp_answer: ["追问", "Follow-up"],
  usageState_success: ["请求成功", "Request succeeded"],
  usageState_content_error: ["内容校验失败", "Content validation failed"],
  usageState_transport_error: ["网络 / HTTP 失败", "Network / HTTP failure"],
  usageState_interrupted: ["中断 / 未完成", "Interrupted / unfinished"],

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

  // System design
  sdCandidate: ["你", "You"],
  sdInterviewer: ["面试官", "Interviewer"],
  moduleSystemDesign: ["系统设计", "System Design"],
  sdTagline: [
    "需求 → 容量估算 → 架构 → 深入分析 → 权衡",
    "Requirements → estimates → architecture → deep dive → trade-offs",
  ],
  sdScope: [
    "围绕完整系统练习架构、容量与跨组件取舍。阶段由你控制，AI 只给建议。",
    "Practise complete architectures, capacity and cross-component trade-offs. You control stages; AI only suggests.",
  ],
  sdScenarios: ["设计场景", "Design scenarios"],
  sdSearch: ["搜索设计场景", "Search design scenarios"],
  sdSearchHint: [
    "标题、功能或架构关键词",
    "Title, feature or architecture keyword",
  ],
  sdMatches: [
    "显示 {n} / {total} 个场景 · 随机选择遵循筛选",
    "Showing {n} / {total} scenarios · random respects filters",
  ],
  sdClearSearch: ["清除搜索", "Clear search"],
  sdNoMatches: [
    "没有匹配场景，试试其他关键词或清除搜索。",
    "No matching scenarios. Try another keyword or clear search.",
  ],
  sdLocalTargets: [
    "以下规模与可靠性目标均为本地训练假设，并非真实产品的官方指标。估算时请明确平均值、峰值、字节单位、副本和额外开销。",
    "Scale and reliability targets below are local training assumptions, not official production metrics. State averages, peaks, byte units, replicas and additional overhead in estimates.",
  ],
  sdGenerate: [
    "生成设计题并开始（联网）",
    "Generate design question and start (online)",
  ],
  sdCache: ["复用缓存题（离线）", "Reuse cached question (offline)"],
  sdCached: ["已缓存设计题", "Cached design questions"],
  sdHistory: ["系统设计历史", "System design history"],
  sdEmpty: [
    "选择场景生成新题，或复用缓存题开始。已有面试可从左侧恢复。",
    "Choose a scenario to generate a question or reuse a cached one. Restore interviews from the history.",
  ],
  sdNoCache: [
    "此场景与语言下尚无缓存题，首次生成需要联网。",
    "No cached questions for this scenario and language. First generation needs a connection.",
  ],
  sdSeed: ["场景范围与训练假设", "Scenario scope and training assumptions"],
  sdFunctions: ["功能范围", "Functional scope"],
  sdScale: ["规模范围", "Scale ranges"],
  sdReliability: ["可靠性约束", "Reliability constraints"],
  sdDirections: ["可选架构深挖方向", "Optional architecture deep dives"],
  sdCurrent: ["当前设计面试", "Current design interview"],
  sdStage_requirements: ["需求澄清", "Requirements"],
  sdStage_estimation: ["容量估算", "Estimation"],
  sdStage_high_level: ["整体架构", "Architecture"],
  sdStage_deep_dive: ["深入分析", "Deep dive"],
  sdStage_tradeoffs: ["跨组件权衡", "Trade-offs"],
  sdStage_feedback: ["最终反馈", "Feedback"],
  sdStages: ["面试阶段（可自由切换）", "Interview stages (switch freely)"],
  sdStageTip: [
    "切换阶段不联网，各阶段草稿分别保留。新 AI 回复沿用本题语言。",
    "Switching stages stays offline; each stage keeps its own draft. New AI replies use this question's language.",
  ],
  sdSuggest: [
    "AI 建议：{stage}；是否切换由你决定。",
    "AI suggests: {stage}. You decide whether to switch.",
  ],
  sdNotes: [
    "设计笔记（Markdown，自动保存）",
    "Design notes (Markdown, auto-saved)",
  ],
  sdNotesHint: [
    "记录需求、公式与单位、API、数据模型、端到端数据流及取舍。提交回答或生成反馈时会发送给模型。",
    "Record requirements, formulas with units, APIs, data models, end-to-end flows and trade-offs. Sent with answers and feedback requests.",
  ],
  sdNotesPlaceholder: [
    "## 需求与假设\n\n## 容量估算\n公式、单位、平均与峰值\n\n## 架构与数据流\n\n## 一致性、故障与取舍",
    "## Requirements and assumptions\n\n## Capacity estimates\nFormulas, units, average and peak\n\n## Architecture and data flow\n\n## Consistency, failures and trade-offs",
  ],
  sdDiagram: [
    "架构图（Mermaid 源码，自动保存）",
    "Architecture diagram (Mermaid source, auto-saved)",
  ],
  sdDiagramHint: [
    "输入 flowchart 或 sequenceDiagram 源码；仅渲染本地预览，模型接收源码。",
    "Enter flowchart or sequenceDiagram source. Preview renders locally; the model receives source text.",
  ],
  sdRender: ["预览架构图", "Preview diagram"],
  sdRendering: ["正在渲染…", "Rendering…"],
  sdDiagramPreview: ["架构图预览", "Architecture diagram preview"],
  sdDiagramError: [
    "图表语法无法渲染，请检查 Mermaid 源码；源码已保留。",
    "Could not render this diagram. Check Mermaid syntax; your source is preserved.",
  ],
  sdDiagramEmpty: [
    "填写 Mermaid 源码后点击预览。",
    "Enter Mermaid source, then preview it.",
  ],
  sdDiagramStale: [
    "源码已更新，请重新预览。",
    "Source changed; preview it again.",
  ],
  sdNotesPreview: ["笔记预览", "Notes preview"],
  sdWorkspace: ["设计工作区", "Design workspace"],
  sdAnswer: ["本阶段回答（自动保存）", "Answer for this stage (auto-saved)"],
  sdAnswerHint: [
    "只提交本轮要讨论的内容；设计笔记与图会一并作为上下文。",
    "Submit what you want to discuss this round; notes and diagram accompany it as context.",
  ],
  sdAnswerPlaceholder: [
    "说明本阶段的假设、推导或架构决策及理由…",
    "Explain this stage's assumptions, calculations or architectural decisions and reasoning…",
  ],
  sdSubmit: [
    "提交并讨论本阶段（联网）",
    "Submit and discuss this stage (online)",
  ],
  sdFinish: [
    "结束并生成八维反馈（联网）",
    "Finish and get eight-dimension feedback (online)",
  ],
  sdUnsubmitted: [
    "还有未提交的阶段草稿，请提交或清空后结束。",
    "Some stages have unsubmitted drafts. Submit or clear them before finishing.",
  ],
  sdPending: [
    "回答或结束请求已保存。失败后可重试，不会重复提交回答。",
    "The answer or finish request is saved. Retry failures without submitting the answer twice.",
  ],
  sdSavedLanguage: [
    "题目与会话语言：{language}",
    "Question and conversation language: {language}",
  ],
  sdLangZh: ["中文", "Chinese"],
  sdLangEn: ["英文", "English"],
  sdAssessment_requirements: ["需求澄清", "Requirements clarification"],
  sdAssessment_estimation: [
    "数量级与容量估算",
    "Magnitude and capacity estimates",
  ],
  sdAssessment_architecture: ["架构与数据流", "Architecture and data flow"],
  sdAssessment_scalability: ["扩展性", "Scalability"],
  sdAssessment_availability: [
    "可用性与故障处理",
    "Availability and failure handling",
  ],
  sdAssessment_correctness: [
    "数据一致性与正确性",
    "Consistency and correctness",
  ],
  sdAssessment_tradeoffs: [
    "技术与跨组件取舍",
    "Technical and cross-component trade-offs",
  ],
  sdAssessment_communication: ["沟通表达", "Communication"],
  sdConfirmHistoryTitle: ["清空系统设计历史？", "Clear system design history?"],
  sdConfirmHistory: [
    "将删除所有系统设计会话和缓存设计题，无法恢复。LC、技术深挖数据和密钥会保留。",
    "This deletes all system design sessions and cached design questions and cannot be undone. LeetCode and Deep Dive data and keys are kept.",
  ],
  sdConfirmSessionTitle: [
    "删除当前系统设计面试？",
    "Delete this design interview?",
  ],
  sdConfirmSession: [
    "将删除这次面试及其反馈，无法恢复。缓存设计题会保留。",
    "This deletes the interview and its feedback and cannot be undone. Cached design questions are kept.",
  ],
  sdRetrySave: ["重试保存工作区", "Retry saving workspace"],
  sdModelSettings: [
    "模型设置（与技术深挖共享）",
    "Model settings (shared with Deep Dive)",
  ],
  sdProvider: ["模型供应商", "Provider"],
  sdCounts: ["{n} / {max} 字符", "{n} / {max} characters"],
  sdNoNotes: ["暂无笔记", "No notes yet"],
  sdThread: ["阶段讨论记录", "Stage discussion"],
  sdLoading: ["正在读取本地系统设计数据…", "Loading local system design data…"],

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
  timerStart: ["开始", "Start"],
  timerPause: ["暂停", "Pause"],
  timerResume: ["继续", "Resume"],
  timerReset: ["重置", "Reset"],
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
    "这是本地设置，不是 LeetCode 官方资源限额。内存包含输入、包装器和运行时。运行时每 5 ms 采样 RSS；macOS 另合并退出时的内核 RSS 高水位。采样可能漏掉短暂峰值，不等同于算法额外空间。",
    "These are local settings, not LeetCode's official limits. Memory includes input, harness and runtime. RSS is sampled every 5 ms; macOS also merges the kernel-reported exit RSS high-water mark. Sampling can miss short peaks; this is not algorithmic extra space.",
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
  peakMemory: ["观测峰值 {m} MiB", "observed peak {m} MiB"],
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
  confirmKeyTitle: ["删除 {p} 密钥？", "Delete the {p} key?"],
  confirmKey: [
    "将从 .env 删除该密钥。历史记录和其他 Provider 的密钥会保留。",
    "This removes the key from .env. History and the other provider's key are kept.",
  ],
  confirmHistoryTitle: ["清空技术深挖历史？", "Clear deep-dive history?"],
  confirmHistory: [
    "将删除所有技术深挖会话、反馈和缓存题目，无法恢复。LC 数据和密钥会保留。",
    "This deletes all deep-dive sessions, feedback and cached questions and cannot be undone. LeetCode data and keys are kept.",
  ],
  confirmSessionTitle: ["删除当前面试？", "Delete this interview?"],
  confirmSession: [
    "将删除这次面试及其反馈，无法恢复。缓存题目会保留。",
    "This deletes the interview and its feedback and cannot be undone. Cached questions are kept.",
  ],
  confirmDelete: ["确认删除", "Delete"],
  cancel: ["取消", "Cancel"],
  topics: ["训练主题", "Topics"],
  topicCount: ["{n} 个", "{n}"],
  topicSearch: ["搜索主题", "Search topics"],
  topicSearchHint: [
    "标题、知识点或场景关键词",
    "Title, concept or scenario keyword",
  ],
  topicCategory: ["主题分类", "Topic category"],
  topicAllCategories: ["全部分类", "All categories"],
  topicMatches: [
    "显示 {n} / {total} 个主题 · 随机选择遵循筛选",
    "Showing {n} / {total} topics · random respects filters",
  ],
  topicClearFilters: ["清除筛选", "Clear filters"],
  topicNoMatches: [
    "没有匹配主题，试试其他关键词或清除筛选。",
    "No matching topics. Try another keyword or clear filters.",
  ],
  topicCategory_Caching: ["缓存", "Caching"],
  topicCategory_Correctness: ["正确性", "Correctness"],
  topicCategory_Messaging: ["消息队列", "Messaging"],
  topicCategory_Concurrency: ["并发", "Concurrency"],
  topicCategory_Consistency: ["一致性", "Consistency"],
  topicCategory_Availability: ["可用性", "Availability"],
  topicCategory_Scalability: ["扩展性", "Scalability"],
  topicCategory_Performance: ["性能", "Performance"],
  topicCategory_Database: ["数据库", "Database"],
  topicCategory_Networking: ["计算机网络", "Networking"],
  "topicCategory_Operating Systems": ["操作系统", "Operating Systems"],
  topicCategory_Runtime: ["语言与运行时", "Languages and Runtime"],
  topicCategory_Redis: ["Redis", "Redis"],
  topicCategory_Security: ["安全基础", "Security"],
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
    "先回答当前问题，再解释涉及的机制或操作过程…",
    "Answer the current question, then explain the mechanism or execution steps…",
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
  rating_missing: ["独立证据不足", "Insufficient evidence"],
  rating_incorrect: ["有误", "Incorrect"],
  rating_uncertain: ["待确认", "Uncertain"],
  dim_principles: ["原理理解", "Principles"],
  dim_selection_rationale: ["技术选型依据", "Selection rationale"],
  dim_tradeoffs: ["替代方案权衡", "Trade-offs"],
  dim_failure_and_correctness: ["故障边界与正确性", "Failure and correctness"],
  dim_communication: ["沟通清晰度", "Communication"],
  liveHide: ["收起", "Hide"],
  liveShow: ["展开", "Show"],
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
