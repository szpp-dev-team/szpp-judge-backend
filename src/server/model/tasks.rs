use serde::Deserialize;

use crate::db::model::task::NewTask;

#[derive(Deserialize)]
pub struct FTask {
    pub name: String,
    pub statement: String,
    pub part_score: Option<String>,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
}

impl FTask {
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
        }
    }
}
