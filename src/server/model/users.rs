use chrono::Local;
use serde::Deserialize;

use crate::{db::model::user::NewUser, util::hash_password};

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
            session_token: None,
            created_at: Local::now().naive_local(),
        }
    }
}
