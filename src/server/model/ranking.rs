use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfo {
    pub task_id: i32,
    pub score: i32,
    pub duration: i32,
    pub penarty_count: i32,
    pub submit_ids: Vec<i32>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankInfo {
    pub rank: i32,
    pub user_id: i32,
    pub username: String,
    pub score: i32,
    pub duration: i32, // ペナルティ込み
    pub penarty_count: i32,
    pub task_info_list: Vec<TaskInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingResponse {
    pub rank_info_list: Vec<RankInfo>,
}

impl RankingResponse {
    pub fn from_model(new_rank_list: Vec<RankInfo>) -> Self {
        Self {
            rank_info_list: new_rank_list,
        }
    }
}
