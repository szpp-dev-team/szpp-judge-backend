use chrono::NaiveDateTime;
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

#[derive(Deserialize)]
pub struct SubmitResponse {
    pub id: i32,
    pub status: String,
    pub score: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub language_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,

    pub user_id: i32,
    pub task_id: i32,
    pub contest_id: i32,

    pub source_code: Option<String>,
}
