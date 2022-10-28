use crate::db::model::contest::{Contest, NewContest};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestPayload {
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub penalty: i32, // seconds
}

impl ContestPayload {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestResponse {
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
}

impl ContestResponse {
    pub fn from_model(contest: &Contest) -> Self {
        Self {
            id: contest.id,
            name: contest.name.clone(),
            slug: contest.slug.clone(),
            category: contest.category.clone(),
            description: contest.description.clone(),
            start_at: contest.start_at,
            end_at: contest.end_at,
            penalty: contest.penalty,
            created_at: contest.created_at,
            updated_at: contest.updated_at,
        }
    }
}
