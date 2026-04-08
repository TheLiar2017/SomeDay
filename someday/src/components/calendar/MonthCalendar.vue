<script setup lang="ts">
import { ref, computed } from 'vue'
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  isSameMonth,
  isToday,
  addMonths,
  subMonths,
} from 'date-fns'
import { zhCN } from 'date-fns/locale'
import TaskItem from '@/components/tasks/TaskItem.vue'
import TaskDetailPopup from '@/components/tasks/TaskDetailPopup.vue'
import { useTaskStore } from '@/stores/taskStore'
import { useUiStore } from '@/stores/uiStore'
import type { Task } from '@/types/task'

const taskStore = useTaskStore()
const uiStore = useUiStore()

const selectedTask = ref<Task | null>(null)
const selectedTaskPosition = ref<{ x: number; y: number } | null>(null)

const currentMonth = ref(new Date())

const weekDays = ['一', '二', '三', '四', '五', '六', '日']

const calendarDays = computed(() => {
  const monthStart = startOfMonth(currentMonth.value)
  const monthEnd = endOfMonth(currentMonth.value)
  const calendarStart = startOfWeek(monthStart, { weekStartsOn: 1 })
  const calendarEnd = endOfWeek(monthEnd, { weekStartsOn: 1 })

  return eachDayOfInterval({ start: calendarStart, end: calendarEnd })
})

const monthLabel = computed(() => {
  return format(currentMonth.value, 'yyyy 年 MMMM', { locale: zhCN })
})

function getTasksForDay(day: Date) {
  const dateStr = format(day, 'yyyy-MM-dd')
  return taskStore.getTasksForDate(dateStr)
}

function previousMonth() {
  currentMonth.value = subMonths(currentMonth.value, 1)
}

function nextMonth() {
  currentMonth.value = addMonths(currentMonth.value, 1)
}

function goToToday() {
  currentMonth.value = new Date()
}

function openTaskCreate(day: Date) {
  uiStore.openTaskCreateModal(format(day, 'yyyy-MM-dd'))
}

function openTaskDetail(task: Task, event: MouseEvent) {
  selectedTask.value = task
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  selectedTaskPosition.value = { x: rect.left, y: rect.bottom + 8 }
}

function closeTaskDetail() {
  selectedTask.value = null
  selectedTaskPosition.value = null
}

function isWeekend(day: Date): boolean {
  const d = day.getDay()
  return d === 0 || d === 6
}
</script>

<template>
  <div class="bg-surface-container-lowest rounded-xl p-6 shadow-cloud">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <h2 class="text-2xl font-bold font-headline text-on-surface">
          {{ monthLabel }}
        </h2>
        <button
          @click="goToToday"
          class="px-3 py-1 text-xs font-medium bg-surface-container-high text-on-surface-variant rounded-full hover:bg-surface-container transition-colors"
        >
          今天
        </button>
      </div>
      <div class="flex items-center gap-2">
        <button
          @click="previousMonth"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
        >
          <span class="material-symbols-outlined text-on-surface-variant">chevron_left</span>
        </button>
        <button
          @click="nextMonth"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
        >
          <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
        </button>
      </div>
    </div>

    <!-- Weekday headers -->
    <div class="grid grid-cols-7 gap-2 mb-2">
      <div
        v-for="(day, index) in weekDays"
        :key="day"
        :class="[
          'text-center text-xs font-semibold uppercase py-2',
          index === 0 || index === 6 ? 'text-error/70' : 'text-on-surface-variant',
        ]"
      >
        {{ day }}
      </div>
    </div>

    <!-- Calendar grid -->
    <div class="grid grid-cols-7 gap-2">
      <div
        v-for="day in calendarDays"
        :key="day.toISOString()"
        :class="[
          'min-h-[120px] rounded-xl p-2 transition-colors cursor-pointer group',
          isSameMonth(day, currentMonth)
            ? isWeekend(day)
              ? 'bg-error-container/20'
              : 'bg-surface'
            : 'bg-surface-container-low opacity-50',
          isToday(day) ? 'ring-2 ring-primary/30' : '',
        ]"
        @click="openTaskCreate(day)"
      >
        <div class="flex items-center justify-between mb-2">
          <span
            :class="[
              'text-sm font-medium',
              isToday(day) ? 'text-primary font-bold' : isWeekend(day) ? 'text-error/80' : 'text-on-surface',
            ]"
          >
            {{ format(day, 'd') }}
          </span>
          <button
            v-if="isSameMonth(day, currentMonth)"
            class="opacity-0 group-hover:opacity-100 p-1 hover:bg-surface-container-high rounded-full transition-all"
            @click.stop="openTaskCreate(day)"
          >
            <span class="material-symbols-outlined text-xs text-on-surface-variant">add</span>
          </button>
        </div>
        <div class="space-y-1 overflow-y-auto max-h-[80px]" @click.stop>
          <TaskItem
            v-for="task in getTasksForDay(day).slice(0, 3)"
            :key="task.id"
            :task="task"
            @task-click="openTaskDetail"
          />
          <div
            v-if="getTasksForDay(day).length > 3"
            class="text-xs text-on-surface-variant text-center py-1"
          >
            +{{ getTasksForDay(day).length - 3 }} 更多
          </div>
        </div>
      </div>
    </div>

    <!-- Task Detail Popup -->
    <Teleport to="body">
      <div
        v-if="selectedTask && selectedTaskPosition"
        class="fixed z-[100]"
        :style="{ left: selectedTaskPosition.x + 'px', top: selectedTaskPosition.y + 'px' }"
        @click.stop
      >
        <TaskDetailPopup
          :task="selectedTask"
          :visible="!!selectedTask"
          @close="closeTaskDetail"
        />
      </div>
      <!-- Backdrop -->
      <div
        v-if="selectedTask"
        class="fixed inset-0 z-[99]"
        @click="closeTaskDetail"
      ></div>
    </Teleport>
  </div>
</template>
