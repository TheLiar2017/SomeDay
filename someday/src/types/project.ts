export type ProjectStatus = 'active' | 'completed' | 'archived'

export interface Project {
  id: string
  name: string
  description?: string
  status: ProjectStatus
  progress: number
  coverImage?: string
  tags: string[]
  taskIds: string[]
  dueDate?: string
  createdAt: string
  updatedAt: string
  completedAt?: string
  archivedAt?: string
}

export interface ProjectCreateInput {
  name: string
  description?: string
  coverImage?: string
  tags?: string[]
  dueDate?: string
}

export interface ProjectUpdateInput {
  name?: string
  description?: string
  status?: ProjectStatus
  progress?: number
  coverImage?: string
  tags?: string[]
  dueDate?: string
}
