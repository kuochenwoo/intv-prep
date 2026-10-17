# SWE 面试准备

> 已安装的 Release 版本使用运行时应用配置目录中的 `.env`，本指南中项目 `.env` 的说明适用于 Debug 开发版本。安装与当前授权说明见[发版准备](docs/RELEASING.md)和[预览版说明](docs/releases/v0.1.0-preview.6.md)。

[English](README.md) · 简体中文

一个用于软件工程师面试练习的本地桌面应用，基于 Tauri 2（Rust）和 React。

- **LC 本地判题：** 收录 LeetCode Hot 100 全部 100 道题，使用 C++20。每道题都可以用函数模式（只写 `class Solution`）或 ACM 模式（写完整程序）作答。测试用本机真实编译器离线运行，并限制时间、内存和输出。每道题都标注了时间和空间复杂度要求。
- **技术深挖：** AI 面试官（Claude 或 DeepSeek）生成场景题，针对你的回答追问，结束时给出五个维度的反馈。回答边生成边显示。可选的“AI 参考回答”会为当前问题给出一份简洁的参考答案。
- **界面：** 中英文可切换（右上角按钮），支持浅色和深色模式，使用 Meta 的 [Astryx](https://github.com/facebook/astryx) 设计系统。

题面、面试主题、AI 生成的题目与回答以及错误提示都会跟随所选语言切换。

## 环境要求

- Node.js 22.12+ 和 npm
- 稳定版 Rust，以及对应平台的 [Tauri 2 依赖](https://v2.tauri.app/start/prerequisites/)
- 支持 C++20 的编译器：`clang++` 或 `g++`。macOS 安装 Xcode Command Line Tools 即可。

## 运行

```sh
npm install
npm run tauri dev
```

`npm run dev` 只启动网页前端，无法调用编译器，请使用 Tauri 窗口。

## 构建与测试

```sh
npm run build                         # 类型检查并构建前端
npm test                              # Rust 测试：判题、全部题目的两种模式、存储、AI 客户端（不调用付费 API）
npm run tauri build -- --bundles app  # 未签名的 macOS .app
```

构建结果在 `src-tauri/target/release/bundle/macos/`。运行时仍需要本机安装 C++ 编译器。

## 发布新的预览版

以在 preview.5 之后发布 `0.1.0-preview.6` 为例：

### 1. 创建分支并更新版本号

开始修改前执行：

```sh
git switch main
git pull --ff-only origin main
git switch -c codex/preview-6
```

完成功能修改后，统一更新以下位置的应用版本号：

| 文件 | 修改位置 |
|---|---|
| `package.json` | 顶层 `version` |
| `package-lock.json` | 顶层 `version` 和 `packages[""].version` |
| `src-tauri/Cargo.toml` | `[package]` 下的 `version` |
| `src-tauri/Cargo.lock` | 仅 `name = "swe-interview-prep"` 对应的版本 |
| `src-tauri/tauri.conf.json` | 顶层 `version` |

将这些位置都设为 `0.1.0-preview.6`，不要修改依赖包的版本号。每次发布都应使用严格递增的新预发布版本号，不要复用已发布版本。如果改动已经在功能分支上，继续使用当前分支即可，不必再创建分支。

### 2. 编写 release notes 并检查配置

新建 `docs/releases/v0.1.0-preview.6.md`，描述实际改动和限制。项目的 release notes 使用英文，例如：

```markdown
# SWE Prep 0.1.0-preview.6

- Fix ...
- Improve ...

macOS 13+ Apple Silicon preview.
Ad-hoc signed; not Apple-notarized.
```

把示例条目替换成实际改动，更新 README 中指向预览版说明的链接，然后执行：

```sh
npm run check:release
```

### 3. 提交、推送并合并 PR

```sh
git diff
git add -p
git add docs/releases/v0.1.0-preview.6.md
git diff --cached
git commit -m "Prepare preview.6"
git push -u origin codex/preview-6
```

`git add -p` 用于逐项选择已跟踪文件的改动。除新增的 release notes 外，新增源码文件也需要单独暂存。不要提交 `.env`、`.release-keys` 或本地数据。

在 GitHub 点击 **Compare & pull request**，目标分支选择 `main`。检查改动、等待 CI 全部通过，然后点击 **Merge pull request**。如果使用了其他分支名，推送命令也应使用相应名称。

合并后，GitHub 会自动执行：

```text
main CI → 创建版本标签 → macOS 签名构建 → 附件校验 → 发布 prerelease → 同步更新源
```

在 **Actions → CI** 和 **Automatic preview release** 中查看结果即可。不需要手动打标签或点击发布，也不需要特殊 PR 标题、标签或新增 PAT；流程会复用已配置的更新签名密钥。普通 PR 不提升版本号时只运行 CI。手动推送标签的构建仍会先生成草稿。

保护条件和重试方法见[自动发版指南](docs/AUTOMATIC_RELEASES.md)，安装方式与签名密钥备份见[签名更新指南](docs/UPDATES.md)。首次安装或使用 preview.1 的用户，需要先手动安装一次支持更新的预览版。

## AI 设置

打开“技术深挖”，选择 Provider、模型和超时时间，然后在页面中保存 API Key，或者把 `.env.example` 复制为 `.env` 后填写。密钥只保存在项目的 `.env` 中（已被 Git 忽略，权限 0600），不会写入 SQLite，也不会发送给前端。默认模型是 `claude-haiku-5-5`。

生成题目、提交回答、AI 参考回答和生成反馈都会调用 API，可能产生费用。查看历史、复用缓存题目和保存草稿不联网。

## 数据

练习进度、草稿和面试记录保存在应用数据目录下的 SQLite 中。macOS 上的路径是：

```text
~/Library/Application Support/local.swe.interview-prep/prep.sqlite3
```

## 添加题目

每道题是 `content/leetcode/problems/` 下的一个 JSON 文件，其中的 `io` 描述决定了函数模式的包装代码、ACM 输入格式和答案检查方式。`build.rs` 会自动打包新文件。完整流程（生成脚本、参考解答、`verify_problem` 工具和内容规范）见 [tools/leetcode/README.md](tools/leetcode/README.md)（英文）。

## 目录结构

```text
src/                         React 界面（main.tsx = LC 判题，features/interview = 技术深挖，i18n.tsx = 界面文案）
src-tauri/src/               Rust：判题、题库、SQLite、LLM 客户端、Tauri 命令
src-tauri/tests/             集成测试与参考解答
content/leetcode/            题目 JSON 与自定义 C++ 包装代码
content/deep-dive/           面试主题
tools/leetcode/              题目生成脚本与编写指南
```

## 限制

- 判题程序以当前用户权限运行你自己的代码，不是安全沙箱，请勿运行不可信代码。
- 判题依赖 Unix 进程组和资源限制，支持 macOS 和 Linux，暂不支持 Windows。
- 时间和内存上限是本地设置，不是 LeetCode 官方限额。通过测试也不代表算法满足 Big-O 或原地要求。
- 部分大规模测试为控制应用体积，规模小于 LeetCode 官方上限，具体在每道题的说明中注明。
- AI 生成的题目和反馈可能有误，仅供练习参考。
