use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
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
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub contest_id: Option<i32>,
    pub author_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
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
    pub created_at: NaiveDateTime,
    pub contest_id: Option<i32>,
    pub author_id: i32,
}
