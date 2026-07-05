//! # Tauri Todo 应用后端模块
//!
//! 这个模块提供了 Todo 应用的 Tauri 后端命令实现，
//! 包括增删改查等核心功能。

mod models;

use models::{NewTodo, Todo, UpdateTodo};
use std::sync::Mutex;

/// 简单的问候命令
///
/// # 参数
/// - `name`: 要问候的名字
///
/// # 返回值
/// 返回包含问候语的字符串
///
/// # 示例
/// ```
/// let greeting = greet("World");
/// assert_eq!(greeting, "Hello, World! from 瓦夏曼波");
/// ```
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! from 瓦夏曼波", name)
}

/// 应用全局状态结构体
///
/// 使用 `Mutex` 保护 `Vec<Todo>`，确保在多线程环境下的数据安全。
///
/// # 字段
/// - `todos`: 使用互斥锁保护的 Todo 列表
///
/// # 线程安全
/// 通过 `Mutex` 实现 `Send + Sync`，可以在多个 Tauri 命令间安全共享。
struct AppState {
    /// 受互斥锁保护的 Todo 列表
    todos: Mutex<Vec<Todo>>,
}

/// 添加新的 Todo 项
///
/// # 参数
/// - `state`: Tauri 应用状态（自动注入）
/// - `payload`: 新 Todo 的数据（标题）
///
/// # 返回值
/// - `Ok(Todo)`: 成功创建并添加的 Todo 项
/// - `Err(String)`: 失败时的错误信息
///
/// # 错误
/// - 标题为空字符串（或仅包含空格）时返回错误
/// - 无法获取锁时返回错误
///
/// # 示例
/// ```
/// let payload = NewTodo {
///     title: "学习 Rust".to_string()
/// };
/// let result = add_todo(state, payload);
/// assert!(result.is_ok());
/// ```
#[tauri::command]
fn add_todo(state: tauri::State<AppState>, payload: NewTodo) -> Result<Todo, String> {
    // 去掉标题首尾空格
    let trimmed = payload.title.trim();
    // 如果标题为空，返回错误
    if trimmed.is_empty() {
        return Err("标题不能为空".to_string());
    }
    // 创建新的 Todo
    let todo = Todo {
        id: format!("todo_{}", chrono::Utc::now().timestamp_millis()),
        title: trimmed.to_string(),
        done: false,
        created_at: chrono::Utc::now(),
    };
    // 将新的 Todo 添加到全局状态中
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    todos.push(todo.clone());
    Ok(todo)
}

/// 获取所有 Todo 列表
///
/// # 参数
/// - `state`: Tauri 应用状态（自动注入）
///
/// # 返回值
/// - `Ok(Vec<Todo>)`: 当前所有 Todo 项的副本
/// - `Err(String)`: 无法获取锁时的错误信息
///
/// # 注意
/// 返回的是 Todo 列表的克隆副本，不会影响原始数据。
#[tauri::command]
fn list_todos(state: tauri::State<AppState>) -> Result<Vec<Todo>, String> {
    let todos = state.todos.lock().map_err(|e| e.to_string())?;
    Ok(todos.clone())
}

/// 切换 Todo 的完成状态
///
/// 将指定 ID 的 Todo 的 `done` 字段取反（true ↔ false）。
///
/// # 参数
/// - `state`: Tauri 应用状态（自动注入）
/// - `id`: 要切换状态的 Todo ID
///
/// # 返回值
/// - `Ok(Todo)`: 更新后的 Todo 项
/// - `Err(String)`: 找不到指定 ID 或无法获取锁时的错误信息
///
/// # 错误
/// - 找不到对应 ID 的 Todo 时返回错误
///
/// # 示例
/// ```
/// // 添加一个 Todo
/// let todo = add_todo(state, NewTodo { title: "完成项目".to_string() })?;
/// // 切换状态
/// let updated = toggle_todo(state, todo.id)?;
/// assert_eq!(updated.done, true);
/// ```
#[tauri::command]
fn toggle_todo(state: tauri::State<AppState>, id: String) -> Result<Todo, String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("找不到 id = {id} 的 todo"))?;
    todo.done = !todo.done;
    Ok(todo.clone())
}

/// 删除指定的 Todo
///
/// # 参数
/// - `state`: Tauri 应用状态（自动注入）
/// - `id`: 要删除的 Todo ID
///
/// # 返回值
/// - `Ok(())`: 成功删除
/// - `Err(String)`: 找不到指定 ID 或无法获取锁时的错误信息
///
/// # 错误
/// - 找不到对应 ID 的 Todo 时返回错误
#[tauri::command]
fn delete_todo(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == before {
        return Err(format!("找不到 id = {id} 的 todo"));
    }
    Ok(())
}

/// 更新 Todo 的标题或完成状态
///
/// 可以同时或单独更新标题和完成状态。
///
/// # 参数
/// - `state`: Tauri 应用状态（自动注入）
/// - `id`: 要更新的 Todo ID
/// - `payload`: 更新数据（标题和/或完成状态）
///
/// # 返回值
/// - `Ok(Todo)`: 更新后的 Todo 项
/// - `Err(String)`: 更新失败时的错误信息
///
/// # 错误
/// - 找不到对应 ID 的 Todo
/// - 新标题为空字符串（或仅包含空格）
/// - 无法获取锁
///
/// # 示例
/// ```
/// let update = UpdateTodo {
///     title: Some("更新后的标题".to_string()),
///     done: Some(true),
/// };
/// let updated = update_todo(state, todo_id, update)?;
/// assert_eq!(updated.title, "更新后的标题");
/// assert_eq!(updated.done, true);
/// ```
#[tauri::command]
fn update_todo(
    state: tauri::State<AppState>,
    id: String,
    payload: UpdateTodo,
) -> Result<Todo, String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("找不到 id = {id} 的 todo"))?;
    if let Some(title) = payload.title {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err("标题不能为空".to_string());
        }
        todo.title = trimmed.to_string();
    }
    if let Some(done) = payload.done {
        todo.done = done;
    }
    Ok(todo.clone())
}

/// Tauri 应用的入口点
///
/// 配置并启动 Tauri 应用，包括：
/// - 插件初始化
/// - 状态管理
/// - 命令注册
///
/// # 配置
/// - 使用 `tauri_plugin_opener` 插件
/// - 初始化 `AppState` 作为全局状态
/// - 注册所有 Todo 相关的命令
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            todos: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            add_todo,
            list_todos,
            toggle_todo,
            delete_todo,
            update_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
