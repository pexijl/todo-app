use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 完整的 Todo 项,后端返回给前端用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
}

/// 新建 Todo 时的入参,前端 -> 后端
#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
}

/// 更新 Todo 时的入参(标题 / 完成状态)
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub done: Option<bool>,
}
