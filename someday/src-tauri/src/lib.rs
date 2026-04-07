use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub tags: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
    #[serde(rename = "archivedAt")]
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub progress: f64,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,
    pub tags: Vec<String>,
    #[serde(rename = "taskIds")]
    pub task_ids: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
    #[serde(rename = "archivedAt")]
    pub archived_at: Option<String>,
}

#[derive(Default)]
pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
    pub projects: Mutex<Vec<Project>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub progress: Option<f64>,
    pub tags: Option<Vec<String>>,
}

// Task Commands
#[tauri::command]
pub fn load_tasks(state: tauri::State<AppState>) -> Result<Vec<Task>, String> {
    let tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    Ok(tasks.clone())
}

#[tauri::command]
pub fn create_task(state: tauri::State<AppState>, input: CreateTaskInput) -> Result<Task, String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let task = Task {
        id: Uuid::new_v4().to_string(),
        title: input.title,
        description: input.description,
        priority: input.priority.unwrap_or_else(|| "medium".to_string()),
        status: "pending".to_string(),
        due_date: input.due_date,
        project_id: input.project_id,
        tags: input.tags.unwrap_or_default(),
        created_at: now.clone(),
        updated_at: now,
        completed_at: None,
        archived_at: None,
    };
    tasks.push(task.clone());
    Ok(task)
}

#[tauri::command]
pub fn update_task(state: tauri::State<AppState>, id: String, input: UpdateTaskInput) -> Result<Task, String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    let task = tasks.iter_mut().find(|t| t.id == id).ok_or("Task not found")?;

    if let Some(title) = input.title { task.title = title; }
    if let Some(desc) = input.description { task.description = Some(desc); }
    if let Some(priority) = input.priority { task.priority = priority; }
    if let Some(status) = input.status {
        task.status = status.clone();
        if status == "completed" {
            task.completed_at = Some(Utc::now().to_rfc3339());
        }
    }
    if let Some(due_date) = input.due_date { task.due_date = Some(due_date); }
    if let Some(project_id) = input.project_id { task.project_id = Some(project_id); }
    if let Some(tags) = input.tags { task.tags = tags; }
    task.updated_at = Utc::now().to_rfc3339();

    Ok(task.clone())
}

#[tauri::command]
pub fn delete_task(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    tasks.retain(|t| t.id != id);
    Ok(())
}

// Project Commands
#[tauri::command]
pub fn load_projects(state: tauri::State<AppState>) -> Result<Vec<Project>, String> {
    let projects = state.projects.lock().map_err(|e| e.to_string())?;
    Ok(projects.clone())
}

#[tauri::command]
pub fn create_project(state: tauri::State<AppState>, input: CreateProjectInput) -> Result<Project, String> {
    let mut projects = state.projects.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let project = Project {
        id: Uuid::new_v4().to_string(),
        name: input.name,
        description: input.description,
        status: "active".to_string(),
        progress: 0.0,
        cover_image: input.cover_image,
        tags: input.tags.unwrap_or_default(),
        task_ids: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
        completed_at: None,
        archived_at: None,
    };
    projects.push(project.clone());
    Ok(project)
}

#[tauri::command]
pub fn update_project(state: tauri::State<AppState>, id: String, input: UpdateProjectInput) -> Result<Project, String> {
    let mut projects = state.projects.lock().map_err(|e| e.to_string())?;
    let project = projects.iter_mut().find(|p| p.id == id).ok_or("Project not found")?;

    if let Some(name) = input.name { project.name = name; }
    if let Some(desc) = input.description { project.description = Some(desc); }
    if let Some(status) = input.status {
        project.status = status.clone();
        if status == "completed" {
            project.completed_at = Some(Utc::now().to_rfc3339());
        }
    }
    if let Some(progress) = input.progress { project.progress = progress; }
    if let Some(tags) = input.tags { project.tags = tags; }
    project.updated_at = Utc::now().to_rfc3339();

    Ok(project.clone())
}

#[tauri::command]
pub fn delete_project(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut projects = state.projects.lock().map_err(|e| e.to_string())?;
    projects.retain(|p| p.id != id);
    Ok(())
}

// Archive Commands
#[tauri::command]
pub fn load_archived_tasks(state: tauri::State<AppState>) -> Result<Vec<Task>, String> {
    let tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    Ok(tasks.iter().filter(|t| t.status == "archived").cloned().collect())
}

#[tauri::command]
pub fn load_archived_projects(state: tauri::State<AppState>) -> Result<Vec<Project>, String> {
    let projects = state.projects.lock().map_err(|e| e.to_string())?;
    Ok(projects.iter().filter(|p| p.status == "archived").cloned().collect())
}

#[tauri::command]
pub fn restore_task(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    let task = tasks.iter_mut().find(|t| t.id == id).ok_or("Task not found")?;
    task.status = "pending".to_string();
    task.archived_at = None;
    Ok(())
}

#[tauri::command]
pub fn restore_project(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut projects = state.projects.lock().map_err(|e| e.to_string())?;
    let project = projects.iter_mut().find(|p| p.id == id).ok_or("Project not found")?;
    project.status = "active".to_string();
    project.archived_at = None;
    Ok(())
}

#[tauri::command]
pub fn permanently_delete_task(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    tasks.retain(|t| t.id != id);
    Ok(())
}

#[tauri::command]
pub fn permanently_delete_project(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut projects = state.projects.lock().map_err(|e| e.to_string())?;
    projects.retain(|p| p.id != id);
    Ok(())
}

// Settings Commands (placeholder)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: String,
    pub language: String,
    pub notifications: Notifications,
    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    #[serde(rename = "taskReminders")]
    pub task_reminders: bool,
    #[serde(rename = "projectUpdates")]
    pub project_updates: bool,
    #[serde(rename = "dailyDigest")]
    pub daily_digest: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub title: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[tauri::command]
pub fn load_settings() -> Result<UserSettings, String> {
    Ok(UserSettings {
        theme: "system".to_string(),
        language: "zh-CN".to_string(),
        notifications: Notifications {
            task_reminders: true,
            project_updates: false,
            daily_digest: false,
        },
        profile: Profile {
            name: "Alex Rivera".to_string(),
            title: Some("策展人".to_string()),
            email: None,
            avatar: None,
        },
    })
}

#[tauri::command]
pub fn save_settings(_settings: UserSettings) -> Result<(), String> {
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        tasks: Mutex::new(Vec::new()),
        projects: Mutex::new(Vec::new()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            load_tasks,
            create_task,
            update_task,
            delete_task,
            load_projects,
            create_project,
            update_project,
            delete_project,
            load_archived_tasks,
            load_archived_projects,
            restore_task,
            restore_project,
            permanently_delete_task,
            permanently_delete_project,
            load_settings,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
