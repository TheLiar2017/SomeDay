<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format, startOfWeek, addDays, startOfDay, endOfDay, eachDayOfInterval, startOfMonth, startOfYear } from 'date-fns'
import { useTaskStore } from '@/stores/taskStore'
import { useProjectStore } from '@/stores/projectStore'
import { useArchiveStore } from '@/stores/archiveStore'

const taskStore = useTaskStore()
const projectStore = useProjectStore()
const archiveStore = useArchiveStore()

type TimeRange = 'week' | 'month' | 'year'
const timeRange = ref<TimeRange>('week')

onMounted(async () => {
  await Promise.all([
    taskStore.loadTasks(),
    projectStore.loadProjects(),
    archiveStore.loadArchived()
  ])
})

// 中文星期几
const weekDays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']

// 合并所有已完成的任务（包括 taskStore 和 archiveStore 中的归档任务）
const allArchivedTasks = computed(() => {
  // taskStore.tasks 包含所有任务（包括 status='archived' 的）
  // archiveStore.archivedTasks 是单独的归档任务列表
  // 去重合并
  const taskMap = new Map<string, typeof taskStore.tasks[0]>()
  taskStore.tasks.forEach(t => {
    if (t.status === 'archived') {
      taskMap.set(t.id, t)
    }
  })
  archiveStore.archivedTasks.forEach(t => {
    taskMap.set(t.id, t)
  })
  return Array.from(taskMap.values())
})

// 计算某时间范围内完成的任务
function getCompletedTasksInRange(start: Date, end: Date) {
  return allArchivedTasks.value.filter(t => {
    if (!t.completedAt) return false
    const completedDate = new Date(t.completedAt)
    return completedDate >= start && completedDate <= end
  })
}

// 计算某时间范围内应完成的任务总数
function getTotalTasksInRange(start: Date, end: Date) {
  return taskStore.tasks.filter(t => {
    if (t.status === 'archived') return false
    if (!t.dueDate) return false
    const dueDate = new Date(t.dueDate)
    return dueDate >= start && dueDate <= end
  }).length
}

// 根据时间范围获取统计数据
const currentRangeStats = computed(() => {
  const now = new Date()
  let completed = 0
  let total = 0
  let label = ''
  let chartData: { label: string; count: number; isToday?: boolean }[] = []

  if (timeRange.value === 'week') {
    const weekStart = startOfWeek(now, { weekStartsOn: 1 }) // 本周周一
    const weekEnd = endOfDay(now)
    completed = getCompletedTasksInRange(weekStart, weekEnd).length
    total = getTotalTasksInRange(weekStart, weekEnd) + completed
    label = '本周达成率'

    // 横坐标固定显示周一到周日
    for (let i = 0; i < 7; i++) {
      const date = addDays(weekStart, i)
      const dayStart = startOfDay(date)
      const dayEnd = endOfDay(date)
      const dayCompleted = getCompletedTasksInRange(dayStart, dayEnd).length
      // 判断是否是今天
      const isToday = format(date, 'yyyy-MM-dd') === format(now, 'yyyy-MM-dd')
      chartData.push({
        label: weekDays[i],
        count: dayCompleted,
        isToday
      })
    }
  } else if (timeRange.value === 'month') {
    const monthStart = startOfMonth(now)
    const monthEnd = endOfDay(now)
    completed = getCompletedTasksInRange(monthStart, monthEnd).length
    total = getTotalTasksInRange(monthStart, monthEnd) + completed
    label = '本月达成率'

    // 横坐标固定显示周一到周日，统计本月每个星期几的完成数
    for (let i = 0; i < 7; i++) {
      let dayCompleted = 0
      // 遍历本月所有日期，统计落在对应星期几的任务
      const daysInMonth = eachDayOfInterval({ start: monthStart, end: monthEnd })
      daysInMonth.forEach(day => {
        const dayDow = day.getDay()
        // dayDow: 0=周日, 1=周一, ..., 6=周六
        // weekDays index: 0=周一, 1=周二, ..., 6=周日
        const isTargetWeekday = (dayDow === 0 && i === 6) || (dayDow === i + 1)
        if (isTargetWeekday) {
          const dayStart = startOfDay(day)
          const dayEnd = endOfDay(day)
          dayCompleted += getCompletedTasksInRange(dayStart, dayEnd).length
        }
      })
      chartData.push({
        label: weekDays[i],
        count: dayCompleted,
        isToday: false
      })
    }
  } else {
    const yearStart = startOfYear(now)
    const yearEnd = endOfDay(now)
    completed = getCompletedTasksInRange(yearStart, yearEnd).length
    total = getTotalTasksInRange(yearStart, yearEnd) + completed
    label = '本年达成率'

    // 横坐标固定显示周一到周日，统计本年每个星期几的完成数
    for (let i = 0; i < 7; i++) {
      let dayCompleted = 0
      const daysInYear = eachDayOfInterval({ start: yearStart, end: yearEnd })
      daysInYear.forEach(day => {
        const dayDow = day.getDay()
        const isTargetWeekday = (dayDow === 0 && i === 6) || (dayDow === i + 1)
        if (isTargetWeekday) {
          const dayStart = startOfDay(day)
          const dayEnd = endOfDay(day)
          dayCompleted += getCompletedTasksInRange(dayStart, dayEnd).length
        }
      })
      chartData.push({
        label: weekDays[i],
        count: dayCompleted,
        isToday: false
      })
    }
  }

  return { completed, total, label, rate: total > 0 ? Math.round((completed / total) * 100) : 0, chartData }
})

// 累计完成的任务数
const totalCompletedTasks = computed(() => {
  return allArchivedTasks.value.filter(t => t.completedAt).length
})

// 图表最大值
const maxCount = computed(() => {
  return Math.max(...currentRangeStats.value.chartData.map(d => d.count), 1)
})

// 最近完成的任务
const recentCompletedTasks = computed(() => {
  return [...allArchivedTasks.value]
    .filter(t => t.completedAt)
    .sort((a, b) => new Date(b.completedAt!).getTime() - new Date(a.completedAt!).getTime())
    .slice(0, 5)
})

function getProjectName(projectId: string | undefined): string {
  if (!projectId) return ''
  const project = projectStore.getProjectById(projectId)
  return project?.name || ''
}

function formatRelativeTime(dateStr: string): string {
  const date = new Date(dateStr)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return '刚刚'
  if (diffMins < 60) return `${diffMins}分钟前`
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays === 1) return '昨天'
  if (diffDays < 7) return `${diffDays}天前`
  return format(date, 'yyyy年MM月dd日')
}
</script>

<template>
  <div class="max-w-7xl mx-auto">
    <!-- Header -->
    <header class="flex justify-between items-center w-full px-8 py-6">
      <h1 class="text-3xl font-extrabold tracking-tight text-on-surface font-headline">数据统计</h1>
      <div class="flex items-center gap-4">
        <div class="flex bg-surface-container-high rounded-full p-1">
          <button
            :class="[
              'px-4 py-1.5 rounded-full text-sm transition-all',
              timeRange === 'week'
                ? 'bg-surface-container-lowest shadow-cloud text-primary font-bold'
                : 'text-on-surface-variant hover:text-primary'
            ]"
            @click="timeRange = 'week'"
          >
            本周
          </button>
          <button
            :class="[
              'px-4 py-1.5 rounded-full text-sm transition-all',
              timeRange === 'month'
                ? 'bg-surface-container-lowest shadow-cloud text-primary font-bold'
                : 'text-on-surface-variant hover:text-primary'
            ]"
            @click="timeRange = 'month'"
          >
            本月
          </button>
          <button
            :class="[
              'px-4 py-1.5 rounded-full text-sm transition-all',
              timeRange === 'year'
                ? 'bg-surface-container-lowest shadow-cloud text-primary font-bold'
                : 'text-on-surface-variant hover:text-primary'
            ]"
            @click="timeRange = 'year'"
          >
            全年
          </button>
        </div>
      </div>
    </header>

    <!-- Canvas Area -->
    <div class="px-8 space-y-8">
      <!-- Top Summary Section: 同一行显示 -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- 累计完成任务 -->
        <div class="bg-surface-container-lowest p-8 rounded-xl cloud-shadow border-l-4 border-primary">
          <span class="text-on-surface-variant text-sm font-medium">累计完成任务</span>
          <div class="flex items-baseline gap-2 mt-2">
            <span class="text-5xl font-extrabold text-primary">{{ totalCompletedTasks.toLocaleString() }}</span>
            <span class="text-xs text-green-600 font-bold bg-green-50 px-2 py-0.5 rounded-full">
              +{{ currentRangeStats.completed }} {{ timeRange === 'week' ? '本周' : timeRange === 'month' ? '本月' : '本年' }}
            </span>
          </div>
        </div>

        <!-- 动态达成率 -->
        <div class="bg-surface-container-lowest p-8 rounded-xl cloud-shadow">
          <span class="text-on-surface-variant text-xs font-semibold uppercase tracking-wider">{{ currentRangeStats.label }}</span>
          <div class="flex items-end gap-4 mt-2">
            <span class="text-5xl font-extrabold text-on-surface">{{ currentRangeStats.rate }}%</span>
            <div class="flex-1 mb-2">
              <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                <div
                  class="h-full bg-gradient-to-r from-primary to-primary-container rounded-full transition-all duration-500"
                  :style="{ width: `${currentRangeStats.rate}%` }"
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Middle Section: Task Completion Trend -->
      <div class="bg-surface-container-lowest p-8 rounded-xl cloud-shadow">
        <div class="flex justify-between items-center mb-10">
          <h2 class="text-xl font-bold tracking-tight text-on-surface font-headline">任务完成趋势</h2>
          <span class="text-xs font-bold text-primary tracking-widest uppercase">
            {{ timeRange === 'week' ? '7-Day' : timeRange === 'month' ? '4-Week' : '12-Month' }} Analysis
          </span>
        </div>
        <div class="relative h-64 w-full flex items-end justify-between px-2">
          <!-- Grid Lines -->
          <div class="absolute inset-0 flex flex-col justify-between pointer-events-none">
            <div class="w-full border-b border-outline-variant/30 h-0"></div>
            <div class="w-full border-b border-outline-variant/30 h-0"></div>
            <div class="w-full border-b border-outline-variant/30 h-0"></div>
            <div class="w-full border-b border-outline-variant/30 h-0"></div>
          </div>
          <!-- Bars -->
          <div
            v-for="item in currentRangeStats.chartData"
            :key="item.label"
            class="flex flex-col items-center gap-3 z-10 group relative"
          >
            <!-- Tooltip -->
            <div
              class="absolute -top-10 left-1/2 -translate-x-1/2 bg-on-surface text-surface px-2 py-1 rounded text-xs font-medium opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-20"
            >
              {{ item.count }} 个任务
            </div>
            <div
              :class="[
                'w-14 rounded-t-lg transition-all duration-300 cursor-pointer',
                item.isToday
                  ? 'bg-primary cloud-shadow'
                  : 'bg-surface-container-high hover:bg-primary/20'
              ]"
              :style="{ height: `${Math.max((item.count / maxCount) * 200, 20)}px` }"
            ></div>
            <span
              :class="[
                'text-xs font-medium',
                item.isToday ? 'text-primary font-bold' : 'text-on-surface-variant'
              ]"
            >
              {{ item.label }}
            </span>
          </div>
        </div>
      </div>

      <!-- Bottom Section: Recently Completed Tasks -->
      <div class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold tracking-tight text-on-surface font-headline">最近完成的任务</h2>
          <router-link to="/archive">
            <button class="text-primary text-xs font-bold uppercase tracking-wider hover:opacity-70 transition-opacity">
              查看全部档案
            </button>
          </router-link>
        </div>
        <div class="grid grid-cols-1 gap-4">
          <div
            v-for="task in recentCompletedTasks"
            :key="task.id"
            class="bg-surface-container-lowest p-6 rounded-xl flex items-start gap-4 hover:bg-surface-container-low transition-all duration-200 cursor-pointer"
          >
            <div class="w-10 h-10 rounded-full bg-green-50 flex items-center justify-center flex-shrink-0 mt-1">
              <span class="material-symbols-outlined" style="color: #16a34a; font-variation-settings: 'FILL' 1;">check_circle</span>
            </div>
            <div class="flex-1 min-w-0">
              <h4 class="font-bold text-on-surface text-base">{{ task.title }}</h4>
              <div v-if="task.description" class="text-sm text-on-surface-variant mt-1 line-clamp-2">
                {{ task.description }}
              </div>
              <div v-if="getProjectName(task.projectId)" class="flex items-center gap-4 mt-2">
                <span class="text-xs text-primary flex items-center gap-1 bg-primary-container/30 px-2 py-0.5 rounded">
                  <span class="material-symbols-outlined text-xs">folder</span>
                  {{ getProjectName(task.projectId) }}
                </span>
              </div>
            </div>
            <span class="text-xs font-medium text-on-surface-variant flex-shrink-0">
              {{ formatRelativeTime(task.completedAt!) }}
            </span>
          </div>

          <!-- Empty State -->
          <div
            v-if="recentCompletedTasks.length === 0"
            class="text-center py-16 text-on-surface-variant"
          >
            <span class="material-symbols-outlined text-6xl opacity-30">task_alt</span>
            <p class="mt-4 text-lg font-headline">暂无完成记录</p>
            <p class="text-sm opacity-70">完成任务后会在此处显示</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
