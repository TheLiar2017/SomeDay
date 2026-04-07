import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project, ProjectCreateInput, ProjectUpdateInput } from '@/types/project'
import { invoke } from '@tauri-apps/api/core'

export const useProjectStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])
  const isLoading = ref(false)

  const activeProjects = computed(() =>
    projects.value.filter(p => p.status === 'active')
  )

  const projectsByProgress = computed(() =>
    [...activeProjects.value].sort((a, b) => b.progress - a.progress)
  )

  async function loadProjects() {
    isLoading.value = true
    try {
      projects.value = await invoke<Project[]>('load_projects')
    } catch {
      projects.value = []
    } finally {
      isLoading.value = false
    }
  }

  async function createProject(input: ProjectCreateInput): Promise<Project> {
    const project = await invoke<Project>('create_project', { input })
    projects.value.push(project)
    return project
  }

  async function updateProject(id: string, input: ProjectUpdateInput): Promise<Project> {
    const updated = await invoke<Project>('update_project', { id, input })
    const index = projects.value.findIndex(p => p.id === id)
    if (index !== -1) projects.value[index] = updated
    return updated
  }

  async function archiveProject(id: string) {
    await updateProject(id, { status: 'archived' })
  }

  async function deleteProject(id: string) {
    await invoke('delete_project', { id })
    projects.value = projects.value.filter(p => p.id !== id)
  }

  function getProjectById(id: string): Project | undefined {
    return projects.value.find(p => p.id === id)
  }

  return {
    projects,
    isLoading,
    activeProjects,
    projectsByProgress,
    loadProjects,
    createProject,
    updateProject,
    archiveProject,
    deleteProject,
    getProjectById,
  }
})
