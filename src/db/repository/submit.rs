use crate::{
    db::{
        model::contest::Contest,
        model::task::Task,
        model::user::User,
        model::{
            submit::{NewSubmit, Submit},
        },
        PgPooledConn,
    },
};
use anyhow::Result;
use diesel::{insert_into, prelude::*, update};

pub trait SubmitRepository {
    fn insert_submit(&mut self, new_submit: &NewSubmit) -> Result<Submit>;
    fn fetch_submits(&mut self) -> Result<Vec<Submit>>;
    fn fetch_submits_wj(&mut self) -> Result<Vec<Submit>>;
    fn fetch_submit_by_id(&mut self, id: i32) -> Result<Submit>;
    fn update_submit(&mut self, new_submit: &Submit) -> Result<Submit>;
    fn fetch_submit_by_userid(&mut self, userid: i32, contestid: i32) -> Result<Vec<Submit>>;
    fn fetch_submits_by_contest_id(
        &mut self,
        contest_id: i32,
    ) -> Result<Vec<(Submit, Contest, User, Task)>>;
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

    fn fetch_submits_wj(&mut self) -> Result<Vec<Submit>> {
        use crate::schema::submits::dsl::*;
        let res = submits.filter(status.eq("WJ")).load(self)?;
        Ok(res)
    }

    fn fetch_submit_by_userid(&mut self, user_id2: i32, contest_id2: i32) -> Result<Vec<Submit>> {
        use crate::schema::submits::dsl::*;
        let res = submits
            .filter(contest_id.eq(contest_id2))
            .filter(user_id.eq(user_id2))
            .load(self)?;
        Ok(res)
    }

    fn fetch_submit_by_id(&mut self, id2: i32) -> Result<Submit> {
        use crate::schema::submits::dsl::*;
        let res = submits.find(id2).first(self)?;
        Ok(res)
    }

    fn update_submit(&mut self, new_submit: &Submit) -> Result<Submit> {
        use crate::schema::submits;
        use crate::schema::submits::dsl::*;
        let res = update(submits::table)
            .filter(id.eq(new_submit.id))
            .set((
                status.eq(&new_submit.status),
                score.eq(new_submit.score),
                execution_time.eq(new_submit.execution_time),
                execution_memory.eq(new_submit.execution_memory),
                updated_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(self)?;
        Ok(res)
    }

    fn fetch_submits_by_contest_id(
        &mut self,
        search_contest_id: i32,
    ) -> Result<Vec<(Submit, Contest, User, Task)>> {
        use crate::schema::{contests, submits, submits::dsl::*, tasks, users};
        let res = submits
            .filter(submits::contest_id.eq(search_contest_id))
            .inner_join(contests::table)
            .inner_join(users::table)
            .inner_join(tasks::table)
            .load(self)?;
        Ok(res)
    }
}
