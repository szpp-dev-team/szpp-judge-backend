#[macro_use]
extern crate diesel;

use crate::server::endpoint::{health_check::handle_check_health, users::handle_get_user};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use db::new_pg_pool;
use dotenv::dotenv;
use server::endpoint::users::handle_register_user;
use std::{env, sync::Arc};

mod db;
mod schema;
mod server;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_pool = Arc::new(new_pg_pool(env::var("DATABASE_URL")?.as_str())?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .service(handle_register_user)
            .service(handle_get_user)
            .service(handle_check_health)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await?;

    Ok(())
}
