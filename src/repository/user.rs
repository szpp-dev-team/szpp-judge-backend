use crate::model::{NewUser, User};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

use super::PgPool;

pub trait UserRepository {
    fn fetch_user_by_id(&self, id: i32) -> Result<User>;
    fn insert_user(&self, new_user: &NewUser) -> Result<User>;
}

impl UserRepository for PgPool {
    fn fetch_user_by_id(&self, target_id: i32) -> Result<User> {
        use crate::schema::users::dsl::*;
        let conn = self.get()?;
        let res = users.filter(id.eq(target_id)).first(&conn)?;
        Ok(res)
    }

    fn insert_user(&self, new_user: &NewUser) -> Result<User> {
        use crate::schema::users;
        let conn = self.get()?;
        let res = insert_into(users::table)
            .values(new_user)
            .get_result(&conn)?;
        Ok(res)
    }
}
