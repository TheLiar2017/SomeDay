<script setup lang="ts">
import { onMounted } from 'vue'
import AppInput from '@/components/common/AppInput.vue'
import { useSettingsStore } from '@/stores/settingsStore'

const settingsStore = useSettingsStore()

const themes = [
  { key: 'light', label: '亮色', icon: 'light_mode' },
  { key: 'dark', label: '暗色', icon: 'dark_mode' },
  { key: 'system', label: '系统', icon: 'settings_brightness' },
] as const

const languages = [
  { key: 'zh-CN', label: '简体中文' },
  { key: 'en-US', label: 'English' },
]

onMounted(() => {
  settingsStore.loadSettings()
})
</script>

<template>
  <div class="max-w-5xl mx-auto">
    <!-- Header -->
    <header class="mb-16">
      <h1 class="text-5xl font-extrabold tracking-tight text-on-surface mb-4">设置</h1>
      <p class="text-on-surface-variant text-lg max-w-2xl">
        管理您的账户信息、个性化偏好及通知设置。
      </p>
    </header>

    <div class="grid grid-cols-1 gap-12">
      <!-- Profile Section -->
      <section>
        <h3 class="text-2xl font-bold mb-8 text-on-surface flex items-center gap-2">
          <span class="material-symbols-outlined text-primary">person</span>
          个人资料
        </h3>
        <div class="bg-surface-container-lowest rounded-xl p-8 flex flex-col md:flex-row gap-12 items-start hover:shadow-cloud transition-shadow">
          <!-- Avatar -->
          <div class="relative group">
            <div class="w-32 h-32 rounded-full bg-primary-container flex items-center justify-center text-on-primary text-4xl font-bold">
              {{ settingsStore.settings.profile.name.charAt(0) }}
            </div>
            <button class="absolute bottom-0 right-0 bg-primary text-white p-2 rounded-full shadow-lg hover:scale-105 transition-transform">
              <span class="material-symbols-outlined text-sm">photo_camera</span>
            </button>
          </div>

          <!-- Form -->
          <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-6 w-full">
            <div class="flex flex-col gap-2">
              <label class="text-xs font-semibold text-on-surface-variant uppercase tracking-wider">用户名</label>
              <AppInput
                :modelValue="settingsStore.settings.profile.name"
                @update:modelValue="settingsStore.updateProfile({ name: $event })"
              />
            </div>
            <div class="flex flex-col gap-2">
              <label class="text-xs font-semibold text-on-surface-variant uppercase tracking-wider">职业头衔</label>
              <AppInput
                :modelValue="settingsStore.settings.profile.title || ''"
                @update:modelValue="settingsStore.updateProfile({ title: $event })"
              />
            </div>
            <div class="flex flex-col gap-2 md:col-span-2">
              <label class="text-xs font-semibold text-on-surface-variant uppercase tracking-wider">电子邮件</label>
              <AppInput
                :modelValue="settingsStore.settings.profile.email || ''"
                @update:modelValue="settingsStore.updateProfile({ email: $event })"
                type="email"
              />
            </div>
          </div>
        </div>
      </section>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
        <!-- Preferences Section -->
        <section>
          <h3 class="text-2xl font-bold mb-8 text-on-surface flex items-center gap-2">
            <span class="material-symbols-outlined text-primary">palette</span>
            偏好设置
          </h3>
          <div class="bg-surface-container-lowest rounded-xl p-8 space-y-8 h-full">
            <!-- Language -->
            <div class="flex items-center justify-between">
              <div>
                <p class="font-semibold text-on-surface">显示语言</p>
                <p class="text-sm text-on-surface-variant">选择界面的默认语言</p>
              </div>
              <select
                :value="settingsStore.settings.language"
                @change="settingsStore.setLanguage(($event.target as HTMLSelectElement).value as 'zh-CN' | 'en-US')"
                class="bg-surface-container-highest border-none rounded-full py-2 px-6 text-sm font-medium focus:ring-1 focus:ring-primary/40"
              >
                <option v-for="lang in languages" :key="lang.key" :value="lang.key">
                  {{ lang.label }}
                </option>
              </select>
            </div>

            <!-- Theme -->
            <div class="flex items-center justify-between">
              <div>
                <p class="font-semibold text-on-surface">外观主题</p>
                <p class="text-sm text-on-surface-variant">根据您的光线环境进行调整</p>
              </div>
              <div class="flex bg-surface-container-highest rounded-full p-1">
                <button
                  v-for="theme in themes"
                  :key="theme.key"
                  :class="[
                    'p-2 px-4 rounded-full flex items-center gap-2 transition-all',
                    settingsStore.settings.theme === theme.key
                      ? 'bg-surface-container-lowest shadow-sm'
                      : 'opacity-50 hover:opacity-100',
                  ]"
                  @click="settingsStore.setTheme(theme.key)"
                >
                  <span class="material-symbols-outlined text-sm">{{ theme.icon }}</span>
                  <span class="text-xs font-bold">{{ theme.label }}</span>
                </button>
              </div>
            </div>
          </div>
        </section>

        <!-- Notifications Section -->
        <section>
          <h3 class="text-2xl font-bold mb-8 text-on-surface flex items-center gap-2">
            <span class="material-symbols-outlined text-primary">notifications</span>
            通知
          </h3>
          <div class="bg-surface-container-lowest rounded-xl p-8 space-y-6 h-full">
            <label
              v-for="(value, key) in settingsStore.settings.notifications"
              :key="key"
              class="flex items-center justify-between cursor-pointer group"
            >
              <div>
                <p class="font-semibold text-on-surface group-hover:text-primary transition-colors">
                  {{ key === 'taskReminders' ? '任务提醒' : key === 'projectUpdates' ? '项目更新' : '每日摘要' }}
                </p>
                <p class="text-sm text-on-surface-variant">
                  {{ key === 'taskReminders' ? '关于任务截止日期的即时提醒' : key === 'projectUpdates' ? '项目状态变化时收到通知' : '每天早晨发送今日概览邮件' }}
                </p>
              </div>
              <button
                role="switch"
                :aria-checked="value"
                :class="[
                  'relative inline-flex items-center w-11 h-6 rounded-full transition-colors',
                  value ? 'bg-primary-container' : 'bg-surface-container-highest',
                ]"
                @click="settingsStore.toggleNotification(key as keyof typeof settingsStore.settings.notifications)"
              >
                <span
                  :class="[
                    'w-5 h-5 bg-white rounded-full shadow-sm transition-transform',
                    value ? 'translate-x-6' : 'translate-x-1',
                  ]"
                ></span>
              </button>
            </label>
          </div>
        </section>
      </div>

      <!-- About Section -->
      <section>
        <h3 class="text-2xl font-bold mb-8 text-on-surface flex items-center gap-2">
          <span class="material-symbols-outlined text-primary">info</span>
          关于
        </h3>
        <div class="bg-primary bg-gradient-to-br from-primary to-primary-container rounded-xl p-8 text-white relative overflow-hidden group">
          <div class="relative z-10">
            <p class="text-xs uppercase tracking-widest font-bold opacity-70 mb-2">应用版本</p>
            <p class="text-4xl font-extrabold mb-6">v1.0.0</p>
            <div class="space-y-3">
              <p class="flex items-center justify-between text-sm">
                <span>Someday · The Curator</span>
              </p>
            </div>
          </div>
          <div class="absolute -bottom-8 -right-8 w-32 h-32 bg-white/10 rounded-full blur-2xl"></div>
        </div>
      </section>
    </div>
  </div>
</template>
