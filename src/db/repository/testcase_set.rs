use crate::db::{
    model::testcase_sets::{NewTestcaseSet, TestcaseSet},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait TestcaseSetRepository {
    fn insert_testcase_set(&mut self, new_testcase_set: &NewTestcaseSet) -> Result<TestcaseSet>;
}

impl TestcaseSetRepository for PgPooledConn {
    fn insert_testcase_set(&mut self, new_testcase_set: &NewTestcaseSet) -> Result<TestcaseSet> {
        use crate::schema::testcase_sets;
        let res = insert_into(testcase_sets::table)
            .values(new_testcase_set)
            .get_result(self)?;
        Ok(res)
    }
}
