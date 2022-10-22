use serde::Deserialize;

#[derive(Deserialize)]
pub struct SigninPayload {
    pub username: String,
    pub password: String,
}
