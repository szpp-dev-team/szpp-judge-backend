use crate::db::{model::user::NewUser, repository::user::UserRepository, PgPool};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Form, Path},
    HttpResponse,
};
use chrono::Local;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
pub struct FUser {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
}

impl FUser {
    fn to_model(&self) -> NewUser {
        let mut hasher = Sha256::new();
        hasher.update(self.password.as_bytes());
        let encrypted_password = String::from_utf8(hasher.finalize().to_vec()).unwrap();

        NewUser {
            username: self.username.clone(),
            encrypted_password,
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
    let user = db_pool
        .insert_user(&new_user)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&user))
}

#[get("/users/{user_id}")]
pub async fn handle_get_user(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = db_pool.fetch_user_by_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;
    Ok(HttpResponse::Ok().json(&user))
}
