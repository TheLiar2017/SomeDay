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
import { useTaskStore } from '@/stores/taskStore'
import { useUiStore } from '@/stores/uiStore'

const taskStore = useTaskStore()
const uiStore = useUiStore()

const currentMonth = ref(new Date())

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

const calendarDays = computed(() => {
  const monthStart = startOfMonth(currentMonth.value)
  const monthEnd = endOfMonth(currentMonth.value)
  const calendarStart = startOfWeek(monthStart, { locale: zhCN })
  const calendarEnd = endOfWeek(monthEnd, { locale: zhCN })

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
        v-for="day in weekDays"
        :key="day"
        class="text-center text-xs font-semibold text-on-surface-variant uppercase py-2"
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
            ? 'bg-surface'
            : 'bg-surface-container-low opacity-50',
          isToday(day) ? 'ring-2 ring-primary/30' : '',
        ]"
        @click="openTaskCreate(day)"
      >
        <div class="flex items-center justify-between mb-2">
          <span
            :class="[
              'text-sm font-medium',
              isToday(day) ? 'text-primary font-bold' : 'text-on-surface',
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
        <div class="space-y-1 overflow-y-auto max-h-[80px]">
          <TaskItem
            v-for="task in getTasksForDay(day).slice(0, 3)"
            :key="task.id"
            :task="task"
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
  </div>
</template>
