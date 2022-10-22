use crate::db::{model::user::User, PgPooledConn};
use anyhow::Result;
use diesel::prelude::*;

pub trait AuthRepository {
    fn fetch_user_by_credential(
        &mut self,
        username: &str,
        encrypted_password: &str,
    ) -> Result<User>;
}

impl AuthRepository for PgPooledConn {
    fn fetch_user_by_credential(
        &mut self,
        username: &str,
        encrypted_password: &str,
    ) -> Result<User> {
        use crate::schema::users::dsl;
        let user = dsl::users
            .filter(dsl::username.eq(username))
            .filter(dsl::encrypted_password.eq(encrypted_password))
            .first(self)?;
        Ok(user)
    }
}
