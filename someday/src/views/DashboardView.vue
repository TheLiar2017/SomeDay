<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { format } from 'date-fns'
import TaskList from '@/components/tasks/TaskList.vue'
import AppButton from '@/components/common/AppButton.vue'
import { useTaskStore } from '@/stores/taskStore'
import { useUiStore } from '@/stores/uiStore'

const taskStore = useTaskStore()
const uiStore = useUiStore()

const today = new Date()
const todayStr = format(today, 'yyyy-MM-dd')

const todayTasks = computed(() => taskStore.getTasksForDate(todayStr))
// 只统计今天的待办任务
const pendingCount = computed(() => todayTasks.value.length)
// 只统计今天完成的任务
const completedTodayCount = computed(() =>
  taskStore.tasks.filter(t =>
    t.status === 'completed' && t.completedAt &&
    format(new Date(t.completedAt), 'yyyy-MM-dd') === todayStr
  ).length
)

onMounted(() => {
  taskStore.loadTasks()
})
</script>

<template>
  <div class="max-w-5xl mx-auto">
    <!-- Header -->
    <header class="mb-12">
      <h1 class="text-5xl font-extrabold tracking-tight text-on-surface mb-4">
        {{ format(today, 'M月d日') }} · 今日
      </h1>
      <p class="text-on-surface-variant text-lg max-w-2xl">
        距离目标更近一步。今天你有 {{ pendingCount }} 个待办任务。
      </p>
    </header>

    <!-- Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
      <div class="bg-surface-container-lowest rounded-xl p-6 shadow-cloud">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 bg-primary-container/20 rounded-full flex items-center justify-center">
            <span class="material-symbols-outlined text-primary">task_alt</span>
          </div>
          <div>
            <p class="text-3xl font-bold font-headline text-on-surface">{{ pendingCount }}</p>
            <p class="text-sm text-on-surface-variant">待完成任务</p>
          </div>
        </div>
      </div>

      <div class="bg-surface-container-lowest rounded-xl p-6 shadow-cloud">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 bg-secondary-container/20 rounded-full flex items-center justify-center">
            <span class="material-symbols-outlined text-secondary">check_circle</span>
          </div>
          <div>
            <p class="text-3xl font-bold font-headline text-on-surface">{{ completedTodayCount }}</p>
            <p class="text-sm text-on-surface-variant">今日已完成</p>
          </div>
        </div>
      </div>

      <div class="bg-surface-container-lowest rounded-xl p-6 shadow-cloud">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 bg-tertiary-container/20 rounded-full flex items-center justify-center">
            <span class="material-symbols-outlined text-tertiary">rocket_launch</span>
          </div>
          <div>
            <p class="text-3xl font-bold font-headline text-on-surface">
              {{ Math.round((completedTodayCount / (pendingCount + completedTodayCount || 1)) * 100) }}%
            </p>
            <p class="text-sm text-on-surface-variant">完成率</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Today's Tasks -->
    <section>
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold font-headline text-on-surface">今日任务</h2>
        <AppButton @click="uiStore.openTaskCreateModal(todayStr)">
          <span class="material-symbols-outlined text-sm">add</span>
          添加任务
        </AppButton>
      </div>
      <TaskList :tasks="todayTasks" />
    </section>
  </div>
</template>
