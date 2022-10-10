use serde::Deserialize;

use crate::db::model::task::NewTask;

#[derive(Deserialize)]
pub struct Task {
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

impl Task {
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
