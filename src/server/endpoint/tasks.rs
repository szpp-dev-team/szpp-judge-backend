use crate::{
    db::{repository::task::TaskRepository, PgPool},
    server::{
        middleware::auth::Claims,
        model::tasks::{TaskPayload, TaskResponse},
    },
};
use actix_web::{
    error::ErrorInternalServerError,
    post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;

#[post("/tasks")]
pub async fn handle_register_task(
    user: Claims,
    db_pool: Data<PgPool>,
    data: Json<TaskPayload>,
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

    let task_resp = TaskResponse::from_model(&task);
    Ok(HttpResponse::Ok().json(&task_resp))
}

#[put("/tasks/{task_id}")]
pub async fn handle_update_task(
    user: Claims,
    db_pool: Data<PgPool>,
    data: Json<TaskPayload>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_task = data.to_model(user.id);
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let task = conn
        .transaction(|conn| {
            let task = conn.update_task(*task_id, &new_task)?;
            Ok::<_, anyhow::Error>(task)
        })
        .map_err(ErrorInternalServerError)?;

    let task_resp = TaskResponse::from_model(&task);
    Ok(HttpResponse::Ok().json(&task_resp))
}
