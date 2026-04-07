<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MonthCalendar from '@/components/calendar/MonthCalendar.vue'
import WeekCalendar from '@/components/calendar/WeekCalendar.vue'
import { useTaskStore } from '@/stores/taskStore'

const taskStore = useTaskStore()
const viewMode = ref<'month' | 'week'>('month')

onMounted(() => {
  taskStore.loadTasks()
})
</script>

<template>
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <header class="mb-8">
      <h1 class="text-5xl font-extrabold tracking-tight text-on-surface mb-6">日历</h1>

      <!-- View toggle -->
      <div class="flex items-center gap-2 bg-surface-container-high rounded-full p-1 inline-flex">
        <button
          :class="[
            'px-6 py-2 rounded-full text-sm font-medium transition-all',
            viewMode === 'month'
              ? 'bg-surface-container-lowest shadow-sm text-on-surface'
              : 'text-on-surface-variant hover:text-on-surface',
          ]"
          @click="viewMode = 'month'"
        >
          月视图
        </button>
        <button
          :class="[
            'px-6 py-2 rounded-full text-sm font-medium transition-all',
            viewMode === 'week'
              ? 'bg-surface-container-lowest shadow-sm text-on-surface'
              : 'text-on-surface-variant hover:text-on-surface',
          ]"
          @click="viewMode = 'week'"
        >
          周视图
        </button>
      </div>
    </header>

    <!-- Calendar -->
    <MonthCalendar v-if="viewMode === 'month'" />
    <WeekCalendar v-else />
  </div>
</template>
