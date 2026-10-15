import type { Lang } from "./i18n";

// The Rust backend reports errors in Chinese. In English mode the UI translates
// them here, so keep this table in sync when adding or rewording a backend error.
const EXACT: [string, string][] = [
  // code_runner.rs
  ["此平台尚未支援記憶體監測，無法按題目限制判題", "Memory monitoring is not supported on this platform, so problem limits cannot be enforced"],
  ["無法讀取執行進程的記憶體用量；已終止進程，未繞過記憶體限制", "Could not read the process's memory usage; it was stopped so the memory limit is not bypassed"],
  ["未找到可用的 clang++ / g++，请指定编译器可执行文件的完整路径（不含参数）", "No usable clang++ / g++ found. Enter the full path of the compiler executable (without arguments)"],
  ["無效的題目執行限制", "Invalid problem limits"],
  ["代码长度超过 256 KiB", "Code is longer than 256 KiB"],
  // content.rs, db.rs
  ["未知题目", "Unknown problem"],
  ["未知编程模式", "Unknown coding mode"],
  ["无效的草稿", "Invalid draft"],
  // credentials.rs
  ["未知凭据供应商", "Unknown credential provider"],
  ["无法初始化系统凭据存储", "Could not initialize the system credential store"],
  ["API Key 格式无效，请检查输入", "Invalid API key format, please check it"],
  ["无法写入系统凭据存储，请检查钥匙串权限", "Could not write to the system credential store; check Keychain permissions"],
  ["尚未保存此供应商的 API Key，请在设置中配置", "No API key saved for this provider; set one in settings"],
  ["无法读取系统凭据存储，请解锁钥匙串并检查访问权限", "Could not read the system credential store; unlock the Keychain and check access"],
  ["无法从系统凭据存储删除 Key", "Could not delete the key from the system credential store"],
  ["Key 格式无效（只接受字母、数字、连字符和下划线）", "Invalid key format (only letters, digits, hyphens and underscores)"],
  ["无法读取项目 .env，请先配置 API Key", "Could not read the project .env; set an API key first"],
  [".env 格式错误，请检查配置（未输出文件内容）", ".env is malformed; check it (contents not shown)"],
  [".env 中尚未配置此 Provider 的 Key", "No key for this provider in .env"],
  ["无法读取 .env，未覆盖文件", "Could not read .env; the file was not overwritten"],
  [".env 路径无效", "Invalid .env path"],
  ["无法创建 .env 临时文件", "Could not create a temporary .env file"],
  ["无法设置 .env 权限", "Could not set .env permissions"],
  ["写入 .env 失败", "Writing .env failed"],
  ["同步 .env 失败", "Syncing .env failed"],
  ["无法原子替换 .env，原文件保留", "Could not replace .env atomically; the original file is kept"],
  ["Key 不可为空", "Key cannot be empty"],
  ["P1 支持 DeepSeek 和 Claude；其他供应商的已存凭据会保留", "Only DeepSeek and Claude are supported; saved keys for other providers are kept"],
  ["模型名称或超时设置无效（15–120 秒）", "Invalid model name or timeout (15–120 seconds)"],
  ["模型配置损坏", "Model settings are corrupted"],
  ["尚未配置", "Not configured"],
  // interview.rs
  ["未知 Deep Dive 主题", "Unknown Deep Dive topic"],
  ["生成题目字段为空、过长或缺少约束/检查点，请重试", "The generated question has empty or overlong fields, or lacks constraints/checkpoints; please retry"],
  ["追问观察为空或超过6000字符，请重试", "The follow-up observation is empty or longer than 6000 characters; please retry"],
  ["追问问题为空或超过6000字符，请重试", "The follow-up question is empty or longer than 6000 characters; please retry"],
  ["参考答案为空、过长或缺少要点，请重试", "The reference answer is empty, too long, or lacks key points; please retry"],
  ["反馈总结为空或过长，请重试", "The feedback summary is empty or too long; please retry"],
  ["反馈维度的依据或改进建议为空或过长，请重试", "A feedback dimension has empty or overlong evidence or advice; please retry"],
  ["反馈优点或不足列表无效，请重试", "The feedback strengths or gaps list is invalid; please retry"],
  ["反馈必须提供至少一个后续练习方向，且不超过12项，请重试", "Feedback must suggest 1–12 next practice topics; please retry"],
  // interview_store.rs
  ["缓存题目不存在", "Cached question not found"],
  ["本地题目数据损坏", "Local question data is corrupted"],
  ["面试会话不存在", "Interview session not found"],
  ["本地参考答案索引无效", "Invalid local reference-answer index"],
  ["本地参考答案数据损坏", "Local reference-answer data is corrupted"],
  ["回答最多 6000 字符", "Answers can be at most 6000 characters"],
  ["会话不存在或正在等待模型，请先重试待处理请求", "The session does not exist or is waiting for the model; retry the pending request first"],
  ["回答需要 1–6000 字符", "Answers must be 1–6000 characters"],
  ["当前已有待处理请求或会话已结束，请使用重试", "A request is pending or the session has ended; use Retry"],
  ["已达 20 轮上限，请结束并生成反馈", "Reached the 20-round limit; finish and generate feedback"],
  ["已达20轮上限，请结束并生成反馈", "Reached the 20-round limit; finish and generate feedback"],
  ["请先提交或清空当前草稿，再使用 AI 代答", "Submit or clear the current draft before using the AI answer"],
  ["请先完成待处理追问或反馈", "Finish the pending follow-up or feedback first"],
  ["有尚未提交的回答，请先提交或清空草稿", "There is an unsubmitted answer; submit or clear the draft first"],
  ["至少提交一轮回答后再生成反馈", "Submit at least one answer before generating feedback"],
  ["会话没有待重试请求", "The session has no request to retry"],
  ["只能为原题或已存在的面试官追问生成参考答案", "Reference answers can only be generated for the question or an existing follow-up"],
  ["请先完成待处理的追问或反馈，再生成参考答案", "Finish the pending follow-up or feedback before generating a reference answer"],
  ["请先重试待处理请求；已结束的会话不能继续作答", "Retry the pending request first; finished sessions cannot take more answers"],
  ["当前问题已变化，请刷新会话后再使用 AI 代答", "The current question changed; refresh the session before using the AI answer"],
  // llm.rs
  ["供应商返回无效 JSON，可重试；本地回答已保留", "The provider returned invalid JSON; you can retry, your answer is saved locally"],
  ["模型拒绝了此次请求，请调整回答或主题后重试", "The model refused this request; adjust the answer or topic and retry"],
  ["模型未完整生成结果（可能达到输出限制或被拒绝），请重试", "The model did not finish its output (output limit or refusal); please retry"],
  ["模型未完整生成结果（可能达到输出限制），请重试", "The model did not finish its output (it may have hit the output limit); please retry"],
  ["Claude 响应缺少内容", "The Claude response has no content"],
  ["模型返回空内容，请重试", "The model returned empty content; please retry"],
  ["模型 JSON 不符合约定结构，请重试；本地回答已保留", "The model's JSON does not match the expected structure; please retry, your answer is saved locally"],
  ["反馈必须包含五个不同评估维度，请重试", "Feedback must contain five different dimensions; please retry"],
  ["反馈维度结构无效", "Invalid feedback dimension structure"],
  ["反馈维度包含非约定字段，请重试", "A feedback dimension has unexpected fields; please retry"],
  ["无法初始化 HTTPS 客户端", "Could not initialize the HTTPS client"],
  ["API 整体请求超时，请重试；本地回答已保留", "The API request timed out overall; please retry, your answer is saved locally"],
  ["API 请求超时，请重试；本地回答已保留", "The API request timed out; please retry, your answer is saved locally"],
  ["无法连接 API（网络、代理或 TLS 错误），请检查后重试", "Could not reach the API (network, proxy or TLS error); check and retry"],
  ["API Key 无效、无权访问或模型不可用，请检查设置", "The API key is invalid, lacks access, or the model is unavailable; check settings"],
  ["API 账户余额不足，请在供应商控制台检查", "The API account balance is too low; check the provider console"],
  ["API 请求限流，请稍后手动重试", "The API is rate limiting requests; retry later"],
  ["API 拒绝请求，请检查模型名称和参数兼容性", "The API rejected the request; check the model name and parameters"],
  ["API 服务暂时不可用，请稍后重试", "The API service is temporarily unavailable; retry later"],
  ["API 请求未成功（重定向也不会自动跟随），请检查设置后重试", "The API request failed (redirects are not followed); check settings and retry"],
  ["API 响应读取超时，请重试；本地回答已保留", "Reading the API response timed out; please retry, your answer is saved locally"],
  ["读取 API 响应失败，可重试", "Reading the API response failed; you can retry"],
  ["API 响应超过 1 MiB 限制，请重试", "The API response exceeded 1 MiB; please retry"],
  ["API 响应超过大小限制，请重试", "The API response exceeded the size limit; please retry"],
  ["无法初始化请求执行器", "Could not initialize the request runtime"],
  // main.rs
  ["此平台尚未支援開啟官方連結", "Opening the official link is not supported on this platform"],
  ["無法開啟官方題面", "Could not open the official problem page"],
  ["已有判题任务正在运行", "A judge run is already in progress"],
  ["后台面试任务意外终止，请重试", "The background interview task stopped unexpectedly; please retry"],
  ["请等待当前面试请求完成", "Wait for the current interview request to finish"],
  ["已有面试请求正在运行", "An interview request is already running"],
  ["启动 Tauri 失败", "Failed to start Tauri"],
];

const PATTERNS: [RegExp, string][] = [
  [/无法执行 (.+?): /g, "Could not run $1: "],
  [/反馈必须包含五个不同评估维度（收到 (\d+) 项、(\d+) 个不同名称），请重试/g, "Feedback must contain five different dimensions (got $1 items, $2 distinct names); please retry"],
];

// Longest first, so a message is never partly replaced by a shorter one it contains.
const SORTED = [...EXACT].sort((a, b) => b[0].length - a[0].length);

/** Translates the backend's Chinese error text (also when embedded in a longer message). */
export function localizeError(message: string, lang: Lang): string {
  if (lang !== "en") return message;
  let out = message;
  for (const [re, en] of PATTERNS) out = out.replace(re, en);
  for (const [zh, en] of SORTED) out = out.split(zh).join(en);
  return out;
}
