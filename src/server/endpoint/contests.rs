use crate::db::{model::contest::NewContest, repository::contest::ContestRepository, PgPool};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::Path,
    web::{Data, Form},
    HttpResponse,
};
use chrono::{NaiveDateTime, Utc};
use diesel::result::Error as DieselError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FContest {
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub penalty: i32, // seconds
}

impl FContest {
    fn to_model(&self) -> NewContest {
        NewContest {
            name: self.name.clone(),
            slug: self.slug.clone(),
            category: self.category.clone(),
            description: self.description.clone(),
            start_at: self.start_time,
            end_at: self.end_time,
            penalty: self.penalty,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[post("/contests")]
pub async fn handle_register_contest(
    db_pool: Data<PgPool>,
    data: Form<FContest>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_contest = data.to_model();
    let contest = db_pool
        .insert_contest(&new_contest)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(&contest))
}

#[get("/contests/{contest_id}")]
pub async fn handle_get_contest(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let contest = db_pool.fetch_contest_by_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;
    Ok(HttpResponse::Ok().json(&contest))
}
