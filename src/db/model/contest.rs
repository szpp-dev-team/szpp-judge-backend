use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub penalty: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = contests)]
pub struct NewContest {
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub penalty: i32,
    pub created_at: NaiveDateTime,
}
