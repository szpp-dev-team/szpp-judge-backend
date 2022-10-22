use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    db::model::user::{NewUser, User},
    util::hash_password,
};

#[derive(Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
}

impl UserPayload {
    pub fn to_model(&self) -> NewUser {
        NewUser {
            username: self.username.clone(),
            encrypted_password: hash_password(&self.password),
            display_name: self.display_name.clone(),
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
    pub display_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl UserResponse {
    pub fn from_model(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            encrypted_password: user.encrypted_password.clone(),
            display_name: user.display_name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
