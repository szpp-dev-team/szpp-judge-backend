#[macro_use]
extern crate diesel;

use crate::{
    gcs::Client,
    server::endpoint::{
        contests::handle_get_contest, health_check::handle_check_health,
        tasks::handle_register_task, testcase_sets::handle_register_testcase_sets,
        testcases::handle_register_testcases, users::handle_get_user,
    },
};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use db::new_pg_pool;
use dotenv::dotenv;
use server::endpoint::users::handle_register_user;
use std::{env, sync::Arc};

mod db;
mod gcs;
mod schema;
mod server;
mod util;

const PORT: u16 = 8080;
const NUM_CPUS: usize = 4;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_pool = Arc::new(new_pg_pool(env::var("DATABASE_URL")?.as_str())?);
    let cloud_storage_client = Arc::new(Client::new());

    // TODO: route 関数に分ける
    // see: https://github.com/kenkoooo/AtCoderProblems/blob/2d3e64869f23c0510797da34039953ae3d4f018b/atcoder-problems-backend/src/server/services.rs
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(cloud_storage_client.clone()))
            .service(handle_register_user)
            .service(handle_get_user)
            .service(handle_check_health)
            .service(handle_get_contest)
            .service(handle_register_testcases)
            .service(handle_register_testcase_sets)
            .service(handle_register_task)
    })
    .bind(("0.0.0.0", PORT))?
    .workers(NUM_CPUS)
    .run()
    .await?;

    Ok(())
}
