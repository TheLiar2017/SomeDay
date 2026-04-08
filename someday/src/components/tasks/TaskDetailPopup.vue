<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import type { Task } from '@/types/task'
import AppCheckbox from '@/components/common/AppCheckbox.vue'
import { useTaskStore } from '@/stores/taskStore'
import { useProjectStore } from '@/stores/projectStore'

const props = defineProps<{
  task: Task
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const taskStore = useTaskStore()
const projectStore = useProjectStore()

let debounceTimer: ReturnType<typeof setTimeout> | null = null

const editTitle = ref('')
const editDescription = ref('')
const editPriority = ref<'low' | 'medium' | 'high'>('medium')
const editDueDate = ref('')
const editProjectId = ref('')

watch(() => props.task, (newTask) => {
  if (newTask) {
    editTitle.value = newTask.title
    editDescription.value = newTask.description || ''
    editPriority.value = newTask.priority
    editDueDate.value = newTask.dueDate || ''
    editProjectId.value = newTask.projectId || ''
  }
}, { immediate: true })

const isCompleted = computed(() => props.task.status !== 'pending')

const priorityLabels = {
  low: '低',
  medium: '中',
  high: '高'
}

const priorityColors = {
  low: 'bg-secondary-container text-on-secondary-container',
  medium: 'bg-tertiary-container text-on-tertiary-container',
  high: 'bg-error-container text-on-error-container'
}

const priorityDotColors = {
  low: 'bg-secondary',
  medium: 'bg-tertiary',
  high: 'bg-error'
}

async function debouncedUpdate(updates: Record<string, any>) {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(async () => {
    await taskStore.updateTask(props.task.id, updates)
  }, 300)
}

async function toggleComplete() {
  if (isCompleted.value) {
    await taskStore.updateTask(props.task.id, { status: 'pending' })
  } else {
    await taskStore.completeTask(props.task.id)
  }
}

function onTitleChange() {
  if (editTitle.value.trim() !== props.task.title) {
    debouncedUpdate({ title: editTitle.value.trim() || props.task.title })
  }
}

function onDescriptionChange() {
  debouncedUpdate({ description: editDescription.value || undefined })
}

function onPriorityChange(priority: 'low' | 'medium' | 'high') {
  editPriority.value = priority
  debouncedUpdate({ priority })
}

function onDueDateChange() {
  debouncedUpdate({ dueDate: editDueDate.value || undefined })
}

function onProjectChange() {
  debouncedUpdate({ projectId: editProjectId.value || undefined })
}

onMounted(() => {
  projectStore.loadProjects()
})
</script>

<template>
  <div
    v-if="visible"
    class="absolute z-50 bg-surface-container-lowest rounded-xl shadow-elevation3 border border-outline/10 overflow-hidden"
    style="min-width: 400px; max-width: 500px;"
  >
    <!-- Header -->
    <div class="flex items-center gap-3 px-4 py-3 border-b border-outline/10">
      <AppCheckbox :checked="isCompleted" @update:checked="toggleComplete" />
      <div class="flex items-center gap-2 flex-1">
        <span :class="['w-2 h-2 rounded-full', priorityDotColors[props.task.priority]]"></span>
        <input
          v-model="editTitle"
          class="flex-1 px-2 py-1 bg-transparent rounded text-on-surface font-medium focus:outline-none focus:ring-1 focus:ring-primary hover:bg-surface-container-high focus:bg-surface-container-high transition-colors"
          @blur="onTitleChange"
          @keydown.enter="($event.target as HTMLInputElement).blur()"
        />
      </div>
      <button
        @click="emit('close')"
        class="p-1 hover:bg-surface-container-high rounded-full transition-colors"
      >
        <span class="material-symbols-outlined text-on-surface-variant text-sm">close</span>
      </button>
    </div>

    <!-- Content -->
    <div class="p-4 space-y-4">
      <!-- Priority -->
      <div class="flex items-center gap-3">
        <span class="material-symbols-outlined text-on-surface-variant text-sm">flag</span>
        <div class="flex gap-2">
          <button
            v-for="(label, key) in priorityLabels"
            :key="key"
            :class="[
              'px-3 py-1 rounded-full text-xs font-medium transition-all',
              editPriority === key
                ? priorityColors[key]
                : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container'
            ]"
            @click="onPriorityChange(key as 'low' | 'medium' | 'high')"
          >
            {{ label }}
          </button>
        </div>
      </div>

      <!-- Due Date -->
      <div class="flex items-center gap-3">
        <span class="material-symbols-outlined text-on-surface-variant text-sm">calendar_today</span>
        <input
          type="date"
          v-model="editDueDate"
          class="px-2 py-1 bg-surface-container-high rounded text-on-surface text-sm focus:outline-none focus:ring-1 focus:ring-primary"
          @change="onDueDateChange"
        />
      </div>

      <!-- Project -->
      <div class="flex items-center gap-3">
        <span class="material-symbols-outlined text-on-surface-variant text-sm">folder</span>
        <select
          v-model="editProjectId"
          class="px-2 py-1 bg-surface-container-high rounded text-on-surface text-sm focus:outline-none focus:ring-1 focus:ring-primary"
          @change="onProjectChange"
        >
          <option value="">无</option>
          <option
            v-for="project in projectStore.activeProjects"
            :key="project.id"
            :value="project.id"
          >
            {{ project.name }}
          </option>
        </select>
      </div>

      <!-- Description -->
      <div class="pt-2">
        <textarea
          v-model="editDescription"
          placeholder="添加描述..."
          rows="4"
          class="w-full px-3 py-2 bg-surface-container-high rounded-lg text-on-surface text-sm resize-none focus:outline-none focus:ring-1 focus:ring-primary"
          @blur="onDescriptionChange"
        ></textarea>
      </div>
    </div>
  </div>
</template>
