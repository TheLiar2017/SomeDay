<script setup lang="ts">
import AppSidebar from './AppSidebar.vue'
import TaskCreateModal from '@/components/tasks/TaskCreateModal.vue'
import { useUiStore } from '@/stores/uiStore'

const uiStore = useUiStore()
</script>

<template>
  <div class="min-h-screen bg-surface">
    <AppSidebar />
    <main class="ml-64 p-12">
      <slot />
    </main>
    <TaskCreateModal
      v-if="uiStore.taskCreateModalOpen"
      @close="uiStore.closeTaskCreateModal()"
    />
  </div>
</template>
