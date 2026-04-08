<script setup lang="ts">
import { computed } from 'vue'
import type { Task } from '@/types/task'
import AppCheckbox from '@/components/common/AppCheckbox.vue'
import { useTaskStore } from '@/stores/taskStore'
import { useProjectStore } from '@/stores/projectStore'
import { format } from 'date-fns'

const props = defineProps<{
  task: Task
}>()

const taskStore = useTaskStore()
const projectStore = useProjectStore()

const priorityColors = {
  low: 'bg-secondary-container text-on-secondary-container',
  medium: 'bg-tertiary-container text-on-tertiary-container',
  high: 'bg-error-container text-on-error-container',
}

const isCompleted = computed(() => props.task.status === 'completed')

const projectName = computed(() => {
  if (!props.task.projectId) return null
  const project = projectStore.getProjectById(props.task.projectId)
  return project?.name || null
})

async function toggleComplete() {
  if (isCompleted.value) {
    await taskStore.updateTask(props.task.id, { status: 'pending' })
  } else {
    await taskStore.completeTask(props.task.id)
  }
}

const formattedDate = computed(() => {
  if (!props.task.dueDate) return null
  return format(new Date(props.task.dueDate), 'MM月dd日')
})
</script>

<template>
  <div
    :class="[
      'bg-surface-container-lowest rounded-xl p-6 transition-all duration-300 hover:bg-surface-container-low group',
    ]"
  >
    <div class="flex justify-between items-start gap-4">
      <div class="flex gap-4">
        <div class="mt-1">
          <AppCheckbox :checked="isCompleted" @update:checked="toggleComplete" />
        </div>
        <div>
          <h3
            :class="[
              'text-lg font-semibold transition-all',
              isCompleted ? 'text-on-surface-variant line-through opacity-60' : 'text-on-surface',
            ]"
          >
            {{ task.title }}
          </h3>
          <div v-if="task.description" class="text-sm text-on-surface-variant mt-1">
            {{ task.description }}
          </div>
          <div class="flex items-center gap-3 mt-3">
            <span
              :class="['text-xs font-medium px-2 py-0.5 rounded', priorityColors[task.priority]]"
            >
              {{ task.priority === 'low' ? '低' : task.priority === 'medium' ? '中' : '高' }}
            </span>
            <span v-if="formattedDate" class="text-xs text-on-surface-variant">
              {{ formattedDate }}
            </span>
            <span
              v-if="projectName"
              class="text-xs px-2 py-0.5 bg-primary-container/30 text-primary rounded flex items-center gap-1"
            >
              <span class="material-symbols-outlined text-xs">folder</span>
              {{ projectName }}
            </span>
            <span
              v-for="tag in task.tags"
              :key="tag"
              class="text-xs px-2 py-0.5 bg-surface-container text-on-surface-variant rounded"
            >
              {{ tag }}
            </span>
          </div>
        </div>
      </div>
      <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          @click="taskStore.archiveTask(task.id)"
          class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
          :title="'归档'"
        >
          <span class="material-symbols-outlined text-on-surface-variant text-sm">archive</span>
        </button>
        <button
          @click="taskStore.deleteTask(task.id)"
          class="p-2 hover:bg-error-container rounded-full transition-colors"
          :title="'删除'"
        >
          <span class="material-symbols-outlined text-error text-sm">delete</span>
        </button>
      </div>
    </div>
  </div>
</template>
