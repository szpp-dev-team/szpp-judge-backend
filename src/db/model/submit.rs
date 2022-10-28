use crate::schema::*;
use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable};
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

impl Submit {
    pub fn update(
        &mut self,
        status: &str,
        compile_message: Option<String>,
        execution_memory: i32,
        execution_time: i32,
        score: i32,
    ) {
        self.status = status.to_string();
        self.compile_message = compile_message;
        self.execution_memory = Some(execution_memory);
        self.execution_time = Some(execution_time);
        self.score = Some(score);
        self.updated_at = Some(Local::now().naive_local());
    }
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
