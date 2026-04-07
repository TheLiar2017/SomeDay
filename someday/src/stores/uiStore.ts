import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const sidebarCollapsed = ref(false)
  const taskCreateModalOpen = ref(false)
  const activeModalDate = ref<string | null>(null)

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  function openTaskCreateModal(date?: string) {
    taskCreateModalOpen.value = true
    activeModalDate.value = date ?? null
  }

  function closeTaskCreateModal() {
    taskCreateModalOpen.value = false
    activeModalDate.value = null
  }

  return {
    sidebarCollapsed,
    taskCreateModalOpen,
    activeModalDate,
    toggleSidebar,
    openTaskCreateModal,
    closeTaskCreateModal,
  }
})
