use crate::db::{
    model::user::{NewUser, User},
    PgPool,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait UserRepository {
    fn fetch_user_by_id(&self, id: i32) -> Result<User>;
    fn fetch_user_by_session(&self, session: &str) -> Result<User>;
    fn insert_user(&self, new_user: &NewUser) -> Result<User>;
}

impl UserRepository for PgPool {
    fn fetch_user_by_id(&self, target_id: i32) -> Result<User> {
        use crate::schema::users::dsl::*;
        let mut conn = self.get()?;
        let res = users.filter(id.eq(target_id)).first(&mut conn)?;
        Ok(res)
    }

    fn fetch_user_by_session(&self, session_str: &str) -> Result<User> {
        use crate::schema::users::dsl::*;
        let mut conn = self.get()?;
        let res = users
            .filter(session_token.eq(session_str))
            .first(&mut conn)?;
        Ok(res)
    }

    fn insert_user(&self, new_user: &NewUser) -> Result<User> {
        use crate::schema::users;
        let mut conn = self.get()?;
        let res = insert_into(users::table)
            .values(new_user)
            .get_result(&mut conn)?;
        Ok(res)
    }
}
