use crate::db::{repository::user::UserRepository, PgPool};
use crate::server::model::users::User;
use actix_web::web::Json;
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Path},
    HttpResponse,
};
use diesel::result::Error as DieselError;

#[post("/users")]
pub async fn handle_register_user(
    db_pool: Data<PgPool>,
    data: Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = data.to_model();
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = conn
        .insert_user(&new_user)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&user))
}

#[get("/users/{user_id}")]
pub async fn handle_get_user(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = conn.fetch_user_by_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;
    Ok(HttpResponse::Ok().json(&user))
}
