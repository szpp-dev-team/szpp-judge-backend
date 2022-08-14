use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use endpoint::code::handle_get_codes;
use repository::new_pg_pool;
use std::env;

#[macro_use]
extern crate diesel;

mod endpoint;
mod model;
mod repository;
mod schema;

use anyhow::Result;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_pool = new_pg_pool(env::var("DATABASE_URL")?.as_str())?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(handle_get_codes)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await?;

    Ok(())
}
