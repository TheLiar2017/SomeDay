# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## Project Overview

This repository contains two related projects:

1. **Someday** (`someday/`) — A personal desktop todo-list application (Tauri + Vue 3)
2. **stitch_todo_list/** — UI prototypes and design specifications

## Someday — Tauri + Vue Desktop Application

### Tech Stack

- **Frontend:** Vue 3.5 + TypeScript + Vite 8 + Pinia + Vue Router 4
- **Desktop:** Tauri 2.x (Rust backend)
- **Styling:** Tailwind CSS v4 with CSS-first configuration
- **Icons:** Material Symbols Outlined
- **Fonts:** Manrope (headlines) + Inter (body)
- **Date handling:** date-fns

### Commands

```bash
cd someday

# Frontend development
npm run dev          # Start Vite dev server (http://localhost:5173)
npm run build        # Build frontend for production
npm run preview      # Preview production build

# Desktop app (requires Tauri CLI and Rust)
npm run tauri dev    # Run desktop app in development
npm run tauri build  # Build .exe installer
```

### Architecture

```
someday/src/
├── components/
│   ├── common/      # AppButton, AppInput, AppModal, AppCheckbox
│   ├── layout/       # AppSidebar (fixed 256px), AppLayout
│   ├── tasks/        # TaskCard, TaskList, TaskItem, TaskCreateModal
│   ├── calendar/     # MonthCalendar, WeekCalendar, DayColumn
│   ├── projects/     # ProjectCard, ProjectList
│   └── archive/      # ArchiveList
├── views/           # Dashboard, Calendar, Projects, Archive, Settings
├── stores/           # Pinia: taskStore, projectStore, archiveStore, settingsStore, uiStore
├── types/            # TypeScript: task.ts, project.ts, settings.ts
├── router/
└── styles/           # main.css (Tailwind v4 CSS-first config)
```

**State management:** Pinia stores connect to Tauri commands (Rust IPC). Task/Project CRUD operations invoke `@tauri-apps/api/core` `invoke()` calls.

**Rust backend:** `src-tauri/src/lib.rs` defines Tauri commands for task/project CRUD, archive operations, and settings — all in-memory with `Mutex<Vec<>>` for now.

### Design System (Aeon Minimalist)

- **Typography:** Manrope (display/headlines) + Inter (body/labels)
- **Colors:** Deep blues — primary `#24389c`, primary_container `#3f51b5`
- **No borders:** Sectioning via background color shifts (`surface-container` tiers)
- **Cloud Shadow:** `box-shadow: 0 12px 40px rgba(25, 28, 30, 0.06)`
- **Border radius:** `xl` (8px) for cards, `full` (12px) for buttons

### Navigation

Fixed left sidebar (256px): Dashboard → Calendar → Projects → Stats → Archive → Settings

---

## Stitch Todo List — HTML Prototypes

```
stitch_todo_list/
├── prd.html              # Product Requirements Document (Chinese)
├── aeon_minimalist/
│   └── DESIGN.md         # Design System specification
├── _1/ through _8/        # Iterative UI prototype screens
│   ├── code.html         # Standalone Tailwind HTML (CDN)
│   └── screen.png        # Visual reference
```

- Pure static HTML, no build step
- Reference for UI patterns and design decisions

## Key Files

- `someday/src-tauri/tauri.conf.json` — Tauri app configuration
- `someday/src/styles/main.css` — Tailwind v4 CSS-first theme definitions
- `stitch_todo_list/aeon_minimalist/DESIGN.md` — Full design system spec
- `stitch_todo_list/prd.html` — Product requirements document

## Development Notes

- Tauri build requires Rust toolchain + MinGW or MSVC on Windows
- Frontend-only development: `npm run dev` in `someday/` works without Tauri
- The `dist/` folder contains the last built frontend assets
