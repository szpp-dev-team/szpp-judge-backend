use crate::{
    db::{repository::testcase_set::TestcaseSetRepository, PgPool},
    server::model::testcase_sets::TestcaseSetPayload,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;

#[post("/tasks/{task_id}/testcase_sets")]
pub async fn handle_register_testcase_sets(
    db_pool: Data<PgPool>,
    data: Json<TestcaseSetPayload>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_testcase_set = data.to_model(*task_id);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let testcase = conn
        .transaction(|conn| {
            let testcase = conn.insert_testcase_set(&new_testcase_set)?;
            Ok::<_, anyhow::Error>(testcase)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcase))
}
