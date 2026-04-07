<script setup lang="ts">
import type { Project } from '@/types/project'
import ProjectCard from './ProjectCard.vue'

defineProps<{
  projects: Project[]
}>()
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <ProjectCard
      v-for="project in projects"
      :key="project.id"
      :project="project"
    />
    <div
      v-if="projects.length === 0"
      class="col-span-full text-center py-16 text-on-surface-variant"
    >
      <span class="material-symbols-outlined text-6xl opacity-30">folder_open</span>
      <p class="mt-4 text-lg font-headline">暂无项目</p>
      <p class="text-sm opacity-70">创建你的第一个项目开始吧</p>
    </div>
  </div>
</template>
