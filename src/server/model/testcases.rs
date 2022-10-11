use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::db::model::{
    testcase::{NewTestcase, Testcase},
    testcase_sets::TestcaseSet,
};

#[derive(Deserialize)]
pub struct TestcasePayload {
    pub name: String,
    pub task_id: i32,
    pub input: String,
    pub output: String,
}

impl TestcasePayload {
    pub fn to_model(&self, task_id: i32) -> NewTestcase {
        NewTestcase {
            name: self.name.clone(),
            task_id,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Serialize)]
pub struct TestcaseBody {
    pub id: i32,
    pub name: String,
    pub task_id: i32,
    pub input: String,
    pub output: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TestcaseBody {
    pub fn from_model(testcase: &Testcase, input: Vec<u8>, output: Vec<u8>) -> Self {
        Self {
            id: testcase.id,
            name: testcase.name.clone(),
            task_id: testcase.task_id,
            input: String::from_utf8(input).unwrap(),
            output: String::from_utf8(output).unwrap(),
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct TestcaseInfo {
    pub id: i32,
    pub name: String,
    pub task_id: i32,
    pub testcase_set: Vec<TestcaseSet>,
}

impl TestcaseInfo {
    pub fn from_model(testcase: &Testcase, testcase_sets: &[TestcaseSet]) -> Self {
        Self {
            id: testcase.id,
            name: testcase.name.clone(),
            task_id: testcase.task_id,
            testcase_set: testcase_sets.to_vec(),
        }
    }
}
