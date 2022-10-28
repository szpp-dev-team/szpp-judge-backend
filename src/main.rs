use crate::{
    judge_runner::JudgeRunner,
    server::endpoint::{
        auth::handle_signin,
        contests::{
            handle_get_contest, handle_get_contest_tasks, handle_get_submit_me,
            handle_register_contest, handle_update_contest,
        },
        health_check::handle_check_health,
        ranking::handle_get_ranking,
        submits::{handle_get_submit, handle_get_submits, handle_submit},
        tasks::{handle_register_task, handle_update_task},
        testcase_sets::{handle_link_testcase_sets, handle_register_testcase_set},
        testcases::handle_register_testcase,
        users::handle_get_user,
    },
};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use db::new_pg_pool;
use dotenv::dotenv;
use gcs::Client;
use server::endpoint::{
    tasks::handle_get_task,
    testcases::{handle_get_testcase, handle_get_testcases},
    users::handle_register_user,
};
use std::{collections::VecDeque, env, sync::Arc};
use tokio::sync::Mutex;

mod db;
mod gcs;
mod judge_runner;
mod schema;
mod server;
mod util;

const PORT: &str = "8080";
const NUM_CPUS: usize = 4;
const JUDGE_THREAD_NUM: usize = 5;
const QUEUE_CAPACITY: usize = 100_000;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_pool = Arc::new(new_pg_pool(env::var("DATABASE_URL")?.as_str())?);
    let cloud_storage_client = Arc::new(Client::new());
    let judge_queue = Arc::new(Mutex::new(VecDeque::with_capacity(QUEUE_CAPACITY)));
    let judge_runner = JudgeRunner::new(
        judge_queue.clone(),
        db_pool.clone(),
        JUDGE_THREAD_NUM,
        env::var("JUDGE_SERVER_URL")?,
    );
    tokio::spawn(async move {
        judge_runner.run().await?;
        Ok::<(), anyhow::Error>(())
    });

    // TODO: route 関数に分ける
    // see: https://github.com/kenkoooo/AtCoderProblems/blob/2d3e64869f23c0510797da34039953ae3d4f018b/atcoder-problems-backend/src/server/services.rs
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(cloud_storage_client.clone()))
            .app_data(Data::new(judge_queue.clone()))
            .service(handle_signin)
            .service(handle_register_contest)
            .service(handle_get_contest)
            .service(handle_get_contest_tasks)
            .service(handle_get_submit_me)
            .service(handle_update_contest)
            .service(handle_check_health)
            .service(handle_get_ranking)
            .service(handle_submit)
            .service(handle_get_submits)
            .service(handle_get_submit)
            .service(handle_register_task)
            .service(handle_update_task)
            .service(handle_get_task)
            .service(handle_register_testcase_set)
            .service(handle_link_testcase_sets)
            .service(handle_get_testcase)
            .service(handle_get_testcases)
            .service(handle_register_testcase)
            .service(handle_register_user)
            .service(handle_get_user)
    })
    .bind((
        "0.0.0.0",
        env::var("PORT")
            .unwrap_or_else(|_| PORT.to_string())
            .parse()?,
    ))?
    .workers(NUM_CPUS)
    .run()
    .await?;

    Ok(())
}
