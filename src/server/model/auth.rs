use serde::Deserialize;

#[derive(Deserialize)]
pub struct SigninPayload {
    pub user_id: String,
    pub password: String,
}
