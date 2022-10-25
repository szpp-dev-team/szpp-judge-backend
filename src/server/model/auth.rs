use serde::{Deserialize, Serialize};
use super::users::UserResponse;

#[derive(Deserialize)]
pub struct SigninPayload {
    pub username: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct SigninResponse {
    pub user: UserResponse,
    pub token: String,
}