<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Task } from '@/types/task'
import {
  format,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  addWeeks,
  subWeeks,
} from 'date-fns'
import DayColumn from './DayColumn.vue'
import TaskDetailPopup from '@/components/tasks/TaskDetailPopup.vue'
import { useTaskStore } from '@/stores/taskStore'

const taskStore = useTaskStore()

const selectedTask = ref<Task | null>(null)
const selectedTaskPosition = ref<{ x: number; y: number } | null>(null)

const currentWeek = ref(new Date())

const weekDays = computed(() => {
  const weekStart = startOfWeek(currentWeek.value, { weekStartsOn: 1 })
  const weekEnd = endOfWeek(currentWeek.value, { weekStartsOn: 1 })
  return eachDayOfInterval({ start: weekStart, end: weekEnd })
})

const weekLabel = computed(() => {
  const weekStart = weekDays.value[0]
  const weekEnd = weekDays.value[6]
  if (format(weekStart, 'yyyy-MM') === format(weekEnd, 'yyyy-MM')) {
    return `${format(weekStart, 'M月d日')} - ${format(weekEnd, 'd日')}日`
  }
  return `${format(weekStart, 'M月d日')} - ${format(weekEnd, 'M月d日')}`
})

function getTasksForDay(day: Date) {
  const dateStr = format(day, 'yyyy-MM-dd')
  return taskStore.getTasksForDate(dateStr)
}

function previousWeek() {
  currentWeek.value = subWeeks(currentWeek.value, 1)
}

function nextWeek() {
  currentWeek.value = addWeeks(currentWeek.value, 1)
}

function goToToday() {
  currentWeek.value = new Date()
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
</script>

<template>
  <div class="bg-surface-container-lowest rounded-xl p-6 shadow-cloud">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <h2 class="text-2xl font-bold font-headline text-on-surface">
          {{ weekLabel }}
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
          @click="previousWeek"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
        >
          <span class="material-symbols-outlined text-on-surface-variant">chevron_left</span>
        </button>
        <button
          @click="nextWeek"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
        >
          <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
        </button>
      </div>
    </div>

    <!-- Week grid -->
    <div class="grid grid-cols-7 gap-3">
      <DayColumn
        v-for="day in weekDays"
        :key="day.toISOString()"
        :day="day"
        :tasks="getTasksForDay(day)"
        @task-click="openTaskDetail"
      />
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
