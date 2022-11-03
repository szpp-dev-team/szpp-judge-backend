use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable, Clone)]
pub struct TestcaseSet {
    pub id: i32,
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub task_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = testcase_sets)]
pub struct NewTestcaseSet {
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
    pub created_at: NaiveDateTime,
    pub task_id: i32,
}

// TODO: いい感じの名前にする
#[derive(Queryable, Identifiable)]
pub struct TestcaseTestcaseSet {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub testcase_id: i32,
    pub testcase_set_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = testcase_testcase_sets)]
pub struct NewTestcaseTestcaseSet {
    pub testcase_id: i32,
    pub testcase_set_id: i32,
}
