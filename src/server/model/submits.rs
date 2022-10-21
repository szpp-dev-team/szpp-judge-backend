use serde::Deserialize;

use crate::db::model::submit::NewSubmit;

#[derive(Deserialize)]
pub struct SubmitPayload {
    pub task_id: i32, // パスパラメータに含める？
    pub contest_id: i32,
    pub language_id: String,
    pub source_code: String,
}

impl SubmitPayload {
    pub fn to_model(&self, user_id: i32) -> NewSubmit {
        NewSubmit {
            user_id,
            task_id: self.task_id,
            language_id: self.language_id.clone(),
            contest_id: self.contest_id,
            status: "WJ".to_string(),
            score: None,
            execution_time: None,
            execution_memory: None,
            created_at: chrono::Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
