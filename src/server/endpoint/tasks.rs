use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Form},
    HttpResponse,
};
use diesel::Connection;
use serde::Deserialize;

use crate::db::{model::task::NewTask, repository::task::TaskRepository, PgPool};

#[derive(Deserialize)]
pub struct FTask {
    pub name: String,
    pub statement: String,
    pub part_score: Option<String>,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
}

impl FTask {
    fn to_model(&self) -> NewTask {
        NewTask {
            name: self.name.clone(),
            statement: self.statement.clone(),
            part_score: self.part_score.clone(),
            constraints: self.constraints.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
            score: self.score,
            time_limit: self.time_limit,
            memory_limit: self.memory_limit,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[post("/tasks")]
pub async fn handle_register_task(
    db_pool: Data<PgPool>,
    data: Form<FTask>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_task = data.to_model();
    let mut conn = db_pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let task = conn
        .transaction(|conn| {
            let task = conn.insert_task(&new_task)?;
            Ok::<_, anyhow::Error>(task)
        })
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(&task))
}
