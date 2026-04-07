<script setup lang="ts">
import { computed } from 'vue'
import { format, isToday } from 'date-fns'
import TaskItem from '@/components/tasks/TaskItem.vue'
import { useUiStore } from '@/stores/uiStore'
import type { Task } from '@/types/task'

const props = defineProps<{
  day: Date
  tasks: Task[]
}>()

const uiStore = useUiStore()

const dayLabel = computed(() => format(props.day, 'EEE'))
const dayNumber = computed(() => format(props.day, 'd'))

function openTaskCreate() {
  uiStore.openTaskCreateModal(format(props.day, 'yyyy-MM-dd'))
}
</script>

<template>
  <div
    :class="[
      'flex flex-col bg-surface rounded-xl min-h-[400px]',
      isToday(day) ? 'ring-2 ring-primary/30' : '',
    ]"
  >
    <!-- Day header -->
    <div class="p-4 border-b border-outline-variant/20">
      <div class="flex items-center gap-3">
        <span
          :class="[
            'text-2xl font-bold font-headline',
            isToday(day) ? 'text-primary' : 'text-on-surface',
          ]"
        >
          {{ dayNumber }}
        </span>
        <span class="text-sm text-on-surface-variant">{{ dayLabel }}</span>
        <span
          v-if="isToday(day)"
          class="px-2 py-0.5 bg-primary text-on-primary text-xs rounded-full font-medium"
        >
          今天
        </span>
      </div>
    </div>

    <!-- Tasks -->
    <div class="flex-1 p-2 space-y-1 overflow-y-auto">
      <TaskItem
        v-for="task in tasks"
        :key="task.id"
        :task="task"
      />
    </div>

    <!-- Add button -->
    <button
      @click="openTaskCreate"
      class="m-2 p-3 border-2 border-dashed border-outline-variant/50 rounded-xl text-on-surface-variant hover:border-primary hover:text-primary transition-colors flex items-center justify-center gap-2"
    >
      <span class="material-symbols-outlined text-sm">add</span>
      <span class="text-sm font-medium">添加任务</span>
    </button>
  </div>
</template>
