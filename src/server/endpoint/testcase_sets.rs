use crate::{
    db::{repository::testcase_set::TestcaseSetRepository, PgPool},
    server::model::testcase_sets::{TestcaseSetPayload, TestcaseSetResponse},
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;

#[post("/tasks/{task_id}/testcase_sets")]
pub async fn handle_register_testcase_set(
    db_pool: Data<PgPool>,
    data: Json<TestcaseSetPayload>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_testcase_set = data.to_model(*task_id);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let testcase_set = conn
        .transaction(|conn| {
            let testcase = conn.insert_testcase_set(&new_testcase_set)?;
            Ok::<_, anyhow::Error>(testcase)
        })
        .map_err(ErrorInternalServerError)?;

    let testcase_set_resp = TestcaseSetResponse::from_model(&testcase_set, None);
    Ok(HttpResponse::Ok().json(&testcase_set_resp))
}

#[post("/tasks/{task_id}/testcase_sets/{testcase_set_id}")]
pub async fn handle_link_testcase_sets(
    db_pool: Data<PgPool>,
    data: Json<Vec<i32>>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    conn.transaction(|conn| {
        conn.link_testcase_with_testcase_set(path.1, &data)?;
        Ok::<_, anyhow::Error>(())
    })
    .map_err(ErrorInternalServerError)?;

    // TODO: 何かしら返す
    Ok(HttpResponse::Ok().finish())
}
