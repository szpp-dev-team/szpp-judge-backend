use crate::{
    db::{
        model::{
            testcase::Testcase,
            testcase_sets::{
                NewTestcaseSet, NewTestcaseTestcaseSet, TestcaseSet, TestcaseTestcaseSet,
            },
        },
        PgPooledConn,
    },
    schema::testcase_testcase_sets,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*, ExpressionMethods};

pub trait TestcaseSetRepository {
    fn insert_testcase_set(&mut self, new_testcase_set: &NewTestcaseSet) -> Result<TestcaseSet>;
    fn link_testcase_with_testcase_set(
        &mut self,
        testcase_set_id: i32,
        testcase_ids: &[i32],
    ) -> Result<Vec<(TestcaseTestcaseSet, TestcaseSet, Testcase)>>;
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
    ) -> Result<Vec<(TestcaseTestcaseSet, TestcaseSet, Testcase)>> {
        use crate::schema::{testcase_sets, testcase_testcase_sets::dsl, testcases};
        let list = testcase_ids
            .iter()
            .map(|&id2| NewTestcaseTestcaseSet {
                testcase_id: id2,
                testcase_set_id,
            })
            .collect::<Vec<_>>();
        let res = insert_into(testcase_testcase_sets::table)
            .values(&list)
            .get_results::<TestcaseTestcaseSet>(self)?;
        let testcase_set_ids = res.iter().map(|set| set.id).collect::<Vec<_>>();
        let res = dsl::testcase_testcase_sets
            .filter(dsl::id.eq_any(testcase_set_ids))
            .inner_join(testcase_sets::table)
            .inner_join(testcases::table)
            .get_results::<(TestcaseTestcaseSet, TestcaseSet, Testcase)>(self)?;
        Ok(res)
    }
}
