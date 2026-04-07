<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ProjectList from '@/components/projects/ProjectList.vue'
import AppButton from '@/components/common/AppButton.vue'
import AppModal from '@/components/common/AppModal.vue'
import AppInput from '@/components/common/AppInput.vue'
import { useProjectStore } from '@/stores/projectStore'
import { useTaskStore } from '@/stores/taskStore'

const projectStore = useProjectStore()
const taskStore = useTaskStore()

const showCreateModal = ref(false)
const newProjectName = ref('')
const newProjectDesc = ref('')

onMounted(() => {
  projectStore.loadProjects()
  taskStore.loadTasks()
})

async function createProject() {
  if (!newProjectName.value.trim()) return
  await projectStore.createProject({
    name: newProjectName.value,
    description: newProjectDesc.value || undefined,
  })
  newProjectName.value = ''
  newProjectDesc.value = ''
  showCreateModal.value = false
}
</script>

<template>
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <header class="mb-12">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-5xl font-extrabold tracking-tight text-on-surface mb-4">项目</h1>
          <p class="text-on-surface-variant text-lg">管理你的个人项目组合</p>
        </div>
        <AppButton @click="showCreateModal = true">
          <span class="material-symbols-outlined text-sm">add</span>
          新建项目
        </AppButton>
      </div>
    </header>

    <!-- Project list -->
    <ProjectList :projects="projectStore.activeProjects" />
  </div>

  <!-- Create modal -->
  <AppModal v-if="showCreateModal" title="新建项目" @close="showCreateModal = false">
    <form @submit.prevent="createProject" class="space-y-6">
      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          项目名称 *
        </label>
        <AppInput
          v-model="newProjectName"
          placeholder="输入项目名称..."
          icon="folder"
        />
      </div>

      <div>
        <label class="block text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-2">
          描述
        </label>
        <textarea
          v-model="newProjectDesc"
          placeholder="添加项目描述..."
          rows="3"
          class="w-full input-soft resize-none"
        ></textarea>
      </div>
    </form>

    <template #footer>
      <div class="flex justify-end gap-3">
        <AppButton variant="secondary" @click="showCreateModal = false">取消</AppButton>
        <AppButton
          variant="primary"
          :disabled="!newProjectName.trim()"
          @click="createProject"
        >
          创建项目
        </AppButton>
      </div>
    </template>
  </AppModal>
</template>
