use crate::db::{
    model::testcase::{NewTestcase, Testcase},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait TestcaseRepository {
    fn insert_testcase(&mut self, new_testcase: &NewTestcase) -> Result<Testcase>;
    fn insert_testcases(&mut self, new_testcases: &[NewTestcase]) -> Result<Vec<Testcase>>;
}

impl TestcaseRepository for PgPooledConn {
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
}
