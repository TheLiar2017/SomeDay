<script setup lang="ts">
import { onMounted } from 'vue'
import AppInput from '@/components/common/AppInput.vue'
import ArchiveList from '@/components/archive/ArchiveList.vue'
import { useArchiveStore } from '@/stores/archiveStore'

const archiveStore = useArchiveStore()

onMounted(() => {
  archiveStore.loadArchived()
})
</script>

<template>
  <div class="max-w-5xl mx-auto">
    <!-- Header -->
    <header class="mb-12">
      <h1 class="text-5xl font-extrabold tracking-tight text-on-surface mb-4">归档</h1>
      <p class="text-on-surface-variant text-lg max-w-2xl">
        已完成的任务和项目会显示在这里。
      </p>
    </header>

    <!-- Stats and search -->
    <div class="grid grid-cols-1 lg:grid-cols-4 gap-8 mb-8">
      <!-- Stats sidebar -->
      <div class="lg:col-span-1 space-y-6">
        <div class="bg-primary-container/10 p-8 rounded-2xl border border-primary/5">
          <span class="material-symbols-outlined text-primary text-3xl mb-4">auto_awesome</span>
          <h4 class="text-xl font-headline font-bold text-on-surface mb-2">归档摘要</h4>
          <p class="text-sm text-on-surface-variant leading-relaxed">
            您目前已完成并归档了
            <span class="font-bold text-primary">{{ archiveStore.archiveStats.totalTasks }}</span>
            个任务和
            <span class="font-bold text-primary">{{ archiveStore.archiveStats.totalProjects }}</span>
            个项目。
          </p>
        </div>

        <div class="p-6 rounded-2xl bg-surface-container-low">
          <h4 class="text-sm font-bold text-on-surface mb-4 uppercase tracking-widest">清理建议</h4>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-xs font-medium text-on-surface-variant">超过 1 年的内容</span>
              <span class="text-xs bg-surface-container-highest px-2 py-1 rounded">
                {{ Math.floor(archiveStore.archiveStats.totalTasks / 3) }} 个
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Archive content -->
      <div class="lg:col-span-3">
        <!-- Search -->
        <div class="mb-6">
          <AppInput
            v-model="archiveStore.searchQuery"
            placeholder="搜索归档内容..."
            icon="search"
          />
        </div>

        <!-- List -->
        <ArchiveList />
      </div>
    </div>
  </div>
</template>
