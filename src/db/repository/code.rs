use crate::db::{
    model::code::{Code, NewCode},
    PgPool,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait CodeRepository {
    fn fetch_all_codes(&self) -> Result<Vec<Code>>;
    fn insert_code(&self, new_code: &NewCode) -> Result<Code>;
}

impl CodeRepository for PgPool {
    fn fetch_all_codes(&self) -> Result<Vec<Code>> {
        use crate::schema::codes::dsl::*;
        let mut conn = self.get()?;
        let res = codes.load::<Code>(&mut conn)?;
        Ok(res)
    }

    fn insert_code(&self, new_code: &NewCode) -> Result<Code> {
        use crate::schema::codes;
        let mut conn = self.get()?;
        let res = insert_into(codes::table)
            .values(new_code)
            .get_result(&mut conn)?;
        Ok(res)
    }
}
