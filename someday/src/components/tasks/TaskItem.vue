<script setup lang="ts">
import { computed } from 'vue'
import type { Task } from '@/types/task'
import AppCheckbox from '@/components/common/AppCheckbox.vue'
import { useTaskStore } from '@/stores/taskStore'

const props = defineProps<{
  task: Task
  disableToggle?: boolean
}>()

const emit = defineEmits<{
  'task-click': [task: Task, mouseEvent: MouseEvent]
}>()

const taskStore = useTaskStore()

const isCompleted = computed(() => props.task.status !== 'pending')

async function handleCheckboxClick() {
  if (isCompleted.value) {
    await taskStore.updateTask(props.task.id, { status: 'pending' })
  } else {
    await taskStore.completeTask(props.task.id)
  }
}

function handleTitleClick(event: MouseEvent) {
  emit('task-click', props.task, event)
}
</script>

<template>
  <div class="flex items-center gap-2 py-2 px-3 rounded-lg hover:bg-surface-container-low transition-colors group">
    <div @click="handleCheckboxClick">
      <AppCheckbox :checked="isCompleted" />
    </div>
    <span :class="['w-2 h-2 rounded-full', task.priority === 'high' ? 'bg-error' : task.priority === 'medium' ? 'bg-tertiary' : task.priority === 'low' ? 'bg-secondary' : 'bg-outline']"></span>
    <span
      :class="[
        'text-sm flex-1 truncate cursor-pointer',
        isCompleted ? 'line-through text-on-surface-variant' : 'text-on-surface',
      ]"
      @click="handleTitleClick"
    >
      {{ task.title }}
    </span>
  </div>
</template>
