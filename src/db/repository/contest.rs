use crate::db::{
    model::contest::{Contest, NewContest},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*, update};

pub trait ContestRepository {
    fn fetch_contest_by_id(&mut self, id: i32) -> Result<Contest>;
    fn insert_contest(&mut self, new_contest: &NewContest) -> Result<Contest>;
    fn update_contest(&mut self, contest_id: i32, newcontest: &NewContest) -> Result<Contest>;
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

    fn update_contest(&mut self, contest_id: i32, new_contest: &NewContest) -> Result<Contest> {
        use crate::schema::contests;
        use crate::schema::contests::dsl::*;
        let res = update(contests::table)
            .filter(id.eq(contest_id))
            .set((
                name.eq(&new_contest.name),
                slug.eq(&new_contest.slug),
                category.eq(&new_contest.category),
                description.eq(&new_contest.description),
                start_at.eq(&new_contest.start_at),
                end_at.eq(&new_contest.end_at),
                penalty.eq(&new_contest.penalty),
                created_at.eq(&new_contest.created_at),
                updated_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(self)?;
        Ok(res)
    }
}
