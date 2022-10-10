use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;

use crate::db::model::contest::NewContest;

#[derive(Deserialize)]
pub struct Contest {
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub penalty: i32, // seconds
}

impl Contest {
    pub fn to_model(&self) -> NewContest {
        NewContest {
            name: self.name.clone(),
            slug: self.slug.clone(),
            category: self.category.clone(),
            description: self.description.clone(),
            start_at: self.start_time,
            end_at: self.end_time,
            penalty: self.penalty,
            created_at: Utc::now().naive_utc(),
        }
    }
}
