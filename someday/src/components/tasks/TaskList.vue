<script setup lang="ts">
import type { Task } from '@/types/task'
import TaskCard from './TaskCard.vue'

defineProps<{
  tasks: Task[]
}>()
</script>

<template>
  <div class="space-y-4">
    <TaskCard v-for="task in tasks" :key="task.id" :task="task" />
    <div
      v-if="tasks.length === 0"
      class="text-center py-16 text-on-surface-variant"
    >
      <span class="material-symbols-outlined text-6xl opacity-30">task_alt</span>
      <p class="mt-4 text-lg font-headline">暂无任务</p>
      <p class="text-sm opacity-70">点击上方按钮创建新任务</p>
    </div>
  </div>
</template>
