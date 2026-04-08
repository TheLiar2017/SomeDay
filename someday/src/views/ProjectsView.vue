<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format } from 'date-fns'
import AppButton from '@/components/common/AppButton.vue'
import AppModal from '@/components/common/AppModal.vue'
import AppInput from '@/components/common/AppInput.vue'
import { useProjectStore } from '@/stores/projectStore'
import { useTaskStore } from '@/stores/taskStore'
import type { Project, Task } from '@/types'

const projectStore = useProjectStore()
const taskStore = useTaskStore()

const showCreateModal = ref(false)
const newProjectName = ref('')
const newProjectDesc = ref('')
const selectedProjectId = ref<string | null>(null)
const newTaskTitle = ref('')

onMounted(() => {
  projectStore.loadProjects()
  taskStore.loadTasks()
})

const selectedProject = computed(() => {
  if (!selectedProjectId.value) return null
  return projectStore.activeProjects.find(p => p.id === selectedProjectId.value) || null
})

const selectedProjectTasks = computed(() => {
  if (!selectedProjectId.value) return []
  return taskStore.tasksByProject.get(selectedProjectId.value) || []
})

const pendingTasks = computed(() =>
  selectedProjectTasks.value.filter(t => t.status === 'pending')
)

const completedTasks = computed(() =>
  selectedProjectTasks.value.filter(t => t.status !== 'pending')
)

const projectStats = computed(() => {
  const activeCount = projectStore.activeProjects.length
  let totalCompletion = 0
  projectStore.activeProjects.forEach(p => {
    const tasks = taskStore.tasksByProject.get(p.id) || []
    if (tasks.length > 0) {
      const completed = tasks.filter(t => t.status !== 'pending').length
      totalCompletion += Math.round((completed / tasks.length) * 100)
    }
  })
  const avgCompletion = activeCount > 0 ? Math.round(totalCompletion / activeCount) : 0
  return { activeCount, avgCompletion }
})

const nextDueProject = computed(() => {
  // Find the project with nearest due date among pending tasks
  let nearestTask: Task | null = null
  let nearestDate = Infinity

  const tasks = taskStore.pendingTasks
  for (const task of tasks) {
    if (task.projectId && task.dueDate) {
      const dueTime = new Date(task.dueDate).getTime()
      if (dueTime < nearestDate) {
        nearestDate = dueTime
        nearestTask = task
      }
    }
  }

  if (!nearestTask) return null
  const project = projectStore.getProjectById(nearestTask.projectId!)
  return {
    project,
    task: nearestTask,
    daysLeft: Math.ceil((nearestDate - Date.now()) / (1000 * 60 * 60 * 24))
  }
})

function selectProject(project: Project) {
  selectedProjectId.value = project.id
}

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

async function createTask() {
  if (!newTaskTitle.value.trim() || !selectedProjectId.value) return
  await taskStore.createTask({
    title: newTaskTitle.value,
    projectId: selectedProjectId.value,
  })
  newTaskTitle.value = ''
}

async function archiveProject(projectId: string) {
  await projectStore.archiveProject(projectId)
  if (selectedProjectId.value === projectId) {
    selectedProjectId.value = null
  }
}

function getProjectProgress(projectId: string): number {
  const tasks = taskStore.tasksByProject.get(projectId) || []
  if (tasks.length === 0) return 0
  const completed = tasks.filter(t => t.status !== 'pending').length
  return Math.round((completed / tasks.length) * 100)
}

function getTaskCount(projectId: string): { completed: number; pending: number } {
  const tasks = taskStore.tasksByProject.get(projectId) || []
  return {
    completed: tasks.filter(t => t.status !== 'pending').length,
    pending: tasks.filter(t => t.status === 'pending').length
  }
}

const priorityColors: Record<string, string> = {
  high: 'bg-error/10 text-error',
  medium: 'bg-secondary-fixed text-on-secondary-fixed',
  low: 'bg-primary-fixed text-on-primary-fixed'
}

function formatTaskDate(dateStr: string | undefined): string {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  const today = new Date()
  const tomorrow = new Date(today)
  tomorrow.setDate(tomorrow.getDate() + 1)

  if (format(date, 'yyyy-MM-dd') === format(today, 'yyyy-MM-dd')) return '今天'
  if (format(date, 'yyyy-MM-dd') === format(tomorrow, 'yyyy-MM-dd')) return '明天'
  return format(date, 'MM月dd日')
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="mb-8">
      <h1 class="text-4xl font-extrabold tracking-tight text-on-surface mb-2">项目管理</h1>
      <p class="text-on-surface-variant">The Digital Curator: 精选并管理您的所有核心任务。</p>
    </header>

    <!-- Stats Overview -->
    <div class="grid grid-cols-12 gap-6 mb-8">
      <!-- Active Projects Overview -->
      <div class="col-span-8 bg-surface-container-lowest p-6 rounded-xl flex items-center justify-between border border-outline/10">
        <div>
          <h3 class="text-on-surface-variant text-[10px] font-bold tracking-widest uppercase mb-1">活跃项目总览</h3>
          <div class="flex items-end gap-3">
            <span class="text-4xl font-black text-primary">{{ projectStats.activeCount }}</span>
            <span class="text-on-surface-variant text-sm mb-1">个进行中项目</span>
          </div>
        </div>
        <div class="h-12 w-px bg-outline/20 hidden md:block"></div>
        <div class="hidden md:block">
          <h3 class="text-on-surface-variant text-[10px] font-bold tracking-widest uppercase mb-1">完成率</h3>
          <div class="flex items-end gap-2">
            <span class="text-2xl font-bold text-on-surface">{{ projectStats.avgCompletion }}%</span>
            <div class="w-24 h-1.5 bg-surface-container-high rounded-full overflow-hidden mb-1.5">
              <div class="h-full bg-gradient-to-r from-primary to-primary-container rounded-full" :style="{ width: `${projectStats.avgCompletion}%` }"></div>
            </div>
          </div>
        </div>
        <button
          @click="showCreateModal = true"
          class="bg-gradient-to-r from-primary to-primary-container text-on-primary px-5 py-2.5 rounded-full font-semibold text-xs flex items-center shadow-cloud hover:brightness-110 transition-all"
        >
          <span class="material-symbols-outlined mr-2 text-[18px]">add</span>
          新建项目
        </button>
      </div>

      <!-- Next Due Date -->
      <div class="col-span-4 bg-gradient-to-br from-primary to-primary-container text-on-primary p-6 rounded-xl flex flex-col justify-between">
        <span class="material-symbols-outlined text-2xl">auto_awesome</span>
        <div v-if="nextDueProject">
          <p class="text-on-primary/80 text-[10px] uppercase font-bold tracking-wider mb-0.5">下个截止日期</p>
          <p class="text-lg font-bold">{{ nextDueProject.project?.name }}</p>
          <p class="text-[11px] mt-0.5 opacity-90">剩余 {{ nextDueProject.daysLeft }} 天</p>
        </div>
        <div v-else>
          <p class="text-on-primary/80 text-[10px] uppercase font-bold tracking-wider mb-0.5">暂无待办</p>
          <p class="text-sm opacity-90">所有任务已完成</p>
        </div>
      </div>
    </div>

    <!-- Dual Pane Content Area -->
    <div class="flex-1 flex gap-8 overflow-hidden" style="max-height: calc(100vh - 340px);">
      <!-- Left: Project Cards (Scrollable) -->
      <div class="w-1/2 flex flex-col gap-4 overflow-y-auto pr-4 custom-scrollbar">
        <div
          v-for="project in projectStore.activeProjects"
          :key="project.id"
          :class="[
            'bg-surface-container-lowest rounded-xl p-5 transition-all duration-300 cursor-pointer group relative',
            selectedProjectId === project.id
              ? 'border-2 border-primary shadow-sm'
              : 'border border-outline/10 hover:border-outline/30'
          ]"
          @click="selectProject(project)"
        >
          <div class="flex justify-between items-start mb-4">
            <div class="p-2.5 bg-secondary-fixed text-on-secondary-fixed rounded-lg">
              <span class="material-symbols-outlined text-[20px]">folder</span>
            </div>
            <span class="text-[10px] font-bold text-on-surface-variant bg-surface-container-high px-2 py-0.5 rounded">
              {{ project.taskIds?.length || 0 }} 任务
            </span>
          </div>
          <h3 class="text-base font-bold text-on-surface mb-1">{{ project.name }}</h3>
          <p v-if="project.description" class="text-on-surface-variant text-xs mb-3 line-clamp-2">
            {{ project.description }}
          </p>
          <div class="space-y-2">
            <div class="flex justify-between items-center text-[11px]">
              <span class="text-on-surface-variant font-medium">
                {{ getTaskCount(project.id).completed }} 已完成 / {{ getTaskCount(project.id).pending }} 剩余
              </span>
              <span :class="['font-bold', getProjectProgress(project.id) >= 100 ? 'text-primary' : 'text-primary/50']">
                {{ getProjectProgress(project.id) }}%
              </span>
            </div>
            <div class="w-full h-1 bg-surface-container-high rounded-full overflow-hidden">
              <div
                :class="['h-full rounded-full transition-all', getProjectProgress(project.id) >= 100 ? 'bg-primary' : 'bg-primary/50']"
                :style="{ width: `${getProjectProgress(project.id)}%` }"
              ></div>
            </div>
          </div>
          <div v-if="selectedProjectId === project.id" class="absolute right-4 top-1/2 -translate-y-1/2 text-primary">
            <span class="material-symbols-outlined">chevron_right</span>
          </div>
          <div class="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              @click.stop="archiveProject(project.id)"
              class="p-1.5 hover:bg-surface-container-high rounded-full transition-colors"
              title="归档项目"
            >
              <span class="material-symbols-outlined text-on-surface-variant text-sm">archive</span>
            </button>
          </div>
        </div>

        <!-- Empty State -->
        <div
          v-if="projectStore.activeProjects.length === 0"
          class="text-center py-16 text-on-surface-variant"
        >
          <span class="material-symbols-outlined text-6xl opacity-30">folder_open</span>
          <p class="mt-4 text-lg font-headline">暂无项目</p>
          <p class="text-sm opacity-70">创建你的第一个项目开始吧</p>
        </div>
      </div>

      <!-- Right: Task List Pane -->
      <div class="w-1/2 flex flex-col bg-surface-container-lowest rounded-xl border border-outline/10 overflow-hidden">
        <!-- Task List Header -->
        <div class="p-6 border-b border-outline/10 flex justify-between items-center bg-surface-bright/50">
          <div v-if="selectedProject">
            <h3 class="text-lg font-extrabold text-on-surface">{{ selectedProject.name }} · 任务清单</h3>
            <p class="text-[11px] text-on-surface-variant flex items-center mt-1">
              <span class="material-symbols-outlined text-[14px] mr-1">check_circle</span>
              {{ completedTasks.length }} 项任务已完成
              <span class="mx-2 opacity-30">|</span>
              <span class="material-symbols-outlined text-[14px] mr-1">pending</span>
              {{ pendingTasks.length }} 项任务进行中
            </p>
          </div>
          <div v-else>
            <h3 class="text-lg font-extrabold text-on-surface opacity-50">选择项目</h3>
            <p class="text-[11px] text-on-surface-variant mt-1">点击左侧项目查看任务</p>
          </div>
        </div>

        <!-- Task List Content -->
        <div class="flex-1 overflow-y-auto p-6 space-y-4 custom-scrollbar">
          <!-- In Progress Tasks -->
          <div v-if="pendingTasks.length > 0" class="mb-6">
            <h4 class="text-[10px] font-bold text-primary tracking-widest uppercase mb-4 px-2">进行中</h4>
            <div class="space-y-2">
              <div
                v-for="task in pendingTasks"
                :key="task.id"
                class="flex items-center p-3 bg-primary/5 rounded-lg border border-primary/10 group"
              >
                <div class="w-5 h-5 border-2 border-primary/30 rounded flex items-center justify-center cursor-pointer mr-4 bg-white"></div>
                <div class="flex-1">
                  <p class="text-sm font-semibold text-on-surface">{{ task.title }}</p>
                  <div class="flex items-center gap-3 mt-1">
                    <span :class="['text-[10px] px-1.5 py-0.5 rounded font-bold', priorityColors[task.priority]]">
                      {{ task.priority === 'high' ? '高优先级' : task.priority === 'medium' ? '中优先级' : '低优先级' }}
                    </span>
                    <span v-if="task.dueDate" class="text-[10px] text-on-surface-variant flex items-center">
                      <span class="material-symbols-outlined text-[12px] mr-1">schedule</span>
                      {{ formatTaskDate(task.dueDate) }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Completed Tasks -->
          <div v-if="completedTasks.length > 0">
            <h4 class="text-[10px] font-bold text-on-surface-variant/50 tracking-widest uppercase mb-4 px-2">已完成</h4>
            <div class="space-y-2">
              <div
                v-for="task in completedTasks"
                :key="task.id"
                class="flex items-center p-3 opacity-60 grayscale-[0.5] group"
              >
                <div class="w-5 h-5 bg-primary rounded flex items-center justify-center cursor-pointer mr-4">
                  <span class="material-symbols-outlined text-white text-[16px]">check</span>
                </div>
                <div class="flex-1">
                  <p class="text-sm font-medium text-on-surface line-through">{{ task.title }}</p>
                  <p v-if="task.completedAt" class="text-[10px] text-on-surface-variant mt-0.5">
                    已于 {{ format(new Date(task.completedAt), 'MM月dd日') }} 完成
                  </p>
                </div>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div
            v-if="!selectedProject"
            class="flex-1 flex items-center justify-center text-center py-16"
          >
            <div>
              <span class="material-symbols-outlined text-6xl opacity-20">folder_open</span>
              <p class="mt-4 text-on-surface-variant">从左侧选择一个项目</p>
            </div>
          </div>
          <div
            v-else-if="selectedProjectTasks.length === 0"
            class="flex-1 flex items-center justify-center text-center py-16"
          >
            <div>
              <span class="material-symbols-outlined text-6xl opacity-20">task_alt</span>
              <p class="mt-4 text-on-surface-variant">该项目暂无任务</p>
            </div>
          </div>
        </div>

        <!-- Quick Add Task -->
        <div v-if="selectedProject" class="p-4 bg-surface-bright/50 border-t border-outline/10">
          <form @submit.prevent="createTask" class="relative flex items-center">
            <span class="material-symbols-outlined absolute left-3 text-outline text-[20px]">add</span>
            <input
              v-model="newTaskTitle"
              class="w-full pl-10 pr-4 py-2 bg-surface-container-lowest border border-outline/20 rounded-lg text-sm focus:ring-1 focus:ring-primary focus:border-primary outline-none transition-all"
              placeholder="添加新任务..."
              type="text"
            />
          </form>
        </div>
      </div>
    </div>
  </div>

  <!-- Create Project Modal -->
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

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: theme('colors.outline-variant');
  border-radius: 10px;
}
</style>
