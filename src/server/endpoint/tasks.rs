use crate::{
    db::{repository::task::TaskRepository, PgPool},
    server::{middleware::auth::Claims, model::tasks::Task},
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json},
    HttpResponse,
};
use diesel::Connection;

#[post("/tasks")]
pub async fn handle_register_task(
    user: Claims,
    db_pool: Data<PgPool>,
    data: Json<Task>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_task = data.to_model(user.id);
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let task = conn
        .transaction(|conn| {
            let task = conn.insert_task(&new_task)?;
            Ok::<_, anyhow::Error>(task)
        })
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(&task))
}
