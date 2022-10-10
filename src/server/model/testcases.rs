use chrono::Local;
use serde::Deserialize;

use crate::db::model::testcase::NewTestcase;

#[derive(Deserialize)]
pub struct Testcase {
    pub name: String,
    pub task_id: i32,
    pub input: String,
    pub output: String,
}

impl Testcase {
    pub fn to_model(&self, task_id: i32) -> NewTestcase {
        NewTestcase {
            name: self.name.clone(),
            task_id,
            created_at: Local::now().naive_local(),
        }
    }
}
