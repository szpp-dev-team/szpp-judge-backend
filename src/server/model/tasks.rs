use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::db::model::task::{NewTask, Task};

#[derive(Deserialize)]
pub struct TaskPayload {
    pub name: String,
    pub statement: String,
    pub part_score: Option<String>,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub task_type: String,
    pub is_draft: bool,
    pub is_public: bool,
}

impl TaskPayload {
    pub fn to_model(&self, author_id: i32) -> NewTask {
        NewTask {
            name: self.name.clone(),
            statement: self.statement.clone(),
            part_score: self.part_score.clone(),
            constraints: self.constraints.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
            score: self.score,
            time_limit: self.time_limit,
            memory_limit: self.memory_limit,
            created_at: chrono::Local::now().naive_local(),
            author_id,
            contest_id: None,
            task_type: self.task_type.clone(),
            is_draft: self.is_draft,
            is_public: self.is_public,
        }
    }
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: i32,
    pub name: String,
    pub statement: String,
    pub part_score: Option<String>,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub task_type: String,
    pub is_draft: bool,
    pub is_public: bool,
    pub contest_id: Option<i32>,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TaskResponse {
    pub fn from_model(task: &Task) -> Self {
        Self {
            id: task.id,
            name: task.name.clone(),
            statement: task.statement.clone(),
            part_score: task.part_score.clone(),
            constraints: task.constraints.clone(),
            input: task.input.clone(),
            output: task.output.clone(),
            score: task.score,
            time_limit: task.time_limit,
            memory_limit: task.memory_limit,
            task_type: task.task_type.clone(),
            is_draft: task.is_draft,
            is_public: task.is_public,
            contest_id: task.contest_id,
            author_id: task.author_id,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}
