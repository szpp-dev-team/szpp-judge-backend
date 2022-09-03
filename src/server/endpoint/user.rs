use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Form},
    HttpResponse,
};
use chrono::Local;
use serde::Deserialize;

use crate::db::{model::user::NewUser, repository::user::UserRepository, PgPool};

#[derive(Deserialize)]
pub struct FUser {
    pub name: String,
}

impl FUser {
    fn to_model(&self) -> NewUser {
        NewUser {
            name: self.name.to_string(),
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}

#[post("/users")]
pub async fn handle_post_user(
    db_pool: Data<PgPool>,
    data: Form<FUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = data.to_model();
    let user = db_pool
        .insert_user(&new_user)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&user))
}
