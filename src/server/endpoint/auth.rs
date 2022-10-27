use std::{ops::Add, sync::Arc};

use crate::{
    db::{repository::auth::AuthRepository, PgPool},
    server::{
        middleware::auth::{Claims, SECRET},
        model::{
            auth::{SigninPayload, SigninResponse},
            users::UserResponse,
        },
    },
    util::hash_password,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    post,
    web::{Data, Json},
    HttpResponse,
};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};

#[post("/auth/signin")]
pub async fn handle_signin(
    db_pool: Data<Arc<PgPool>>,
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
        exp: Local::now().add(Duration::hours(24 * 7)).timestamp() as usize,
        id: user.id,
        role: "general".to_string(),
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
