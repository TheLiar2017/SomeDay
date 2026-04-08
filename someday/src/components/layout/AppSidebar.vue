<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const navItems = [
  { name: 'dashboard', label: '控制面板', icon: 'dashboard', path: '/' },
  { name: 'calendar', label: '日历', icon: 'calendar_month', path: '/calendar' },
  { name: 'projects', label: '项目', icon: 'folder', path: '/projects' },
  { name: 'stats', label: '统计', icon: 'bar_chart', path: '/stats' },
]

const bottomNavItems = [
  { name: 'archive', label: '归档', icon: 'archive', path: '/archive' },
  { name: 'settings', label: '设置', icon: 'settings', path: '/settings' },
]

function isActive(path: string) {
  return route.path === path
}
</script>

<template>
  <aside class="bg-surface-container-high h-full w-64 fixed left-0 top-0 flex flex-col py-8 z-50">
    <!-- Logo -->
    <div class="px-6 mb-10">
      <h1 class="text-xl font-bold font-headline text-on-surface">SomeDay</h1>
      <p class="text-xs text-on-surface-variant font-medium mt-1">Something attempted, something achieved.</p>
    </div>

    <!-- Main Nav -->
    <nav class="flex-1 space-y-1 px-3">
      <router-link
        v-for="item in navItems"
        :key="item.name"
        :to="item.path"
        :class="['nav-item', isActive(item.path) ? 'nav-item-active' : '']"
      >
        <span class="material-symbols-outlined">{{ item.icon }}</span>
        <span class="font-headline text-sm font-medium">{{ item.label }}</span>
      </router-link>
    </nav>

    <!-- Add Task Button -->
    <!-- <div class="px-6 mb-8">
      <button
        @click="uiStore.openTaskCreateModal()"
        class="w-full py-3 bg-gradient-to-br from-primary to-primary-container text-on-primary rounded-full font-headline font-semibold text-sm shadow-cloud hover:shadow-cloud-hover hover:scale-[1.02] transition-all flex items-center justify-center gap-2"
      >
        <span class="material-symbols-outlined text-sm">add</span>
        快速添加
      </button>
    </div> -->

    <!-- Bottom Nav -->
    <div class="px-3 mt-auto space-y-1 border-t border-outline-variant/20 pt-6">
      <router-link
        v-for="item in bottomNavItems"
        :key="item.name"
        :to="item.path"
        :class="['nav-item', isActive(item.path) ? 'nav-item-active' : '']"
      >
        <span class="material-symbols-outlined">{{ item.icon }}</span>
        <span class="font-headline text-sm font-medium">{{ item.label }}</span>
      </router-link>
    </div>

    <!-- User Profile -->
    <div class="px-6 mt-6">
      <div class="p-4 bg-surface-container rounded-xl">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-primary-container flex items-center justify-center text-on-primary font-bold text-sm">
            A
          </div>
          <div class="overflow-hidden">
            <p class="text-xs font-bold text-on-surface truncate">Alex Rivera</p>
            <p class="text-[10px] text-on-surface-variant truncate">策展人</p>
          </div>
        </div>
      </div>
    </div>
  </aside>
</template>
