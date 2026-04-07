import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Task, TaskCreateInput, TaskUpdateInput } from '@/types/task'
import { invoke } from '@tauri-apps/api/core'

export const useTaskStore = defineStore('tasks', () => {
  const tasks = ref<Task[]>([])
  const isLoading = ref(false)

  const pendingTasks = computed(() =>
    tasks.value.filter(t => t.status === 'pending')
  )

  const tasksByDate = computed(() => {
    const map = new Map<string, Task[]>()
    pendingTasks.value.forEach(task => {
      if (task.dueDate) {
        const existing = map.get(task.dueDate) || []
        map.set(task.dueDate, [...existing, task])
      }
    })
    return map
  })

  const tasksByProject = computed(() => {
    const map = new Map<string, Task[]>()
    tasks.value.forEach(task => {
      if (task.projectId) {
        const existing = map.get(task.projectId) || []
        map.set(task.projectId, [...existing, task])
      }
    })
    return map
  })

  async function loadTasks() {
    isLoading.value = true
    try {
      tasks.value = await invoke<Task[]>('load_tasks')
    } catch {
      tasks.value = []
    } finally {
      isLoading.value = false
    }
  }

  async function createTask(input: TaskCreateInput): Promise<Task> {
    const task = await invoke<Task>('create_task', { input })
    tasks.value.push(task)
    return task
  }

  async function updateTask(id: string, input: TaskUpdateInput): Promise<Task> {
    const updated = await invoke<Task>('update_task', { id, input })
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) tasks.value[index] = updated
    return updated
  }

  async function completeTask(id: string) {
    await updateTask(id, { status: 'completed' })
  }

  async function archiveTask(id: string) {
    await updateTask(id, { status: 'archived' })
  }

  async function deleteTask(id: string) {
    await invoke('delete_task', { id })
    tasks.value = tasks.value.filter(t => t.id !== id)
  }

  function getTasksForDate(date: string): Task[] {
    return tasksByDate.value.get(date) || []
  }

  function getTasksForWeek(startDate: Date): Task[] {
    const endDate = new Date(startDate)
    endDate.setDate(endDate.getDate() + 7)
    return pendingTasks.value.filter(t => {
      if (!t.dueDate) return false
      const due = new Date(t.dueDate)
      return due >= startDate && due < endDate
    })
  }

  function getTasksForMonth(year: number, month: number): Task[] {
    return pendingTasks.value.filter(t => {
      if (!t.dueDate) return false
      const due = new Date(t.dueDate)
      return due.getFullYear() === year && due.getMonth() === month
    })
  }

  return {
    tasks,
    isLoading,
    pendingTasks,
    tasksByDate,
    tasksByProject,
    loadTasks,
    createTask,
    updateTask,
    completeTask,
    archiveTask,
    deleteTask,
    getTasksForDate,
    getTasksForWeek,
    getTasksForMonth,
  }
})
