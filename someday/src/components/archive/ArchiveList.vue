<script setup lang="ts">
import { useArchiveStore } from '@/stores/archiveStore'
import { useProjectStore } from '@/stores/projectStore'
import { computed, onMounted } from 'vue'
import { format } from 'date-fns'

const archiveStore = useArchiveStore()
const projectStore = useProjectStore()

onMounted(() => {
  projectStore.loadProjects()
})

const tabs = [
  { key: 'all', label: '全部' },
  { key: 'tasks', label: '任务' },
  { key: 'projects', label: '项目' },
] as const

const sortedArchivedTasks = computed(() => {
  const tasks = archiveStore.filteredArchivedTasks
  return [...tasks].sort((a, b) => {
    const dateA = a.completedAt ? new Date(a.completedAt).getTime() : 0
    const dateB = b.completedAt ? new Date(b.completedAt).getTime() : 0
    return dateB - dateA
  })
})

function getProjectName(projectId: string | undefined): string | null {
  if (!projectId) return null
  const project = projectStore.getProjectById(projectId)
  return project?.name || null
}
</script>

<template>
  <div class="space-y-6">
    <!-- Filter tabs -->
    <div class="flex gap-2">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        :class="[
          'px-6 py-2 rounded-full text-sm font-medium transition-colors',
          archiveStore.activeFilter === tab.key
            ? 'bg-primary text-on-primary'
            : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container',
        ]"
        @click="archiveStore.activeFilter = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Tasks list -->
    <div
      v-if="archiveStore.activeFilter === 'all' || archiveStore.activeFilter === 'tasks'"
      class="space-y-4"
    >
      <div
        v-for="task in sortedArchivedTasks"
        :key="task.id"
        class="bg-surface-container-lowest rounded-xl p-6 group transition-all hover:bg-surface-container-low"
      >
        <div class="flex justify-between items-start">
          <div class="flex gap-4">
            <div class="mt-1">
              <span
                class="material-symbols-outlined text-primary/40"
                style="font-variation-settings: 'FILL' 1;"
              >
                check_circle
              </span>
            </div>
            <div>
              <h3 class="text-lg font-semibold text-on-surface-variant line-through opacity-60">
                {{ task.title }}
              </h3>
              <div class="flex items-center gap-3 mt-2">
                <span
                  v-if="getProjectName(task.projectId)"
                  class="text-xs px-2 py-0.5 bg-primary-container/30 text-primary rounded flex items-center gap-1"
                >
                  <span class="material-symbols-outlined text-xs">folder</span>
                  {{ getProjectName(task.projectId) }}
                </span>
                <span class="text-xs text-on-surface-variant">
                  完成日期: {{ task.completedAt ? format(new Date(task.completedAt), 'yyyy年MM月dd日') : '-' }}
                </span>
              </div>
            </div>
          </div>
          <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              @click="archiveStore.restoreTask(task.id)"
              class="flex items-center gap-1 text-primary text-xs font-bold uppercase tracking-wider hover:underline"
            >
              <span class="material-symbols-outlined text-sm">settings_backup_restore</span>
              恢复
            </button>
            <button
              @click="archiveStore.permanentlyDeleteTask(task.id)"
              class="flex items-center gap-1 text-error text-xs font-bold uppercase tracking-wider hover:underline ml-2"
            >
              <span class="material-symbols-outlined text-sm">delete_forever</span>
              彻底删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Projects list -->
    <div
      v-if="archiveStore.activeFilter === 'all' || archiveStore.activeFilter === 'projects'"
      class="space-y-4"
    >
      <div
        v-for="project in archiveStore.filteredArchivedProjects"
        :key="project.id"
        class="bg-surface-container-lowest rounded-xl p-6 group transition-all hover:bg-surface-container-low"
      >
        <div class="flex justify-between items-start">
          <div class="flex gap-4">
            <div class="mt-1">
              <span class="material-symbols-outlined text-secondary">folder</span>
            </div>
            <div>
              <h3 class="text-lg font-bold text-on-surface tracking-tight">
                {{ project.name }}
              </h3>
              <div class="flex items-center gap-3 mt-2">
                <span class="text-xs px-2 py-0.5 bg-secondary-container/20 text-on-secondary-container rounded">
                  项目已关闭
                </span>
                <span class="text-xs text-on-surface-variant">
                  归档日期: {{ project.archivedAt ? format(new Date(project.archivedAt), 'yyyy年MM月dd日') : '-' }}
                </span>
              </div>
            </div>
          </div>
          <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              @click="archiveStore.restoreProject(project.id)"
              class="flex items-center gap-1 text-primary text-xs font-bold uppercase tracking-wider hover:underline"
            >
              <span class="material-symbols-outlined text-sm">settings_backup_restore</span>
              恢复
            </button>
            <button
              @click="archiveStore.permanentlyDeleteProject(project.id)"
              class="flex items-center gap-1 text-error text-xs font-bold uppercase tracking-wider hover:underline ml-2"
            >
              <span class="material-symbols-outlined text-sm">delete_forever</span>
              彻底删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div
      v-if="sortedArchivedTasks.length === 0 && archiveStore.filteredArchivedProjects.length === 0"
      class="text-center py-16 text-on-surface-variant"
    >
      <span class="material-symbols-outlined text-6xl opacity-30">archive</span>
      <p class="mt-4 text-lg font-headline">归档区为空</p>
      <p class="text-sm opacity-70">已完成的任务和项目将显示在这里</p>
    </div>
  </div>
</template>
