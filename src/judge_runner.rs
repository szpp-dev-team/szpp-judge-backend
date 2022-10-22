use crate::db::{repository::submit::SubmitRepository, PgPool};
use anyhow::Result;
use chrono::Local;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

pub struct JudgeRunner {
    client: Arc<Client>,
    max_thread_num: usize,
    queue: Arc<Mutex<VecDeque<JudgeRequest>>>,
    db_pool: Arc<PgPool>,
}

#[derive(Serialize)]
pub struct JudgeRequest {
    pub submit_id: i32,
    pub language_id: String,
    pub testcase_names: Vec<String>,
}

#[derive(Deserialize)]
pub struct JudgeResponse {
    pub status: String,
    pub execution_time: i32,
    pub execution_memory: i32,
    pub compile_message: Option<String>,
    pub error_message: Option<String>,
    pub testcase_results: Vec<TestcaseResult>,
}

#[derive(Deserialize)]
pub struct TestcaseResult {
    pub id: u64,
    pub status: String,
    pub execution_time: i32,
    pub execution_memory: i32,
}

impl JudgeRunner {
    pub fn new(
        queue: Arc<Mutex<VecDeque<JudgeRequest>>>,
        db_pool: Arc<PgPool>,
        max_thread_num: usize,
    ) -> Self {
        let client = Arc::new(Client::default());
        Self {
            client,
            max_thread_num,
            queue,
            db_pool,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut handles = Vec::with_capacity(self.max_thread_num);

        for _ in 0..self.max_thread_num {
            let queue = self.queue.clone();
            let client = self.client.clone();
            let db_pool = self.db_pool.clone();
            let handle = tokio::spawn(async move {
                loop {
                    sleep(Duration::from_secs(1)).await;
                    let judge_request = match queue.lock().await.pop_front() {
                        Some(judge_request) => judge_request,
                        None => {
                            println!("no judge request found");
                            continue;
                        }
                    };

                    let resp = client
                        .post("http://example.com")
                        .json(&judge_request)
                        .send()
                        .await?;
                    let judge_response = resp.json::<JudgeResponse>().await?;

                    let mut db_conn = db_pool.get()?;
                    let mut submit = db_conn.fetch_submit_by_id(judge_request.submit_id)?;
                    submit.status = judge_response.status;
                    submit.compile_message = judge_response.compile_message;
                    submit.execution_memory = Some(judge_response.execution_memory);
                    submit.execution_time = Some(judge_response.execution_time);
                    submit.updated_at = Some(Local::now().naive_local());
                    // TODO: スコア計算

                    db_conn.update_submit(&submit)?;
                }

                #[allow(unreachable_code)]
                Ok::<(), anyhow::Error>(())
            });
            handles.push(handle);
        }

        let spawn_res_list = join_all(handles).await;
        for spawn_res in spawn_res_list {
            spawn_res??;
        }

        unreachable!()
    }
}
