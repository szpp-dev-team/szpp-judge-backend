use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::db::model::testcase_sets::{NewTestcaseSet, TestcaseSet};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSetPayload {
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
}

impl TestcaseSetPayload {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSetResponse {
    pub id: i32,
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
    pub testcase_ids: Option<Vec<i32>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TestcaseSetResponse {
    pub fn from_model(testcase_set: &TestcaseSet, testcase_ids: Option<Vec<i32>>) -> Self {
        Self {
            id: testcase_set.id,
            name: testcase_set.name.clone(),
            is_sample: testcase_set.is_sample,
            score: testcase_set.score,
            testcase_ids,
            created_at: testcase_set.created_at,
            updated_at: testcase_set.updated_at,
        }
    }
}
