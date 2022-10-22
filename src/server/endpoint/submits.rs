use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use diesel::Connection;

use crate::{
    db::{repository::submit::SubmitRepository, PgPool},
    gcs::Client,
    server::{middleware::auth::Claims, model::submits::SubmitPayload},
};

#[post("/submits")]
pub async fn handle_submit(
    user: Claims,
    cloud_storage_client: Data<Client>,
    db_pool: Data<PgPool>,
    data: Json<SubmitPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_submit = data.to_model(user.id);
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let submit = conn
        .transaction(|conn| {
            let submit = conn.insert_submit(&new_submit)?;
            Ok::<_, anyhow::Error>(submit)
        })
        .map_err(ErrorInternalServerError)?;
    cloud_storage_client
        .upload_submit_source(submit.id, &data.source_code)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&submit))
}

#[get("/submits")]
pub async fn handle_get_submits(
    _user: Claims,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let submit = conn.fetch_submits().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&submit))
}
