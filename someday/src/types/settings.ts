export type Theme = 'light' | 'dark' | 'system'
export type Language = 'zh-CN' | 'en-US'

export interface UserSettings {
  theme: Theme
  language: Language
  notifications: {
    taskReminders: boolean
    projectUpdates: boolean
    dailyDigest: boolean
  }
  profile: {
    name: string
    title?: string
    email?: string
    avatar?: string
  }
}
