<script setup lang="ts">
import { ref, computed } from 'vue'
import {
  format,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  addWeeks,
  subWeeks,
} from 'date-fns'
import { zhCN } from 'date-fns/locale'
import DayColumn from './DayColumn.vue'
import { useTaskStore } from '@/stores/taskStore'

const taskStore = useTaskStore()

const currentWeek = ref(new Date())

const weekDays = computed(() => {
  const weekStart = startOfWeek(currentWeek.value, { locale: zhCN })
  const weekEnd = endOfWeek(currentWeek.value, { locale: zhCN })
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
      />
    </div>
  </div>
</template>
