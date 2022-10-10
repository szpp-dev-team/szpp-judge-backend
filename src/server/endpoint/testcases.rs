use crate::{
    db::{repository::testcase::TestcaseRepository, PgPool},
    gcs::Client,
    server::model::testcases::FTestcase,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Form, Path},
    HttpResponse,
};
use diesel::Connection;
use tokio::runtime::Runtime;

#[post("/tasks/{task_id}/testcases")]
pub async fn handle_register_testcases(
    db_pool: Data<PgPool>,
    gcs_client: Data<Client>,
    data: Form<FTestcase>,
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
