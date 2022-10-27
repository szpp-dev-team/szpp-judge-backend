use crate::{
    db::{repository::contest::ContestRepository, PgPool},
    server::model::contests::{ContestPayload, ContestResponse},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::result::Error as DieselError;
use diesel::Connection;

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
    let contest_resp = ContestResponse::from_model(&contest);
    Ok(HttpResponse::Ok().json(&contest_resp))
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
    let contest_resp = ContestResponse::from_model(&contest);
    Ok(HttpResponse::Ok().json(&contest_resp))
}

#[put("/contests/{contest_id}")]
pub async fn handle_update_contest(
    db_pool: Data<PgPool>,
    data: Json<ContestPayload>,
    contest_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_contest = data.to_model();
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let contest = conn
        .transaction(|conn| {
            let contest = conn.update_contest(*contest_id, &new_contest)?;
            Ok::<_, anyhow::Error>(contest)
        })
        .map_err(ErrorInternalServerError)?;

    let contest_resp = ContestResponse::from_model(&contest);
    Ok(HttpResponse::Ok().json(&contest_resp))
}
