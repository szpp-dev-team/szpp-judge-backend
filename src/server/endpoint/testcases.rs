use crate::{
    db::{repository::testcase::TestcaseRepository, PgPool},
    gcs::Client,
    server::model::testcases::{TestcaseBody, TestcasePayload},
};
use actix_web::{
    delete,
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;
use tokio::runtime::Runtime;

#[get("/tasks/{task_id}/testcases/{testcase_id}")]
pub async fn handle_get_testcase(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    task_id: Path<i32>,
    testcase_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let rt = Runtime::new().unwrap();
    let testcase_body = conn
        .transaction(|conn| {
            let testcase = conn.fetch_testcase(*testcase_id)?;
            let (input, output) = rt.block_on(async {
                let (input, output) = gcs_client
                    .download_testcase(*task_id, &testcase.name)
                    .await?;
                Ok::<_, anyhow::Error>((input, output))
            })?;
            let testcase_body = TestcaseBody::from_model(&testcase, input, output);

            Ok::<_, anyhow::Error>(testcase_body)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcase_body))
}

#[post("/tasks/{task_id}/testcases")]
pub async fn handle_register_testcase(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    data: Json<TestcasePayload>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_testcase = data.to_model(*task_id);
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let rt = Runtime::new().map_err(ErrorInternalServerError)?;
    let testcase = conn
        .transaction(|conn| {
            let testcase = conn.insert_testcase(&new_testcase)?;
            rt.block_on(async {
                let bytes = gcs_client
                    .upload_testcase(testcase.id, &testcase.name, &data.input, &data.output)
                    .await?;
                Ok::<_, anyhow::Error>(bytes)
            })?;
            Ok::<_, anyhow::Error>(testcase)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcase))
}

#[post("/tasks/{task_id}/testcases")]
pub async fn handle_register_testcases(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    data: Json<Vec<TestcasePayload>>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_testcases = data
        .iter()
        .map(|t| t.to_model(*task_id))
        .collect::<Vec<_>>();
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let rt = Runtime::new().map_err(ErrorInternalServerError)?;
    let testcases = conn
        .transaction(|conn| {
            let testcases = conn.insert_testcases(&new_testcases)?;
            rt.block_on(async {
                for (testcase, data) in testcases.iter().zip(data.iter()) {
                    gcs_client
                        .upload_testcase(testcase.id, &testcase.name, &data.input, &data.output)
                        .await?;
                }
                Ok::<_, anyhow::Error>(())
            })?;
            Ok::<_, anyhow::Error>(testcases)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcases))
}

#[delete("/tasks/{task_id}/testcases")]
pub async fn handle_delete_testcases(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    data: Json<Vec<i32>>,
    task_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let rt = Runtime::new().map_err(ErrorInternalServerError)?;
    let testcases = conn
        .transaction(|conn| {
            let testcases = conn.delete_testcases(*task_id, &data)?;
            rt.block_on(async {
                for testcase in &testcases {
                    gcs_client
                        .remove_testcase(testcase.task_id, &testcase.name)
                        .await?;
                }
                Ok::<_, anyhow::Error>(())
            })?;
            Ok::<_, anyhow::Error>(testcases)
        })
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&testcases))
}
