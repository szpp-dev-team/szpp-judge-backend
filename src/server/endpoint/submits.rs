use std::sync::Arc;

use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::result::Error as DieselError;
use diesel::Connection;

use crate::{
    db::{
        repository::{submit::SubmitRepository, testcase::TestcaseRepository},
        PgPool,
    },
    gcs::Client,
    judge_runner::{self, JudgeQueue, JudgeRequest},
    server::{
        middleware::auth::Claims,
        model::submits::{SubmitPayload, SubmitResponse},
    },
};

#[post("/submits")]
pub async fn handle_submit(
    user: Claims,
    cloud_storage_client: Data<Arc<Client>>,
    db_pool: Data<Arc<PgPool>>,
    judge_queue: Data<Arc<JudgeQueue>>,
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

    let testcases = conn
        .fetch_testcases_by_task_id(submit.task_id)
        .map_err(ErrorInternalServerError)?;
    let judge_testcases = testcases
        .into_iter()
        .map(|testcase| judge_runner::Testcase {
            name: testcase.1.name,
            id: testcase.1.id,
        })
        .collect::<Vec<_>>();
    judge_queue.lock().await.push_back(JudgeRequest {
        submit_id: submit.id,
        language_id: submit.language_id.clone(),
        task_id: submit.task_id,
        testcases: judge_testcases,
    });

    let submit_resp = SubmitResponse::from_model(&submit);
    Ok(HttpResponse::Ok().json(&submit_resp))
}

#[get("/submits")]
pub async fn handle_get_submits(
    _user: Claims,
    db_pool: Data<Arc<PgPool>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let submits = conn.fetch_submits().map_err(ErrorInternalServerError)?;
    let submits_resp = submits
        .iter()
        .map(SubmitResponse::from_model)
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(&submits_resp))
}

#[get("/submits/{submit_id}")]
pub async fn handle_get_submit(
    _user: Claims,
    db_pool: Data<Arc<PgPool>>,
    submit_id: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;

    let submit = conn.fetch_submit_by_id(*submit_id).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;

    let submit_resp = SubmitResponse::from_model(&submit);
    Ok(HttpResponse::Ok().json(&submit_resp))
}
