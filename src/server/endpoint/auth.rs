use crate::{
    db::{repository::auth::AuthRepository, PgPool},
    server::model::auth::SigninPayload,
    util::hash_password,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/auth/signin")]
pub async fn handle_signin(
    db_pool: Data<PgPool>,
    data: Json<SigninPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let password = &data.password;
    let encrypted_password = hash_password(password);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    match conn.fetch_user_by_credential(&data.username, &encrypted_password) {
        Ok(s) => Ok(HttpResponse::Ok().json(&s)),
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}
