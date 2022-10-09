use crate::db::{model::user::NewUser, repository::user::UserRepository, PgPool};
use crate::util::hash_password;
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Form, Path},
    HttpResponse,
};
use chrono::Local;
use diesel::result::Error as DieselError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FUser {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
}

impl FUser {
    fn to_model(&self) -> NewUser {
        NewUser {
            username: self.username.clone(),
            encrypted_password: hash_password(&self.password),
            display_name: self.display_name.clone(),
            session_token: None,
            created_at: Local::now().naive_local(),
        }
    }
}

#[post("/users")]
pub async fn handle_register_user(
    db_pool: Data<PgPool>,
    data: Form<FUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = data.to_model();
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = conn
        .insert_user(&new_user)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&user))
}

#[get("/users/{user_id}")]
pub async fn handle_get_user(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = conn.fetch_user_by_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;
    Ok(HttpResponse::Ok().json(&user))
}
