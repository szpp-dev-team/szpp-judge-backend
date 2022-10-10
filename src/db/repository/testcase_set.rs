use crate::db::{
    model::testcase_sets::{NewTestcaseSet, NewTestcaseTestcaseSet, TestcaseSet},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait TestcaseSetRepository {
    fn insert_testcase_set(&mut self, new_testcase_set: &NewTestcaseSet) -> Result<TestcaseSet>;
    fn link_testcase_with_testcase_set(
        &mut self,
        testcase_set_id: i32,
        testcase_ids: &[i32],
    ) -> Result<()>;
}

impl TestcaseSetRepository for PgPooledConn {
    fn insert_testcase_set(&mut self, new_testcase_set: &NewTestcaseSet) -> Result<TestcaseSet> {
        use crate::schema::testcase_sets;
        let res = insert_into(testcase_sets::table)
            .values(new_testcase_set)
            .get_result(self)?;
        Ok(res)
    }

    fn link_testcase_with_testcase_set(
        &mut self,
        testcase_set_id: i32,
        testcase_ids: &[i32],
    ) -> Result<()> {
        use crate::schema::testcase_testcase_sets;
        let list = testcase_ids
            .iter()
            .map(|&id| NewTestcaseTestcaseSet {
                testcase_id: id,
                testcase_set_id,
            })
            .collect::<Vec<_>>();
        insert_into(testcase_testcase_sets::table)
            .values(&list)
            .execute(self)?;
        Ok(())
    }
}
