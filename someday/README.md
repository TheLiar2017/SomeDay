# Someday — Vue Frontend

> Someday 应用的 Vue 3 前端模块。

## 开发

```bash
# 安装依赖
npm install

# 开发服务器 (http://localhost:5173)
npm run dev

# 类型检查
npm run build

# 预览生产构建
npm run preview
```

## 目录结构

```
src/
├── components/       # Vue 组件
│   ├── common/      # AppButton, AppInput, AppModal, AppCheckbox
│   ├── layout/      # AppSidebar, AppLayout
│   ├── tasks/       # TaskCard, TaskList, TaskItem, TaskCreateModal, TaskDetailPopup
│   ├── calendar/    # MonthCalendar, WeekCalendar, DayColumn
│   ├── projects/    # ProjectCard, ProjectList
│   └── archive/     # ArchiveList
├── views/           # Dashboard, Calendar, Projects, Statistics, Archive, Settings
├── stores/          # Pinia stores (task, project, archive, settings, ui)
├── types/           # TypeScript 类型 (task, project, settings)
├── router/          # Vue Router
└── styles/          # Tailwind CSS v4 样式
```

## 设计规范

参考根目录 `stitch_todo_list/aeon_minimalist/DESIGN.md`。
