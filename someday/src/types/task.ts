export type TaskPriority = 'low' | 'medium' | 'high'
export type TaskStatus = 'pending' | 'completed' | 'archived'

export interface Task {
  id: string
  title: string
  description?: string
  priority: TaskPriority
  status: TaskStatus
  dueDate?: string
  projectId?: string
  tags: string[]
  createdAt: string
  updatedAt: string
  completedAt?: string
  archivedAt?: string
}

export interface TaskCreateInput {
  title: string
  description?: string
  priority?: TaskPriority
  dueDate?: string
  projectId?: string
  tags?: string[]
}

export interface TaskUpdateInput {
  title?: string
  description?: string
  priority?: TaskPriority
  status?: TaskStatus
  dueDate?: string
  projectId?: string
  tags?: string[]
}
