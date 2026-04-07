use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;
use tauri::Manager;

// ============================================================================
// Data Models
// ============================================================================

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

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
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
        }
    }
}

// ============================================================================
// Database State
// ============================================================================

pub struct DbState {
    pub conn: Mutex<Connection>,
}

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            priority TEXT NOT NULL DEFAULT 'medium',
            status TEXT NOT NULL DEFAULT 'pending',
            due_date TEXT,
            project_id TEXT,
            tags TEXT NOT NULL DEFAULT '[]',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            completed_at TEXT,
            archived_at TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            progress REAL NOT NULL DEFAULT 0.0,
            cover_image TEXT,
            tags TEXT NOT NULL DEFAULT '[]',
            task_ids TEXT NOT NULL DEFAULT '[]',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            completed_at TEXT,
            archived_at TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status)",
        [],
    )?;

    Ok(())
}

// ============================================================================
// Task Commands
// ============================================================================

#[tauri::command]
fn load_tasks(state: tauri::State<DbState>) -> Result<Vec<Task>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, priority, status, due_date, project_id, tags, created_at, updated_at, completed_at, archived_at FROM tasks WHERE status != 'archived'")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(8)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                status: row.get(4)?,
                due_date: row.get(5)?,
                project_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
fn create_task(state: tauri::State<DbState>, input: CreateTaskInput) -> Result<Task, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let tags = serde_json::to_string(&input.tags.unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());
    let priority = input.priority.unwrap_or_else(|| "medium".to_string());

    conn.execute(
        "INSERT INTO tasks (id, title, description, priority, status, due_date, project_id, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 'pending', ?5, ?6, ?7, ?8, ?8)",
        params![id, input.title, input.description, priority, input.due_date, input.project_id, tags, now],
    ).map_err(|e| e.to_string())?;

    Ok(Task {
        id,
        title: input.title,
        description: input.description,
        priority,
        status: "pending".to_string(),
        due_date: input.due_date,
        project_id: input.project_id,
        tags: input.tags.unwrap_or_default(),
        created_at: now.clone(),
        updated_at: now,
        completed_at: None,
        archived_at: None,
    })
}

#[tauri::command]
fn update_task(state: tauri::State<DbState>, id: String, input: UpdateTaskInput) -> Result<Task, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    // Fetch current
    let mut stmt = conn
        .prepare("SELECT id, title, description, priority, status, due_date, project_id, tags, created_at, updated_at, completed_at, archived_at FROM tasks WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let task = stmt
        .query_row(params![id], |row| {
            let tags_str: String = row.get(8)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                status: row.get(4)?,
                due_date: row.get(5)?,
                project_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut updated_task = task.clone();
    let mut completed_at = task.completed_at.clone();

    if let Some(title) = input.title { updated_task.title = title; }
    if let Some(desc) = input.description { updated_task.description = Some(desc); }
    if let Some(priority) = input.priority { updated_task.priority = priority; }
    if let Some(status) = input.status {
        updated_task.status = status.clone();
        if status == "completed" {
            completed_at = Some(now.clone());
        }
    }
    if let Some(due_date) = input.due_date { updated_task.due_date = Some(due_date); }
    if let Some(project_id) = input.project_id { updated_task.project_id = Some(project_id); }
    if let Some(tags) = input.tags { updated_task.tags = tags; }
    updated_task.updated_at = now;
    updated_task.completed_at = completed_at;

    let tags = serde_json::to_string(&updated_task.tags).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2, priority = ?3, status = ?4, due_date = ?5, project_id = ?6, tags = ?7, updated_at = ?8, completed_at = ?9 WHERE id = ?10",
        params![
            updated_task.title,
            updated_task.description,
            updated_task.priority,
            updated_task.status,
            updated_task.due_date,
            updated_task.project_id,
            tags,
            updated_task.updated_at,
            updated_task.completed_at,
            id
        ],
    ).map_err(|e| e.to_string())?;

    Ok(updated_task)
}

#[tauri::command]
fn delete_task(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// Project Commands
// ============================================================================

#[tauri::command]
fn load_projects(state: tauri::State<DbState>) -> Result<Vec<Project>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, description, status, progress, cover_image, tags, task_ids, created_at, updated_at, completed_at, archived_at FROM projects WHERE status != 'archived'")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(6)?;
            let task_ids_str: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let task_ids: Vec<String> = serde_json::from_str(&task_ids_str).unwrap_or_default();
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                progress: row.get(4)?,
                cover_image: row.get(5)?,
                tags,
                task_ids,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(projects)
}

#[tauri::command]
fn create_project(state: tauri::State<DbState>, input: CreateProjectInput) -> Result<Project, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let tags = serde_json::to_string(&input.tags.unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "INSERT INTO projects (id, name, description, status, progress, cover_image, tags, task_ids, created_at, updated_at) VALUES (?1, ?2, ?3, 'active', 0.0, ?4, ?5, '[]', ?6, ?6)",
        params![id, input.name, input.description, input.cover_image, tags, now],
    ).map_err(|e| e.to_string())?;

    Ok(Project {
        id,
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
    })
}

#[tauri::command]
fn update_project(state: tauri::State<DbState>, id: String, input: UpdateProjectInput) -> Result<Project, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    let mut stmt = conn
        .prepare("SELECT id, name, description, status, progress, cover_image, tags, task_ids, created_at, updated_at, completed_at, archived_at FROM projects WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let project = stmt
        .query_row(params![id], |row| {
            let tags_str: String = row.get(6)?;
            let task_ids_str: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let task_ids: Vec<String> = serde_json::from_str(&task_ids_str).unwrap_or_default();
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                progress: row.get(4)?,
                cover_image: row.get(5)?,
                tags,
                task_ids,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut updated_project = project.clone();
    let mut completed_at = project.completed_at.clone();

    if let Some(name) = input.name { updated_project.name = name; }
    if let Some(desc) = input.description { updated_project.description = Some(desc); }
    if let Some(status) = input.status {
        updated_project.status = status.clone();
        if status == "completed" {
            completed_at = Some(now.clone());
        }
    }
    if let Some(progress) = input.progress { updated_project.progress = progress; }
    if let Some(tags) = input.tags { updated_project.tags = tags; }
    updated_project.updated_at = now;
    updated_project.completed_at = completed_at;

    let tags = serde_json::to_string(&updated_project.tags).unwrap_or_else(|_| "[]".to_string());
    let task_ids = serde_json::to_string(&updated_project.task_ids).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "UPDATE projects SET name = ?1, description = ?2, status = ?3, progress = ?4, tags = ?5, task_ids = ?6, updated_at = ?7, completed_at = ?8 WHERE id = ?9",
        params![
            updated_project.name,
            updated_project.description,
            updated_project.status,
            updated_project.progress,
            tags,
            task_ids,
            updated_project.updated_at,
            updated_project.completed_at,
            id
        ],
    ).map_err(|e| e.to_string())?;

    Ok(updated_project)
}

#[tauri::command]
fn delete_project(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// Archive Commands
// ============================================================================

#[tauri::command]
fn load_archived_tasks(state: tauri::State<DbState>) -> Result<Vec<Task>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, priority, status, due_date, project_id, tags, created_at, updated_at, completed_at, archived_at FROM tasks WHERE status = 'archived'")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(8)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                status: row.get(4)?,
                due_date: row.get(5)?,
                project_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
fn load_archived_projects(state: tauri::State<DbState>) -> Result<Vec<Project>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, description, status, progress, cover_image, tags, task_ids, created_at, updated_at, completed_at, archived_at FROM projects WHERE status = 'archived'")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(6)?;
            let task_ids_str: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let task_ids: Vec<String> = serde_json::from_str(&task_ids_str).unwrap_or_default();
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                progress: row.get(4)?,
                cover_image: row.get(5)?,
                tags,
                task_ids,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                completed_at: row.get(10)?,
                archived_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(projects)
}

#[tauri::command]
fn restore_task(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET status = 'pending', archived_at = NULL, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn restore_project(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects SET status = 'active', archived_at = NULL, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn permanently_delete_task(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn permanently_delete_project(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// Settings Commands (JSON via tauri-plugin-store)
// ============================================================================

#[tauri::command]
async fn load_settings(app: tauri::AppHandle) -> Result<UserSettings, String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    match store.get("settings") {
        Some(value) => serde_json::from_value(value.clone()).map_err(|e| e.to_string()),
        None => Ok(UserSettings::default()),
    }
}

#[tauri::command]
async fn save_settings(app: tauri::AppHandle, settings: UserSettings) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("settings", serde_json::to_value(settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// App Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("someday");

    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

    let db_path = app_dir.join("someday.db");
    let conn = Connection::open(&db_path).expect("Failed to open database");
    init_db(&conn).expect("Failed to initialize database");

    let db_state = DbState {
        conn: Mutex::new(conn),
    };

    tauri::Builder::default()
        .manage(db_state)
        .plugin(tauri_plugin_store::Builder::default().build())
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
