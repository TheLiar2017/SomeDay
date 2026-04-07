import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { UserSettings, Theme, Language } from '@/types/settings'
import { invoke } from '@tauri-apps/api/core'

const defaultSettings: UserSettings = {
  theme: 'system',
  language: 'zh-CN',
  notifications: {
    taskReminders: true,
    projectUpdates: false,
    dailyDigest: false,
  },
  profile: {
    name: 'Alex Rivera',
    title: '策展人',
    email: '',
    avatar: '',
  },
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<UserSettings>({ ...defaultSettings })

  watch(() => settings.value.theme, (theme) => {
    const root = document.documentElement
    if (theme === 'dark') {
      root.classList.add('dark')
    } else if (theme === 'light') {
      root.classList.remove('dark')
    } else {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.classList.toggle('dark', prefersDark)
    }
  }, { immediate: true })

  async function loadSettings() {
    try {
      const loaded = await invoke<UserSettings>('load_settings')
      settings.value = { ...defaultSettings, ...loaded }
    } catch {
      // Use defaults
    }
  }

  async function saveSettings() {
    try {
      await invoke('save_settings', { settings: settings.value })
    } catch {
      // Ignore save errors
    }
  }

  function setTheme(theme: Theme) {
    settings.value.theme = theme
    saveSettings()
  }

  function setLanguage(language: Language) {
    settings.value.language = language
    saveSettings()
  }

  function updateProfile(profile: Partial<UserSettings['profile']>) {
    settings.value.profile = { ...settings.value.profile, ...profile }
    saveSettings()
  }

  function toggleNotification(key: keyof UserSettings['notifications']) {
    settings.value.notifications[key] = !settings.value.notifications[key]
    saveSettings()
  }

  return {
    settings,
    loadSettings,
    saveSettings,
    setTheme,
    setLanguage,
    updateProfile,
    toggleNotification,
  }
})
