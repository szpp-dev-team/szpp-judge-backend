use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Submit {
    pub id: i32,
    pub status: String,
    pub score: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_message: Option<String>,
    pub language_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,

    pub user_id: i32,
    pub task_id: i32,
    pub contest_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = submits)]
pub struct NewSubmit {
    pub status: String,
    pub score: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_message: Option<String>,
    pub language_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: i32,
    pub task_id: i32,
    pub contest_id: i32,
}
