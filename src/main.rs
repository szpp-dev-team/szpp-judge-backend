#[macro_use]
extern crate diesel;

use actix_web::{web::Data, App, HttpServer};
use db::new_pg_pool;
use dotenv::dotenv;
use server::endpoint::{
    code::{handle_get_codes, handle_post_code},
    user::handle_post_user,
};
use std::env;

mod db;
mod schema;
mod server;

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
            .service(handle_post_code)
            .service(handle_post_user)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await?;

    Ok(())
}
