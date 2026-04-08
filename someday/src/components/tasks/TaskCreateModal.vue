<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import AppModal from '@/components/common/AppModal.vue'
import AppButton from '@/components/common/AppButton.vue'
import AppInput from '@/components/common/AppInput.vue'
import { useTaskStore } from '@/stores/taskStore'
import { useProjectStore } from '@/stores/projectStore'
import { useUiStore } from '@/stores/uiStore'
import { format } from 'date-fns'

const emit = defineEmits<{
  close: []
}>()

const taskStore = useTaskStore()
const projectStore = useProjectStore()
const uiStore = useUiStore()

const title = ref('')
const description = ref('')
const priority = ref<'low' | 'medium' | 'high'>('medium')
const dueDate = ref('')
const projectId = ref('')
const isSubmitting = ref(false)

onMounted(() => {
  projectStore.loadProjects()
})

const presetDates = computed(() => {
  const dates = []
  const today = new Date()
  for (let i = 0; i < 7; i++) {
    const d = new Date(today)
    d.setDate(today.getDate() + i)
    dates.push({
      label: i === 0 ? '今天' : i === 1 ? '明天' : format(d, 'MM月dd日'),
      value: format(d, 'yyyy-MM-dd'),
    })
  }
  return dates
})

// Pre-fill date if opened from calendar
if (uiStore.activeModalDate) {
  dueDate.value = uiStore.activeModalDate
}

async function handleSubmit() {
  if (!title.value.trim()) return

  isSubmitting.value = true
  try {
    await taskStore.createTask({
      title: title.value,
      description: description.value || undefined,
      priority: priority.value,
      dueDate: dueDate.value || undefined,
      projectId: projectId.value || undefined,
    })
    emit('close')
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <AppModal title="创建任务" @close="emit('close')">
    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          任务标题 *
        </label>
        <AppInput
          v-model="title"
          placeholder="输入任务标题..."
          icon="edit"
        />
      </div>

      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          描述
        </label>
        <textarea
          v-model="description"
          placeholder="添加描述..."
          rows="3"
          class="w-full input-soft resize-none"
        ></textarea>
      </div>

      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          优先级
        </label>
        <div class="flex gap-2">
          <button
            type="button"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all',
              priority === 'low' ? 'bg-secondary-container text-on-secondary-container' : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container',
            ]"
            @click="priority = 'low'"
          >
            低
          </button>
          <button
            type="button"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all',
              priority === 'medium' ? 'bg-tertiary-container text-on-tertiary-container' : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container',
            ]"
            @click="priority = 'medium'"
          >
            中
          </button>
          <button
            type="button"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all',
              priority === 'high' ? 'bg-error-container text-on-error-container' : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container',
            ]"
            @click="priority = 'high'"
          >
            高
          </button>
        </div>
      </div>

      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          截止日期
        </label>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="date in presetDates"
            :key="date.value"
            type="button"
            :class="[
              'px-3 py-1.5 rounded-full text-xs font-medium transition-all',
              dueDate === date.value
                ? 'bg-primary text-on-primary'
                : 'bg-surface-container-high text-on-surface-variant hover:bg-surface-container',
            ]"
            @click="dueDate = date.value"
          >
            {{ date.label }}
          </button>
        </div>
        <input
          type="date"
          v-model="dueDate"
          class="mt-3 input-soft w-full"
        />
      </div>

      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          所属项目
        </label>
        <select
          v-model="projectId"
          class="w-full input-soft bg-surface-container-highest"
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
    </form>

    <template #footer>
      <div class="flex justify-end gap-3">
        <AppButton variant="secondary" @click="emit('close')">取消</AppButton>
        <AppButton
          variant="primary"
          :disabled="!title.trim() || isSubmitting"
          @click="handleSubmit"
        >
          创建任务
        </AppButton>
      </div>
    </template>
  </AppModal>
</template>
