use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Code {
    pub id: i32,
    pub title: String,
    pub code_url: String,
    pub language: String,
    pub author_id: i32, // User の foreign key
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = codes)]
pub struct NewCode {
    pub title: String,
    pub code_url: String,
    pub language: String,
    pub author_id: i32, // User の foreign key
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub enum Language {
    C,
    Cpp,
    Java,
    Python,
    Csharp,
    Rust,
    Go,
}
