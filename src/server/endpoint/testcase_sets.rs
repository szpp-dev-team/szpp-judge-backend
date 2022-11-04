use std::sync::Arc;

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
    db_pool: Data<Arc<PgPool>>,
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
    db_pool: Data<Arc<PgPool>>,
    data: Json<Vec<i32>>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let (testcase_sets, testcases): (Vec<_>, Vec<_>) = conn
        .transaction(|conn| {
            let res = conn.link_testcase_with_testcase_set(path.1, &data)?;
            Ok::<_, anyhow::Error>(res)
        })
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|(_, testcase_set, testcase)| (testcase_set, testcase))
        .unzip();
    let testcase_ids = testcases
        .iter()
        .map(|testcase| testcase.id)
        .collect::<Vec<_>>();
    let resp = TestcaseSetResponse::from_model(&testcase_sets[0], Some(testcase_ids));
    Ok(HttpResponse::Ok().json(resp))
}
