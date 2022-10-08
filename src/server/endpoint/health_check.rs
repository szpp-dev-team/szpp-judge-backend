use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn handle_check_health() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("ok"))
}
