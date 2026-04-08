import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Task, Project } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { useTaskStore } from './taskStore'
import { useProjectStore } from './projectStore'

export type ArchiveFilter = 'all' | 'tasks' | 'projects'

export const useArchiveStore = defineStore('archive', () => {
  const archivedTasks = ref<Task[]>([])
  const archivedProjects = ref<Project[]>([])
  const isLoading = ref(false)
  const searchQuery = ref('')
  const activeFilter = ref<ArchiveFilter>('all')

  const filteredArchivedTasks = computed(() => {
    if (!searchQuery.value) return archivedTasks.value
    const q = searchQuery.value.toLowerCase()
    return archivedTasks.value.filter(t =>
      t.title.toLowerCase().includes(q) ||
      t.description?.toLowerCase().includes(q)
    )
  })

  const filteredArchivedProjects = computed(() => {
    if (!searchQuery.value) return archivedProjects.value
    const q = searchQuery.value.toLowerCase()
    return archivedProjects.value.filter(p =>
      p.name.toLowerCase().includes(q) ||
      p.description?.toLowerCase().includes(q)
    )
  })

  const archiveStats = computed(() => ({
    totalTasks: archivedTasks.value.length,
    totalProjects: archivedProjects.value.length,
  }))

  async function loadArchived() {
    isLoading.value = true
    try {
      const [tasks, projects] = await Promise.all([
        invoke<Task[]>('load_archived_tasks'),
        invoke<Project[]>('load_archived_projects'),
      ])
      archivedTasks.value = tasks
      archivedProjects.value = projects
    } catch {
      archivedTasks.value = []
      archivedProjects.value = []
    } finally {
      isLoading.value = false
    }
  }

  async function restoreTask(id: string) {
    await invoke('restore_task', { id })
    archivedTasks.value = archivedTasks.value.filter(t => t.id !== id)
    // Reload tasks so restored task appears in main list
    const taskStore = useTaskStore()
    await taskStore.loadTasks()
  }

  async function permanentlyDeleteTask(id: string) {
    await invoke('permanently_delete_task', { id })
    archivedTasks.value = archivedTasks.value.filter(t => t.id !== id)
  }

  async function restoreProject(id: string) {
    await invoke('restore_project', { id })
    archivedProjects.value = archivedProjects.value.filter(p => p.id !== id)
    // Reload projects so restored project appears in main list
    const projectStore = useProjectStore()
    await projectStore.loadProjects()
  }

  async function permanentlyDeleteProject(id: string) {
    await invoke('permanently_delete_project', { id })
    archivedProjects.value = archivedProjects.value.filter(p => p.id !== id)
  }

  return {
    archivedTasks,
    archivedProjects,
    isLoading,
    searchQuery,
    activeFilter,
    filteredArchivedTasks,
    filteredArchivedProjects,
    archiveStats,
    loadArchived,
    restoreTask,
    permanentlyDeleteTask,
    restoreProject,
    permanentlyDeleteProject,
  }
})
