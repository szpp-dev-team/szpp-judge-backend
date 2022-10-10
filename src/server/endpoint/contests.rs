use crate::{
    db::{repository::contest::ContestRepository, PgPool},
    server::model::contests::ContestPayload,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::Path,
    web::{Data, Json},
    HttpResponse,
};
use diesel::result::Error as DieselError;

#[post("/contests")]
pub async fn handle_register_contest(
    db_pool: Data<PgPool>,
    data: Json<ContestPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_contest = data.to_model();
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let contest = conn
        .insert_contest(&new_contest)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(&contest))
}

#[get("/contests/{contest_id}")]
pub async fn handle_get_contest(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let contest = conn.fetch_contest_by_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;
    Ok(HttpResponse::Ok().json(&contest))
}
