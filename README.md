# SWE Interview Prep · P0 + P1

本地 SWE 面试准备桌面应用。P0 的双模式本地 Judge 已完成；P1 的 Technical Deep Dive 已接入 Claude / DeepSeek，包含缓存和可恢复面试流程。Claude Haiku 5.5 已完成真实出题、追问、反馈和失败恢复验证；DeepSeek 真实多轮验收仍受 503 / 保活超时影响。P2 系统设计与 P3 完整 Hot 100 尚未实现。

## 启动

开发依赖：Node.js 22.12+、npm、Rust stable、平台对应的 [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)，以及支持 C++20 的 `clang++` 或 `g++`。macOS 使用 Xcode Command Line Tools 即可提供 C++ 编译器。

```sh
npm ci
npm run tauri dev
```

`npm run dev` 仅供前端开发，不提供本地 Judge；必须在 Tauri 窗口中运行。本应用没有 Node/Express 业务服务器。

本次在开发机上安装了临时 Rust 工具链（没有修改 shell 配置）。若尚未正常安装 Rust，当前机器可使用：

```sh
export PATH="/tmp/intv-cargo/bin:$PATH"
export CARGO_HOME=/tmp/intv-cargo
export RUSTUP_HOME=/tmp/intv-rustup
npm run tauri dev
```

`/tmp` 可能被系统清理；长期开发请按官方先决条件安装 Rust。

## 构建与测试

```sh
npm run build                  # TypeScript 检查 + Vite 生产构建
npm test                       # Judge / SQLite / 面试状态 / HTTP 错误恢复测试，不调用收费 API
npm run tauri build -- --bundles app    # macOS 发布构建（本地未签名 .app）
```

macOS 应用输出到 `src-tauri/target/release/bundle/macos/SWE Interview Prep.app`；可直接双击打开，不再需要 Node 或 Rust，也不依赖 Vite 服务。C++ 编译器仍需安装。调试打包使用 `npm run tauri build -- --debug --bundles app`，输出在 `src-tauri/target/debug/bundle/macos/`。

当前在 macOS Apple Silicon 上验证。进程组终止和资源限制使用 Unix API；尚未实现或验证 Windows 的进程树终止，不应将本版本视为 Windows 可用版本。

测试会真实调用本机编译器，不使用编译、执行或 Accepted 的 Mock。`src-tauri/tests/p0.rs` 覆盖：

- 4 道题的 Function / ACM 全部本地用例，C++20 和含空格的编译器路径。
- 编译错误、错误答案、空输出、非零退出、崩溃、无限循环、stdout/stderr 输出超限。
- 多个合法答案、非法下标、格式错误、空链表的比较。
- 每题独立时间/内存/输出设置；Function/ACM 真实分配并触碰内存后被终止，返回 Memory Limit Exceeded；四题最大规模用例。
- 后台子进程终止、继承管道清理、SQLite 关闭重开后的草稿/状态/结果/设置恢复。

## P0 验证（2026-10-15）

- macOS Apple Silicon：TypeScript/Vite 构建、Tauri 桌面构建及 `.app` 打包成功。
- 11 项自动化集成测试通过；4 道题 × 2 种模式，合计执行 50 个语义测试用例，全部调用真实 C++20 编译器。
- 实际桌面窗口：空实现显示错误答案；两数之和 Function/ACM 分别提交后均显示 5/5；退出重启后草稿、完成状态、编译器设置及最近提交结果恢复。
- 限制功能更新后，实际桌面两数之和 Function 提交显示 6/6、当次上限 1000 ms / 128 MiB，并显示每例真实 RSS 采样峰值。
- 判题测试在无网络权限的执行环境通过，打包资源使用本地 `tauri://localhost` 加载；没有修改整机网络设置做物理断网测试。

## 使用

1. 可自动探测 `clang++` / `g++`，也可在顶部填写可执行文件的完整路径并点击“检测并保存”。路径不含额外编译参数。检测验证程序可以启动；真正判题以 `-std=c++20 -O0` 编译，诊断完整显示。
2. 选择题目，查看题意、约束、逻辑样例和 ACM 输入输出协议。可搜索标题/编号/标签、筛选难度和标签。
3. Function 模式只写 `class Solution`，系统提供头文件、`main()` 和序列化包装。链表题额外提供 `ListNode`。ACM 模式自行编写完整程序，严格遵守题目协议。
4. “运行样例”执行标记为样例的前 2 个用例；“提交本地测试”执行该题全部本地测试。仅提交全部通过会标记该模式“完成”。已完成状态作为练习记录保留，后续编辑/错误提交不会抹除；最近提交结果会更新。
5. 每次编辑自动写入 SQLite。等待“草稿已保存”后退出；两种模式分别保存。样例结果仅在当前窗口显示，最近一次提交结果可在重启后恢复。结果对应上次执行代码，修改后需重新判题。

内置题目（共 25 个自行整理的语义测试，两个模式共享）：

| 题目                 | 包装 / 比较重点                                     |
| -------------------- | --------------------------------------------------- |
| 两数之和             | 数组参数 + target；验证任意合法下标对               |
| 无重复字符的最长子串 | 整行字符串，保留空格，包含空串                      |
| 反转链表             | 根据值序列构建链表、返回链表序列化、检测结果环/过长 |
| 移动零               | 检查 void 方法原地修改后的数组                      |

题面是自行撰写的等价题意，测试是本地维护的小型测试集，不是 LeetCode 官方隐藏测试，不能据此宣称官方评测通过或完整边界覆盖。

## P1：Technical Deep Dive

按用户指定，当前使用 **Claude Haiku 5.5**（`claude-haiku-5-5`），保留 DeepSeek 接口和独立密钥，可在设置中切换。未实现 System Design。

- 内置 10 个主题：缓存选型、热点与缓存故障、幂等与重试、消息可靠投递、队列选型、限流/熔断、并发事务、副本故障转移、分片扩容、RPC 尾延迟。
- 选择主题 → 显式联网生成新题 → SQLite 缓存 → 新建面试 → 提交回答并动态追问 → 结束并生成五维定性反馈。
- 可随机选择本地主题、复用已缓存问题、查看历史和恢复未完成会话；这些操作不调用 API。
- 面试官内部的追问方向和检查点不会随题目/会话 DTO 发给前端；反馈只在结束后显示，不做百分制评分。
- 一次最多 20 轮回答，每条最多 6000 字符。追问只发送当前题目和最近 12 条消息；反馈发送本会话完整记录，不发送其他会话或题库。
- 模型、超时（15–120 秒）可设置。会话使用创建时的模型，之后改设置不会悄悄改变已有会话的模型。
- 失败后保留内容并提供手动重试，不做自动收费重试。超时涵盖整次请求，持续空白保活不会重置截止时间；响应上限 1 MiB。

使用方式：打开顶部 **Technical Deep Dive**，检查模型设置。已有 `.env` 时直接使用；没有时可通过应用内密码输入框保存，或先从 `.env.example` 创建 `.env` 再填写（不要覆盖已有 `.env`）。新配置默认模型 `claude-haiku-5-5`；已有 DeepSeek 配置保留，可在 Provider 下拉框切换并保存。已有会话仍使用创建时的 Provider 和模型。按钮明确标示哪些操作联网，并显示最近一次请求状态。

### 当前临时密钥策略（用户指定）

现在 `credentials::read_key/save_key/delete_key` **仅操作项目 `.env`，不会自动回退读取或写入 Keychain**。`.env` 已加入 `.gitignore`，写入使用原子替换，Unix 文件权限 0600；`.env.example` 只包含空值。按 Provider 分别使用 `DEEPSEEK_API_KEY`、`OPENAI_API_KEY`、`CLAUDE_API_KEY`，更新或清除一个不会覆盖另一个。请勿使用 `VITE_` 前缀保存密钥。实际 Key 不写入 SQLite、日志或前端返回值。

先前保存的 DeepSeek 系统凭据已按用户新要求转存并删除。系统凭据适配代码仍保留，**待真实验收通过后再迁移启用**；当前没有自动迁移操作。测试构建从编译时的项目根目录读取 `.env`，不依赖启动命令所在目录。这是临时开发方案，正式可分发构建前需要完成 Keychain 迁移。

### 失败恢复

回答先写入 SQLite 并进入 `awaiting_reply`，随后才调用 API。失败时，回答同时保留为本地消息和待处理草稿，重启后可重试，重试不会再追加一次候选人回答。结束请求同理进入 `feedback_pending`，只有完整反馈校验通过才标记 `finished`。有未提交草稿时不能直接结束，避免悄悄丢弃回答。

HTTP 401/403、余额不足、限流、服务错误、空响应、截断、无效 JSON、缺少字段和错误枚举均有可恢复错误提示；不回显供应商原始错误体或请求调试对象，避免它们带出凭据。生产请求只访问固定 Claude / DeepSeek HTTPS 端点且不跟随重定向。缓存/历史清理不会删除 LC 数据或密钥；清理需要在界面再次确认。

接口依据：[DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)。使用 JSON Output (`response_format: json_object`)，提示包含 JSON schema；Rust 使用严格 Serde 反序列化、字段长度/列表/枚举和五个维度检查再次验证，不假设供应商支持强制 JSON Schema 输出。

Claude 接口依据：[Haiku 5.5 迁移指南](https://platform.claude.com/docs/en/models/haiku-5-5/migration-guide)、[Messages API](https://platform.claude.com/docs/en/build-with-claude/working-with-messages)、[结构化输出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)。采用原生 `x-api-key` 认证、顶层 `system`、`output_config.format` JSON Schema 和 medium effort；按类型提取文本，不把 thinking 当答案。五个反馈维度使用必填对象字段保证覆盖，再转换为共享存储结构；拒绝、截断和缺项均不会标记成功。

当前两家 Provider 都使用 `stream: false`，尚无 SSE 流式显示；点击提交后整段发送输入。

### P1 验证与实际限制

- 26 项默认自动化测试通过：11 项 P0 回归、9 项 HTTP/JSON/整体超时及 Claude 协议测试、6 项面试存储和临时 `.env` 测试。
- 供应商故障测试使用明确的本机 HTTP 夹具；状态恢复测试使用明确的测试 Provider。这些结果不冒充真实 AI 的出题、追问或评价质量。
- 真实 macOS Keychain 独立测试在切换临时 `.env` 策略前已通过；该测试默认忽略，当前不再写入系统凭据。
- 真实账户的 `/models` 查询成功，返回 `deepseek-flash`、`deepseek-v4-pro`。`flash` 生成请求返回 HTTP 503；`pro` 曾返回 HTTP 200 但持续空白保活，未得到可验证的完成内容。桌面端已确认显示失败、解除忙碌状态并保留密钥设置。DeepSeek 完整真实面试仍待服务恢复后验收。
- Claude Haiku 5.5 最小真实请求成功（输入 14 / 输出 4 tokens），元数据记录 `/tmp/intv-claude-probe.json`。真实面试首次反馈返回空维度列表，被校验拦截；改为五个必填维度对象后，从原会话重试成功，保持 4 条消息且无重复候选人回答，并验证重开恢复与离线复用。随后在全新数据库 `/tmp/intv-p1-live-claude-final.sqlite3` 中完整执行真实出题 → 两轮追问 → 五维反馈 → 重开恢复 → 离线复用，全部通过。AI 技术判断仍需人工复核，不保证每次生成都正确。

重跑自动化测试不需要实际 Key（本机 HTTP 夹具需要 localhost 监听权限）：

```sh
npm test
```

下面是显式的真实联网验收，**会使用 `.env` 中的 Key 并产生 API 费用**，默认把测试数据写入 `/tmp/intv-p1-live.sqlite3`；不调用它就不会自动发起这些请求：

```sh
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_deep_dive
# 或指定账户另一可用模型及独立的测试数据库
INTERVIEW_PROVIDER=deepseek INTERVIEW_MODEL=deepseek-v4-pro cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example live_deep_dive -- /tmp/intv-p1-live-pro.sqlite3
```

单独验证系统凭据适配（当前测试阶段不必执行；只使用独立临时测试条目，不会覆盖实际 Key）：

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --test p1 os_keyring_roundtrip -- --ignored
```

## 每题复杂度要求和执行预算

所有题目的 JSON 均包含 `complexity`（时间、额外空间、官方要求、训练说明、来源）和 `limits`（`timeLimitMs`、`memoryLimitMb`、`outputLimitKb`）。题目详情同时显示，Judge 按题目设置执行，并在结果中保存当次预算、实测耗时和主进程内存采样峰值。复杂度要求适用于两种模式；ACM 解析输入使用的存储需单独说明。

| 题目           | 本地训练时间目标 | 额外空间要求 / 目标              | 官方依据                                                                                                                                 |
| -------------- | ---------------- | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 两数之和       | 平均 O(n)        | O(n)                             | [官方 follow-up](https://leetcode.com/problems/two-sum/) 要求优于 O(n²)，O(n log n) 排序方案也符合该 follow-up；哈希表是本地建议目标     |
| 最长无重复子串 | O(n)             | O(min(n, Σ))；固定 ASCII 时 O(1) | [官方题面](https://leetcode.com/problems/longest-substring-without-repeating-characters/) 未明文规定 Big-O；滑动窗口复杂度为本地训练目标 |
| 反转链表       | O(n)             | 迭代 O(1)，递归 O(n) 栈亦允许    | [官方 follow-up](https://leetcode.com/problems/reverse-linked-list/) 允许练习迭代和递归，不将递归误判为违反官方 O(1) 要求                |
| 移动零         | O(n)             | O(1)，原地修改，不复制数组       | [官方题面](https://leetcode.com/problems/move-zeroes/) 明确要求原地且不复制数组，优化操作次数是 follow-up                                |

查核日期：2026-10-15。上述官方公开题面未列出可核对的逐题固定毫秒/MiB 配额，因此 **1000 ms / 128 MiB 是本地预算，不冒充官方限额**。[官方编译环境说明](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages) 使用 `-O2`、C++23 和 AddressSanitizer；本项目按 P0 保留 C++20 / `-O0`，耗时不能与官方直接比较。

本地数据范围参考公开约束：字符串最长 100000，链表最多 5000 节点，数组最多 10000 元素；保留明确说明的本地变体（多个两数和答案、移动零空数组、ASCII 字符子集）。四题均加入最大规模、确定性预期值的自建用例。

时间测试和 RSS 预算不能证明任意代码的渐进复杂度或原地语义。进程 RSS 包含输入、C++ 运行库及系统包装，**不是算法额外空间**。例如复制数组后写回可能通过输出测试，但不满足移动零的原地要求；仍需审阅代码。页面会直接说明这个边界，不把资源达标显示为复杂度已认证。旧提交记录可能没有内存或预算字段，显示为旧记录/未取得；重新提交会执行新预算。

## 两数之和：双重循环复杂度回归

本地提交测试包含 `worst-case-10000`：`nums = [0, 1, ..., 9997, 100000, 200000]`，`target = 300000`，唯一合法答案是 `[9998, 9999]`（顺序不限）。它处于声明的 n ≤ 10000 约束内，且使 `for (i) for (j=i+1)` 在找到答案前扫描全部 n(n−1)/2 个候选对。该用例在 Function / ACM 提交中共享，不增加样例数量。

`src-tauri/tests/benchmarks/two_sum.cpp` 对两个已知实现计数并实测算法耗时，规模为 1000、2000、4000、10000。自动化测试校验计数；不把不稳定的毫秒比例当作断言，也不声称能自动识别任意用户代码的复杂度。单独运行：

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features two_sum_worst_case_and_complexity_profile -- --nocapture
```

本机一次真实 C++20 / `-O0` 运行（带计数器，算法耗时不含编译、输入生成和进程启动）：

| n     | 双循环候选对数 | 双循环耗时 | 哈希查找次数 | 哈希耗时 | 哈希峰值键数 |
| ----- | -------------: | ---------: | -----------: | -------: | -----------: |
| 1000  |         499500 |    1.97 ms |         1000 |  0.26 ms |          999 |
| 2000  |        1999000 |    6.91 ms |         2000 |  0.50 ms |         1999 |
| 4000  |        7998000 |   27.85 ms |         4000 |  1.04 ms |         3999 |
| 10000 |       49995000 |  169.02 ms |        10000 |  2.44 ms |         9999 |

双循环最坏时间 O(n²)，额外空间 O(1)；哈希表平均时间 O(n)，额外空间 O(n)（极端哈希冲突下时间可退化）。空间分析不计输入数组；测试中的辅助键数是结构规模指标，不是进程 RSS 或内存字节实测。

另以未插入计数器的完整双循环程序运行真实 Judge，此次为 Accepted / 383 ms（包含进程运行开销）。1 秒上限下不会保证 O(n²) 必然超时；机器、编译选项及实现均影响实际速度。正确性通过不等于复杂度最优，测试不会伪造 TLE。

## 结构和存储

```text
content/leetcode/problems/     题目、模板、逻辑测试 JSON
src/main.tsx                  LC 界面、Monaco、Tauri 调用
content/leetcode/adapters/     Function 模式 C++ main 包装器
src-tauri/src/content.rs       内置题库、输入适配、结果比较器
src-tauri/src/code_runner.rs   编译、子进程控制、判题
src-tauri/src/db.rs            SQLite 初始化与 LC 草稿/状态
src/features/interview/       Deep Dive 设置、选题、会话、反馈与历史界面
src-tauri/src/interview.rs     主题与严格响应类型、校验
src-tauri/src/interview_store.rs 题目缓存和可恢复面试状态
src-tauri/src/llm.rs           Claude / DeepSeek 请求、schema、整体超时与安全错误
src-tauri/src/credentials.rs   临时 .env 凭据；保留待启用的系统适配
content/deep-dive/topics.json  10 个主题种子
src-tauri/src/main.rs          Tauri commands
src-tauri/tests/p0.rs          真实编译执行的集成测试
```

题库编译时嵌入 Rust，Monaco 和 worker 通过 Vite 打包，无运行时 CDN 或 LeetCode API。完成依赖安装和打包后，题目、编辑、判题和存储不需要外网。SQLite 保存在 Tauri 的应用数据目录，例如 macOS：

```text
~/Library/Application Support/local.swe.interview-prep/prep.sqlite3
```

用户进度和面试数据保存在应用数据目录。LC 不需要 `.env` 或 API Key。按当前测试要求，Claude / DeepSeek Key 暂存在项目根目录 `.env`，模型名称和超时等非敏感设置保存在 SQLite；临时 Key 文件是这一阶段明确允许的例外。

执行边界：仅运行本人编写且可信的 C++ 代码。每次判题使用独立临时目录，自动清理；编译超时 30 秒/输出 256 KiB；每个测试的时间、内存、输出预算由题目 `limits` 读取（当前 1000 ms / 主进程 128 MiB / stdout + stderr 合计 64 KiB）。Unix 下运行在独立进程组，结束、超时、输出超限时终止整个组；设置 CPU、单文件大小、打开文件数和禁止 core dump 的资源限制。记忆体通过主进程 RSS 采样监测（macOS `proc_pidinfo`；Linux `/proc`），超过题目上限则终止进程组。采样间隔约 5 ms，快速释放/退出的瞬时峰值可能漏计；不合计派生进程内存，不是 OS 硬内存配额。持续无法读取活进程内存时会明确报错，不静默跳过。没有内存/网络/文件系统的强安全沙箱，不能用来执行不可信代码。判题 command 在后台线程执行，UI 不等待主线程上的编译进程。

## 后续工作（本次不实现）

- 密钥存储仍按用户指定保留临时 `.env` 测试模式；正式分发前启用系统凭据存储。DeepSeek 的真实多轮验收仍待服务恢复。
- P2：System Design 场景、阶段控制与解释性反馈。
- P3：逐题补齐并验证完整 Hot 100、更多适配器及比较规则、质量改进。
