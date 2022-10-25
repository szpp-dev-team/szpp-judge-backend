use crate::{
    db::{repository::auth::AuthRepository, PgPool},
    server::{
        middleware::auth::{Claims, SECRET},
        model::{auth::SigninPayload, users::UserResponse},
    },
    util::hash_password,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    post,
    web::{Data, Json},
    HttpResponse,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct SigninResponse {
    pub user: UserResponse,
    pub token: String,
}

#[post("/auth/signin")]
pub async fn handle_signin(
    db_pool: Data<PgPool>,
    data: Json<SigninPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let password = &data.password;
    let encrypted_password = hash_password(password);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = conn
        .fetch_user_by_credential(&data.username, &encrypted_password)
        .map_err(ErrorUnauthorized)?;
    let user = UserResponse::from_model(&user);
    let my_claims = Claims {
        exp: 10000000000,
        id: user.id,
        role: "aiueo".to_string(),
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap();

    let res = SigninResponse { user, token };

    Ok(HttpResponse::Ok().json(&res))
}
