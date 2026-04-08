<script setup lang="ts">
import type { Project } from '@/types/project'
import { useProjectStore } from '@/stores/projectStore'
import { useTaskStore } from '@/stores/taskStore'

const props = defineProps<{
  project: Project
}>()

const projectStore = useProjectStore()
const taskStore = useTaskStore()

const projectTasks = computed(() => taskStore.tasksByProject.get(props.project.id) || [])

import { computed } from 'vue'

const progress = computed(() => {
  if (projectTasks.value.length === 0) return Math.round(props.project.progress)
  const pending = projectTasks.value.filter(t => t.status === 'pending').length
  return Math.round(((projectTasks.value.length - pending) / projectTasks.value.length) * 100)
})

async function archiveProject() {
  await projectStore.archiveProject(props.project.id)
}

async function deleteProject() {
  await projectStore.deleteProject(props.project.id)
}
</script>

<template>
  <div class="bg-surface-container-lowest rounded-xl p-6 hover:shadow-cloud transition-all duration-300 group">
    <div class="flex items-start justify-between mb-4">
      <div class="w-12 h-12 rounded-xl bg-primary-container/20 flex items-center justify-center">
        <span class="material-symbols-outlined text-primary text-2xl">folder</span>
      </div>
      <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          @click="archiveProject"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
          title="归档"
        >
          <span class="material-symbols-outlined text-on-surface-variant text-sm">archive</span>
        </button>
        <button
          @click="deleteProject"
          class="p-2 hover:bg-error-container rounded-full transition-colors"
          title="删除"
        >
          <span class="material-symbols-outlined text-error text-sm">delete</span>
        </button>
      </div>
    </div>

    <h3 class="text-lg font-bold font-headline text-on-surface mb-2">
      {{ project.name }}
    </h3>

    <p v-if="project.description" class="text-sm text-on-surface-variant mb-4 line-clamp-2">
      {{ project.description }}
    </p>

    <!-- Progress bar -->
    <div class="mb-4">
      <div class="flex items-center justify-between text-xs mb-1">
        <span class="text-on-surface-variant font-medium">进度</span>
        <span class="text-primary font-bold">{{ progress }}%</span>
      </div>
      <div class="h-2 bg-surface-container-high rounded-full overflow-hidden">
        <div
          class="h-full bg-gradient-to-r from-primary to-primary-container rounded-full transition-all duration-500"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
    </div>

    <!-- Meta -->
    <div class="flex items-center gap-3 text-xs text-on-surface-variant">
      <span class="flex items-center gap-1">
        <span class="material-symbols-outlined text-sm">task</span>
        {{ projectTasks.length }} 个任务
      </span>
      <span
        v-for="tag in project.tags.slice(0, 2)"
        :key="tag"
        class="px-2 py-0.5 bg-surface-container rounded"
      >
        {{ tag }}
      </span>
    </div>
  </div>
</template>
