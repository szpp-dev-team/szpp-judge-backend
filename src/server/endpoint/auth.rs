use crate::{db::PgPool, server::model::auth::SigninPayload};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/auth/signin")]
pub async fn handle_signin(
    db_pool: Data<PgPool>,
    data: Json<SigninPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    unimplemented!()
}
