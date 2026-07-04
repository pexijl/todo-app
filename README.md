# Todo App · Tauri 学习项目

> 个人用来学习 [Tauri](https://tauri.app) 的练手项目，基于官方 `create-tauri-app` 模板改造而来。
> 前端是 React 19 + TypeScript + Vite，桌面壳子用 Tauri v2，后端用 Rust。

目前后端只暴露了一个最基础的 `greet` 命令，前端展示一个简单的「输入名字 → 弹出招呼语」的页面，整体用作 Tauri 开发、调试、打包全流程的练习载体。

## 技术栈

- **Tauri 2** —— 跨平台桌面应用壳（基于系统 WebView）
- **Rust**（`edition = "2021"`）—— 后端逻辑、命令注册
- **React 19** + **TypeScript 5.8** —— 前端 UI
- **Vite 7** —— 前端开发服务器与构建工具
- **tauri-plugin-opener** —— 用于在系统默认浏览器里打开链接

## 项目结构

```text
todo-app/
├── index.html                # 前端入口 HTML
├── package.json              # 前端依赖与脚本
├── vite.config.ts            # Vite 配置（端口 1420，固定）
├── tsconfig.json             # TS 配置
├── public/                   # 静态资源（vite.svg、tauri.svg 等）
├── src/                      # 前端源码
│   ├── main.tsx              # React 挂载入口
│   ├── App.tsx               # 主页面，调用 `greet` 命令
│   ├── App.css               # 页面样式
│   └── assets/               # 图片等资源
└── src-tauri/                # Tauri / Rust 端
    ├── Cargo.toml            # Rust 依赖
    ├── tauri.conf.json       # Tauri 应用配置
    ├── build.rs
    ├── capabilities/
    │   └── default.json      # 权限声明（core + opener）
    ├── icons/                # 多平台应用图标
    └── src/
        ├── main.rs           # 二进制入口（release 隐藏控制台）
        └── lib.rs            # `greet` 命令与 `run()` 入口
```

## 环境准备

在开始之前，请先按照 Tauri 官方文档安装系统级依赖（WebView2、MSVC、CMake 等）：

- Tauri v2 前提条件：<https://v2.tauri.app/start/prerequisites/>

需要本机具备：

- **Node.js** ≥ 18（推荐 20+）
- **Rust** stable（`rustup` 安装即可）
- **pnpm / npm / yarn** 任选一个

## 安装与运行

```bash
# 1. 安装前端依赖
npm install

# 2. 启动 Tauri 开发模式（会自动跑 Vite 并打开桌面窗口）
npm run tauri dev
```

> 开发模式下，Vite 监听 `http://localhost:1420`，Tauri 窗口会加载该地址。修改 `src/` 下文件会触发前端热更新，修改 `src-tauri/src/` 下文件会触发 Rust 端重新编译。

### 其他常用脚本

```bash
# 仅启动前端（不打开桌面窗口），用于纯 UI 调试
npm run dev

# 类型检查 + 打包前端资源到 dist/
npm run build

# 预览 dist/ 构建产物
npm run preview
```

## 打包发布

```bash
# 在当前平台下生成可分发安装包（exe / dmg / deb 等）
npm run tauri build
```

打包产物会输出到 `src-tauri/target/release/bundle/`，包含当前系统对应的安装包与免安装可执行文件。
首次执行时 Rust 会编译大量依赖，需要耐心等待。

## 已经实现的功能

- [x] 基础 Tauri v2 工程结构（前端 + Rust 端 + 权限声明）
- [x] 一个最简 `greet` 命令：
  - Rust 端：`src-tauri/src/lib.rs` 中的 `greet(name: &str) -> String`
  - 前端调用：`src/App.tsx` 中通过 `invoke("greet", { name })` 调用
  - 响应示例：`Hello, <name>! from 瓦夏曼波`
- [x] 集成 `tauri-plugin-opener`（点击页面里的 Tauri/Vite/React logo 会通过它打开外链）

## 后续学习计划

下面是打算逐步往里加、用来练手的方向：

- [ ] **状态持久化**：用 `tauri-plugin-store` 或本地 SQLite 保存 Todo 数据
- [ ] **系统能力**：调用 `tauri-plugin-dialog` / `tauri-plugin-notification` / `tauri-plugin-fs`
- [ ] **窗口与托盘**：自定义窗口行为 + 系统托盘菜单
- [ ] **多窗口 / Webview 通信**：主窗口与子窗口之间传消息
- [ ] **跨平台打包**：在 Windows / macOS / Linux 上分别打安装包
- [ ] **自动更新**：研究 `tauri-plugin-updater` 接入

## 调试小贴士

- **Tauri DevTools**：开发模式下右键 → 「检查元素」可以打开 WebView 开发者工具
- **Rust 日志**：`src-tauri/src/lib.rs` 里 `println!` 的内容会打到终端
- **权限不足报错**：在 `src-tauri/capabilities/default.json` 里补上对应插件的 `permissions`
- **缓存依赖问题**：Rust 端出怪问题时可尝试 `cd src-tauri && cargo clean`

## 推荐的 IDE 配置

- [VS Code](https://code.visualstudio.com/)
  - [Tauri 扩展](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- 或 [RustRover](https://www.jetbrains.com/rust/) / [CLion](https://www.jetbrains.com/clion/) + 官方 Rust 插件

## 相关文档

- Tauri v2 官方文档：<https://v2.tauri.app/>
- Tauri 命令（前端调用 Rust）：<https://v2.tauri.app/develop/calling-rust/>
- Tauri 权限系统（Capabilities）：<https://v2.tauri.app/security/capabilities/>
- Vite 文档：<https://vite.dev/>
- React 文档：<https://react.dev/>

## 许可证

仅作为个人学习使用，未指定开源协议。
