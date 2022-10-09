use crate::db::{
    model::testcase_sets::NewTestcaseSet, repository::testcase_set::TestcaseSetRepository, PgPool,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Form, Path},
    HttpResponse,
};
use chrono::{Local, NaiveDateTime};
use diesel::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FTestcaseSet {
    pub name: String,
    pub is_sample: bool,
    pub score: i32,
    pub created_at: NaiveDateTime,
}

impl FTestcaseSet {
    fn to_model(&self, task_id: i32) -> NewTestcaseSet {
        NewTestcaseSet {
            name: self.name.clone(),
            is_sample: self.is_sample,
            score: self.score,
            created_at: Local::now().naive_local(),
            task_id,
        }
    }
}

#[post("/tasks/{task_id}/testcase_sets")]
pub async fn handle_register_testcases(
    db_pool: Data<PgPool>,
    data: Form<FTestcaseSet>,
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
