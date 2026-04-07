<script setup lang="ts">
import { computed } from 'vue'
import type { Task } from '@/types/task'
import AppCheckbox from '@/components/common/AppCheckbox.vue'
import { useTaskStore } from '@/stores/taskStore'

const props = defineProps<{
  task: Task
}>()

const taskStore = useTaskStore()

const isCompleted = computed(() => props.task.status === 'completed')

async function toggleComplete() {
  if (isCompleted.value) {
    await taskStore.updateTask(props.task.id, { status: 'pending' })
  } else {
    await taskStore.completeTask(props.task.id)
  }
}

const priorityDot = computed(() => {
  switch (props.task.priority) {
    case 'high': return 'bg-error'
    case 'medium': return 'bg-tertiary'
    case 'low': return 'bg-secondary'
    default: return 'bg-outline'
  }
})
</script>

<template>
  <div
    :class="[
      'flex items-center gap-2 py-2 px-3 rounded-lg hover:bg-surface-container-low transition-colors group cursor-pointer',
      isCompleted ? 'opacity-50' : '',
    ]"
    @click="toggleComplete"
  >
    <AppCheckbox :checked="isCompleted" @update:checked="toggleComplete" />
    <span :class="['w-2 h-2 rounded-full', priorityDot]"></span>
    <span
      :class="[
        'text-sm flex-1 truncate',
        isCompleted ? 'line-through text-on-surface-variant' : 'text-on-surface',
      ]"
    >
      {{ task.title }}
    </span>
  </div>
</template>
