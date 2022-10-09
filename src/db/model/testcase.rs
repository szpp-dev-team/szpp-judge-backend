use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Testcase {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub task_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = testcases)]
pub struct NewTestcase {
    pub name: String,
    pub task_id: i32,
    pub created_at: NaiveDateTime,
}
