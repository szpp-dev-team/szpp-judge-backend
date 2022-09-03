use crate::db::{model::code::NewCode, repository::code::CodeRepository, PgPool};
use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Form},
    HttpResponse,
};
use chrono::Local;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use std::{fs::File, io::Write};

#[derive(Deserialize)]
pub struct FCode {
    pub title: String,
    pub code: String,
    pub language: String,
    pub author_id: i32, // User の foreign key
}

impl FCode {
    fn to_model(&self, code_url: &str) -> NewCode {
        NewCode {
            title: self.title.clone(),
            code_url: code_url.to_string(),
            language: self.language.clone(),
            author_id: self.author_id,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}

#[get("/codes")]
pub async fn handle_get_codes(db_pool: Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let codes = db_pool
        .fetch_all_codes()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(codes))
}

#[post("/codes")]
pub async fn handle_post_code(
    db_pool: Data<PgPool>,
    data: Form<FCode>,
) -> Result<HttpResponse, actix_web::Error> {
    let filename = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();
    let filepath = format!("./codes/{}", &filename);
    let mut f = File::create(&filepath).map_err(ErrorInternalServerError)?;
    write!(&mut f, "{}", &data.code).map_err(ErrorInternalServerError)?;

    let new_code = data.to_model(&filepath);
    let code = db_pool
        .insert_code(&new_code)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&code))
}
