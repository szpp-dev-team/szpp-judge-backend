use std::{collections::HashMap, sync::Arc};

use crate::{
    db::{repository::testcase::TestcaseRepository, PgPool},
    gcs::Client,
    server::model::testcases::{TestcaseInfoResponse, TestcasePayload, TestcaseResponse},
};
use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;
use tokio::runtime::Runtime;

#[get("/tasks/{task_id}/testcases/{testcase_id}")]
pub async fn handle_get_testcase(
    db_pool: Data<Arc<PgPool>>,
    gcs_client: Data<Arc<Client>>,
    params: Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let rt = Runtime::new().unwrap();
    let testcase_resp = conn
        .transaction(|conn| {
            let testcase = conn.fetch_testcase(params.1)?;
            let (input, output) = rt.block_on(async {
                let (input, output) = gcs_client
                    .download_testcase(params.0, &testcase.name)
                    .await?;
                Ok::<_, anyhow::Error>((input, output))
            })?;
            let testcase_resp = TestcaseResponse::from_model(&testcase, input, output);

            Ok::<_, anyhow::Error>(testcase_resp)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcase_resp))
}

#[get("/tasks/{task_id}/testcases")]
pub async fn handle_get_testcases(
    db_pool: Data<Arc<PgPool>>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let list = conn
        .fetch_testcases_by_task_id(*task_id)
        .map_err(ErrorInternalServerError)?;
    let mut mp: HashMap<_, Vec<_>> = HashMap::new();
    for (_, testcase, testcase_set) in list {
        mp.entry(testcase).or_default().push(testcase_set);
    }

    let testcases_resp = mp
        .iter()
        .map(|(testcase, _testcase_set)| TestcaseInfoResponse::from_model(testcase))
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(&testcases_resp))
}

#[post("/tasks/{task_id}/testcases")]
pub async fn handle_register_testcase(
    db_pool: Data<Arc<PgPool>>,
    gcs_client: Data<Arc<Client>>,
    data: Json<TestcasePayload>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_testcase = data.to_model(*task_id);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let testcase = conn
        .transaction(|conn| {
            let testcase = conn.insert_testcase(&new_testcase)?;
            Ok::<_, anyhow::Error>(testcase)
        })
        .map_err(ErrorInternalServerError)?;

    let _bytes = gcs_client
        .upload_testcase(*task_id, &testcase.name, &data.input, &data.output)
        .await
        .map_err(ErrorInternalServerError);

    let testcase_resp = TestcaseResponse::from_model(
        &testcase,
        data.input.bytes().collect(),
        data.output.bytes().collect(),
    );
    Ok(HttpResponse::Ok().json(&testcase_resp))
}
