use crate::db::{
    model::user::{NewUser, User},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait UserRepository {
    fn fetch_user_by_id(&mut self, id: i32) -> Result<User>;
    fn fetch_user_by_session(&mut self, session: &str) -> Result<User>;
    fn insert_user(&mut self, new_user: &NewUser) -> Result<User>;
}

impl UserRepository for PgPooledConn {
    fn fetch_user_by_id(&mut self, target_id: i32) -> Result<User> {
        use crate::schema::users::dsl::*;
        let res = users.filter(id.eq(target_id)).first(self)?;
        Ok(res)
    }

    fn fetch_user_by_session(&mut self, session_str: &str) -> Result<User> {
        use crate::schema::users::dsl::*;
        let res = users.filter(session_token.eq(session_str)).first(self)?;
        Ok(res)
    }

    fn insert_user(&mut self, new_user: &NewUser) -> Result<User> {
        use crate::schema::users;
        let res = insert_into(users::table)
            .values(new_user)
            .get_result(self)?;
        Ok(res)
    }
}
