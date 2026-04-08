# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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
npm run build        # Build frontend (runs vue-tsc type checks first)
npm run preview      # Preview production build

# Desktop app (requires Rust toolchain)
npm run tauri dev    # Run desktop app in development
npm run tauri build  # Build .exe installer
```

**Build note:** `npm run build` runs `vue-tsc -b` (TypeScript strict check) before bundling. Unused imports will cause the build to fail. Always clean up unused imports before committing.

### Architecture

```
someday/src/
├── components/
│   ├── common/      # AppButton, AppInput, AppModal, AppCheckbox
│   ├── layout/      # AppSidebar, AppLayout
│   ├── tasks/       # TaskCard, TaskList, TaskItem, TaskCreateModal, TaskDetailPopup
│   ├── calendar/    # MonthCalendar, WeekCalendar, DayColumn
│   ├── projects/    # ProjectCard, ProjectList
│   └── archive/     # ArchiveList
├── views/           # Dashboard, Calendar, Projects, Archive, Settings, Statistics
├── stores/          # Pinia: taskStore, projectStore, archiveStore, settingsStore, uiStore
├── types/           # TypeScript: task.ts, project.ts, settings.ts
├── router/
└── styles/          # main.css (Tailwind v4 CSS-first + dark mode variables)
```

**State management:** Pinia stores connect to Tauri commands via `@tauri-apps/api/core` `invoke()`. Settings are persisted via `tauri-plugin-store` (JSON file). All task/project data is in SQLite via Rust.

**Rust backend:** `src-tauri/src/lib.rs` defines Tauri commands. Database at `{local_app_data}/someday/someday.db`. Settings at `{local_app_data}/someday/settings.json`. Commands registered with `generate_handler![]`.

### Design System (Aeon Minimalist)

- **Typography:** Manrope (display/headlines) + Inter (body/labels)
- **Colors:** Deep blues — primary `#24389c`, primary_container `#3f51b5`
- **Dark mode:** CSS variables defined under `.dark` class in `main.css`; toggle controlled by `settingsStore.setTheme()` which adds/removes `.dark` on `<html>`
- **No borders:** Sectioning via background color shifts (`surface-container` tiers)
- **Cloud Shadow:** `box-shadow: 0 12px 40px rgba(25, 28, 30, 0.06)`

### Navigation

Fixed left sidebar (256px): Dashboard → Calendar → Projects → Stats → Archive → Settings

Sidebar bottom shows user profile from `settingsStore.settings.profile` (name, title, avatar).

### Key Files

- `someday/src-tauri/tauri.conf.json` — Tauri app config; `identifier` must not end with `.app`
- `someday/src/styles/main.css` — Tailwind v4 CSS-first theme + `.dark` class overrides
- `src-tauri/src/lib.rs` — All Rust Tauri commands (CRUD, archive, settings)
- `stitch_todo_list/aeon_minimalist/DESIGN.md` — Full design system spec
- `stitch_todo_list/prd.html` — Product requirements document

### Development Notes

- Tauri build requires Rust toolchain (Windows: use rustup.rs)
- Frontend-only dev: `npm run dev` in `someday/` works without Tauri
- The `dist/` folder contains the last built frontend assets
- Avatar images are stored as base64 strings in settings profile
- TaskDetailPopup positioning is relative to the clicked task item's edge

---

## Stitch Todo List — HTML Prototypes

```
stitch_todo_list/
├── prd.html              # Product Requirements Document (Chinese)
├── aeon_minimalist/
│   └── DESIGN.md         # Design System specification
├── _1/ through _8/      # Iterative UI prototype screens
│   ├── code.html         # Standalone Tailwind HTML (CDN)
│   └── screen.png        # Visual reference
```

- Pure static HTML, no build step
- Reference for UI patterns and design decisions
