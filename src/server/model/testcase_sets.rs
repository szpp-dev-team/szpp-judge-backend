use chrono::{Local, NaiveDateTime};
use serde::Deserialize;

use crate::db::model::testcase_sets::NewTestcaseSet;

#[derive(Deserialize)]
pub struct TestcaseSet {
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
    pub created_at: NaiveDateTime,
}

impl TestcaseSet {
    pub fn to_model(&self, task_id: i32) -> NewTestcaseSet {
        NewTestcaseSet {
            name: self.name.clone(),
            is_sample: self.is_sample,
            score: self.score,
            created_at: Local::now().naive_local(),
            task_id,
        }
    }
}
