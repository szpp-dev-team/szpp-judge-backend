use crate::db::{
    repository::{submit::SubmitRepository, testcase::TestcaseRepository},
    PgPool,
};
use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

pub type JudgeQueue = Mutex<VecDeque<JudgeRequest>>;

pub struct JudgeRunner {
    client: Arc<Client>,
    max_thread_num: usize,
    queue: Arc<Mutex<VecDeque<JudgeRequest>>>,
    db_pool: Arc<PgPool>,
    judge_server_url: String,
}

#[derive(Serialize, Debug)]
pub struct JudgeRequest {
    pub submit_id: i32,
    pub language_id: String,
    pub task_id: i32,
    pub testcases: Vec<Testcase>,
}

#[derive(Serialize, Debug)]
pub struct Testcase {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct JudgeResponse {
    pub status: String,
    pub execution_time: i32,
    pub execution_memory: i32,
    pub compile_message: Option<String>,
    pub error_message: Option<String>,
    pub testcase_results: Option<Vec<TestcaseResult>>,
}

#[derive(Deserialize, Debug)]
pub struct TestcaseResult {
    pub id: i32,
    pub status: String,
    pub execution_time: i32,
    pub execution_memory: i32,
}

impl JudgeRunner {
    pub fn new(
        queue: Arc<Mutex<VecDeque<JudgeRequest>>>,
        db_pool: Arc<PgPool>,
        max_thread_num: usize,
        judge_server_url: String,
    ) -> Self {
        let client = Arc::new(Client::default());
        Self {
            client,
            max_thread_num,
            queue,
            db_pool,
            judge_server_url,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut handles = Vec::with_capacity(self.max_thread_num);
        let mut db_conn = self.db_pool.get()?;
        let res = db_conn.fetch_submits_wj()?;
        for submit_wj in res {
            let testcases = db_conn.fetch_testcases_by_task_id(submit_wj.task_id)?;
            let mut testcases_name: Vec<String> = Vec::new();
            for testcase in testcases {
                testcases_name.push(testcase.1.name);
            }
            let j = JudgeRequest {
                submit_id: submit_wj.id,
                language_id: submit_wj.language_id,
                task_id: submit_wj.task_id,
                testcase_names: testcases_name,
            };
            self.queue.lock().await.push_back(j);
        }

        for _ in 0..self.max_thread_num {
            let queue = self.queue.clone();
            let client = self.client.clone();
            let db_pool = self.db_pool.clone();
            let judge_server_url = self.judge_server_url.clone();
            let handle = tokio::spawn(async move {
                loop {
                    sleep(Duration::from_secs(1)).await;
                    let judge_request = match queue.lock().await.pop_front() {
                        Some(judge_request) => judge_request,
                        None => {
                            // println!("no judge request found");
                            continue;
                        }
                    };

                    let resp = client
                        .post(&judge_server_url)
                        .json(&judge_request)
                        .send()
                        .await?;
                    let judge_response = resp.json::<JudgeResponse>().await?;

                    let mut db_conn = db_pool.get()?;
                    let res = db_conn.fetch_testcases_by_task_id(judge_request.task_id)?;

                    // testcase_id => (score, testcase_ids) な HashMap を作る
                    let mut testcase_set_mp = HashMap::new();
                    for (_, testcase, testcase_set) in &res {
                        let entry = testcase_set_mp
                            .entry(testcase_set.id)
                            .or_insert((testcase_set.score, Vec::new()));
                        entry.1.push(testcase.id);
                    }

                    let mut all_score = None;
                    if let Some(testcase_results) = judge_response.testcase_results {
                        // testcase_id が AC かどうかの HashSet を作る
                        let mut task_ac_set = HashSet::new();
                        for testcase_result in testcase_results {
                            if testcase_result.status == "AC" {
                                task_ac_set.insert(testcase_result.id);
                            }
                        }

                        // testcase_set に属する testcase 全てが AC だったら score を all_score に加算
                        let mut all_score2 = 0;
                        for (_set_id, (score, testcase_ids)) in testcase_set_mp {
                            if testcase_ids
                                .iter()
                                .all(|testcase_id| task_ac_set.contains(testcase_id))
                            {
                                all_score2 += score;
                            }
                        }

                        all_score = Some(all_score2);
                    }

                    let mut submit = db_conn.fetch_submit_by_id(judge_request.submit_id)?;
                    submit.update(
                        &judge_response.status,
                        judge_response.compile_message,
                        judge_response.execution_memory,
                        judge_response.execution_time,
                        all_score,
                    );

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
