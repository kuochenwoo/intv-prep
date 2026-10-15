# SWE Interview Prep · P0

本地 SWE 面试准备桌面应用。目前仅实现 **P0：项目初始化 + LeetCode 双模式本地 Judge**。不包含 P1–P3 的 AI 面试、API Key、系统设计或完整 Hot 100。

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
npm test                       # Rust Judge / IO / SQLite 集成测试，不需要 GUI
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

## 本次验证（2026-10-15）

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
src-tauri/src/db.rs            SQLite 草稿、状态、最近提交和编译器设置
src-tauri/src/main.rs          Tauri commands
src-tauri/tests/p0.rs          真实编译执行的集成测试
```

题库编译时嵌入 Rust，Monaco 和 worker 通过 Vite 打包，无运行时 CDN 或 LeetCode API。完成依赖安装和打包后，题目、编辑、判题和存储不需要外网。SQLite 保存在 Tauri 的应用数据目录，例如 macOS：

```text
~/Library/Application Support/local.swe.interview-prep/prep.sqlite3
```

不在仓库或静态题库中写入用户数据。P0 不需要 `.env` 或 API Key；唯一设置是应用内编译器路径。

执行边界：仅运行本人编写且可信的 C++ 代码。每次判题使用独立临时目录，自动清理；编译超时 30 秒/输出 256 KiB；每个测试的时间、内存、输出预算由题目 `limits` 读取（当前 1000 ms / 主进程 128 MiB / stdout + stderr 合计 64 KiB）。Unix 下运行在独立进程组，结束、超时、输出超限时终止整个组；设置 CPU、单文件大小、打开文件数和禁止 core dump 的资源限制。记忆体通过主进程 RSS 采样监测（macOS `proc_pidinfo`；Linux `/proc`），超过题目上限则终止进程组。采样间隔约 5 ms，快速释放/退出的瞬时峰值可能漏计；不合计派生进程内存，不是 OS 硬内存配额。持续无法读取活进程内存时会明确报错，不静默跳过。没有内存/网络/文件系统的强安全沙箱，不能用来执行不可信代码。判题 command 在后台线程执行，UI 不等待主线程上的编译进程。

## 后续工作（本次不实现）

- P1：Technical Deep Dive 端到端 AI 面试、安全密钥存储、问题和会话缓存。
- P2：System Design 场景、阶段控制与解释性反馈。
- P3：逐题补齐并验证完整 Hot 100、更多适配器及比较规则、质量改进。
