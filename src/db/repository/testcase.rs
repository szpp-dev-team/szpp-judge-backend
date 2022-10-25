use crate::db::{
    model::{
        testcase::{NewTestcase, Testcase},
        testcase_sets::{TestcaseSet, TestcaseTestcaseSet},
    },
    PgPooledConn,
};
use anyhow::Result;
use chrono::Local;
use diesel::{insert_into, prelude::*, update, ExpressionMethods};

pub trait TestcaseRepository {
    fn fetch_testcase(&mut self, testcase_id: i32) -> Result<Testcase>;
    // TODO: join した結果の型を作る
    fn fetch_testcases_by_task_id(
        &mut self,
        task_id: i32,
    ) -> Result<Vec<(TestcaseTestcaseSet, Testcase, TestcaseSet)>>;
    fn insert_testcase(&mut self, new_testcase: &NewTestcase) -> Result<Testcase>;
    fn insert_testcases(&mut self, new_testcases: &[NewTestcase]) -> Result<Vec<Testcase>>;
    fn delete_testcases(&mut self, task_id: i32, testcase_ids: &[i32]) -> Result<Vec<Testcase>>;
}

impl TestcaseRepository for PgPooledConn {
    fn fetch_testcase(&mut self, testcase_id: i32) -> Result<Testcase> {
        use crate::schema::testcases;
        let res = testcases::table
            .filter(testcases::id.eq(testcase_id))
            .get_result(self)?;
        Ok(res)
    }

    fn fetch_testcases_by_task_id(
        &mut self,
        task_id: i32,
    ) -> Result<Vec<(TestcaseTestcaseSet, Testcase, TestcaseSet)>> {
        use crate::schema::{testcase_sets, testcase_testcase_sets::dsl::*, testcases};
        let res = testcase_testcase_sets
            .filter(testcases::task_id.eq(task_id))
            .inner_join(testcases::table)
            .inner_join(testcase_sets::table)
            .load(self)?;
        Ok(res)
    }

    fn insert_testcase(&mut self, new_testcase: &NewTestcase) -> Result<Testcase> {
        use crate::schema::testcases;
        let res = insert_into(testcases::table)
            .values(new_testcase)
            .get_result(self)?;
        Ok(res)
    }

    fn insert_testcases(&mut self, new_testcases: &[NewTestcase]) -> Result<Vec<Testcase>> {
        use crate::schema::testcases;
        let res = insert_into(testcases::table)
            .values(new_testcases)
            .get_results(self)?;
        Ok(res)
    }

    fn delete_testcases(&mut self, task_id2: i32, testcase_ids: &[i32]) -> Result<Vec<Testcase>> {
        use crate::schema::testcases;
        use crate::schema::testcases::dsl::*;

        let now = Local::now().naive_local();
        let res = update(testcases::table.filter(id.eq_any(testcase_ids)))
            .filter(task_id.eq(task_id2))
            .set(deleted_at.eq(now))
            .get_results(self)?;
        Ok(res)
    }
}
