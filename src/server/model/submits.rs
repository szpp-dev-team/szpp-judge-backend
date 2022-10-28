use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::db::model::submit::{NewSubmit, Submit};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
            compile_message: None,
            created_at: chrono::Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    pub id: i32,
    pub status: String,
    pub score: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub language_id: String,

    pub user_id: i32,
    pub task_id: i32,
    pub contest_id: i32,

    pub source_code: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl SubmitResponse {
    pub fn from_model(submit: &Submit) -> Self {
        Self {
            id: submit.id,
            status: submit.status.clone(),
            score: submit.score,
            execution_time: submit.execution_time,
            execution_memory: submit.execution_memory,
            language_id: submit.language_id.clone(),

            user_id: submit.user_id,
            task_id: submit.task_id,
            contest_id: submit.contest_id,

            source_code: None,
            created_at: submit.created_at,
            updated_at: submit.updated_at,
        }
    }
}
