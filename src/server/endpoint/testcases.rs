use crate::{
    db::{repository::testcase::TestcaseRepository, PgPool},
    gcs::Client,
    server::model::testcases::Testcase,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::Connection;
use tokio::runtime::Runtime;

#[post("/tasks/{task_id}/testcases")]
pub async fn handle_register_testcase(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    data: Json<Testcase>,
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
    data: Json<Vec<Testcase>>,
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
