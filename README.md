# Someday · The Curator

个人高效待办事项桌面应用 — 基于 Aeon Minimalist 设计系统。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.x-2c3e50?logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.5-42b883?logo=vue.js)

## 产品介绍

Someday 是一款专注个人的待办事项管理应用，采用极简主义设计风格，帮助你通过直观的日历视图、项目管理和任务追踪，提升个人工作效率。

**核心特性：**
- 📅 **日历视图** — 月/周视图切换，点击日期快速添加任务
- 📁 **项目管理** — 个人项目卡片式管理，自动追踪进度
- 📦 **归档系统** — 已完成任务自动归档，支持搜索和恢复
- 🎨 **Aeon Minimalist** — 深蓝色调、Manrope + Inter 字体、无边框设计

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3.5 (Composition API) |
| 语言 | TypeScript |
| 构建工具 | Vite 8 |
| 状态管理 | Pinia |
| 路由 | Vue Router 4 |
| 桌面框架 | Tauri 2.x (Rust) |
| 样式 | Tailwind CSS v4 |
| 图标 | Material Symbols Outlined |

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.77+ (仅打包桌面应用需要)
- Windows: MinGW-w64 或 MSVC 工具链 (仅打包需要)

### 安装运行

```bash
# 进入项目目录
cd someday

# 安装依赖
npm install

# 启动前端开发服务器
npm run dev
```

访问 `http://localhost:5173` 查看应用。

### 打包桌面应用

```bash
# 安装 Rust (如尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 打包 .exe
npm run tauri build
```

可执行文件位于 `src-tauri/target/release/bundle/`。

## 项目结构

```
someDay/
├── src/                      # Vue 前端源代码
│   ├── components/           # Vue 组件
│   │   ├── common/          # 通用组件 (Button, Input, Modal)
│   │   ├── layout/          # 布局组件 (Sidebar, Layout)
│   │   ├── tasks/           # 任务相关组件
│   │   ├── calendar/        # 日历组件
│   │   ├── projects/       # 项目相关组件
│   │   └── archive/         # 归档组件
│   ├── views/               # 页面视图
│   ├── stores/              # Pinia 状态管理
│   ├── types/               # TypeScript 类型定义
│   ├── router/              # Vue Router 配置
│   └── styles/              # 全局样式
├── src-tauri/               # Tauri/Rust 后端
│   └── src/lib.rs           # Rust 命令定义
├── stitch_todo_list/        # UI 原型与设计规范
│   ├── prd.html             # 产品需求文档
│   ├── aeon_minimalist/     # 设计系统文档
│   └── _*/                  # 迭代原型屏幕
└── dist/                    # 构建产物
```

## 界面预览

### 设计系统

应用遵循 Aeon Minimalist 设计规范：

- **色彩**: 深蓝色调 (`#24389c` primary, `#3f51b5` primary-container)
- **字体**: Manrope (标题) + Inter (正文)
- **阴影**: Cloud Shadow `0 12px 40px rgba(25, 28, 30, 0.06)`
- **圆角**: 卡片 8px / 按钮 12px
- **无边框**: 通过背景色层级区分区块

### 导航结构

左侧固定边栏 (256px)：
```
控制面板 → 日历 → 项目 → 统计 → 归档 → 设置
```

## 数据说明

当前版本数据存储在内存中，关闭应用后数据将重置。

如需持久化存储，可自行实现：
- `src-tauri/src/lib.rs` 中的 Tauri 命令
- 使用 `tauri-plugin-store` 或 `tauri-plugin-sql` 存储到本地文件/SQLite

## 参考资源

- [Vue 3 文档](https://vuejs.org/guide/introduction.html)
- [Tauri 2 文档](https://v2.tauri.app/)
- [Tailwind CSS v4 文档](https://tailwindcss.com/)
- [date-fns 日期处理](https://date-fns.org/)

## License

MIT
