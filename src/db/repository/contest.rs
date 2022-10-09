use crate::db::{
    model::contest::{Contest, NewContest},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait ContestRepository {
    fn fetch_contest_by_id(&mut self, id: i32) -> Result<Contest>;
    fn insert_contest(&mut self, new_contest: &NewContest) -> Result<Contest>;
}

impl ContestRepository for PgPooledConn {
    fn fetch_contest_by_id(&mut self, target_id: i32) -> Result<Contest> {
        use crate::schema::contests::dsl::*;
        let res = contests.filter(id.eq(target_id)).first(self)?;
        Ok(res)
    }

    fn insert_contest(&mut self, new_contest: &NewContest) -> Result<Contest> {
        use crate::schema::contests;
        let res = insert_into(contests::table)
            .values(new_contest)
            .get_result(self)?;
        Ok(res)
    }
}
