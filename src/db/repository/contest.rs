use crate::db::{
    model::contest::{Contest, NewContest},
    PgPool,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait ContestRepository {
    fn fetch_contest_by_id(&self, id: i32) -> Result<Contest>;
    fn insert_contest(&self, new_contest: &NewContest) -> Result<Contest>;
}

impl ContestRepository for PgPool {
    fn fetch_contest_by_id(&self, target_id: i32) -> Result<Contest> {
        use crate::schema::contests::dsl::*;
        let mut conn = self.get()?;
        let res = contests.filter(id.eq(target_id)).first(&mut conn)?;
        Ok(res)
    }

    fn insert_contest(&self, new_contest: &NewContest) -> Result<Contest> {
        use crate::schema::contests;
        let mut conn = self.get()?;
        let res = insert_into(contests::table)
            .values(new_contest)
            .get_result(&mut conn)?;
        Ok(res)
    }
}
