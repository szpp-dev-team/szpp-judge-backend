use crate::db::{
    model::submit::{NewSubmit, Submit},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait SubmitRepository {
    fn insert_submit(&mut self, new_submit: &NewSubmit) -> Result<Submit>;
    fn fetch_submits(&mut self) -> Result<Vec<Submit>>;
}

impl SubmitRepository for PgPooledConn {
    fn insert_submit(&mut self, new_submit: &NewSubmit) -> Result<Submit> {
        use crate::schema::submits;
        let res = insert_into(submits::table)
            .values(new_submit)
            .get_result(self)?;
        Ok(res)
    }

    fn fetch_submits(&mut self) -> Result<Vec<Submit>> {
        use crate::schema::submits::dsl::*;
        let res = submits.load(self)?;
        Ok(res)
    }
}
