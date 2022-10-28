use std::collections::HashMap;

use crate::{
    db::{repository::submit::SubmitRepository, PgPool},
    server::model::ranking::{RankInfo, RankingResponse, TaskInfo},
};

use diesel::result::Error as DieselError;

use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::Data,
    web::Path,
    HttpResponse,
};

#[get("/contests/{contest_id}/ranking")]
pub async fn handle_get_ranking(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let all_submit_info = conn.fetch_submits_by_contest_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;

    let mut users_task_penarty: HashMap<i32, HashMap<i32, (Vec<i32>, bool, i32)>> = HashMap::new(); // key:user.id value:(key:task.id, value:(Vec<submit.id>,ac_flag,penarty_cnt))
    let mut user_info: HashMap<i32, String> = HashMap::new(); // key:user.id, value:user.username
    let mut task_info: HashMap<i32, (i32, i32)> = HashMap::new(); // key:task.id, value:(task.score, contest.penarty)

    let correct_answer = "AC";

    for (submit, contest, user, task) in all_submit_info {
        user_info.entry(user.id).or_insert(user.username);
        task_info
            .entry(task.id)
            .or_insert((task.score, contest.penalty));

        if let Some(task_penarty) = users_task_penarty.get_mut(&user.id) {
            // exist
            let user_submits = task_penarty
                .entry(task.id)
                .or_insert((Vec::new(), false, 0));

            user_submits.0.push(submit.id);

            if submit.status == correct_answer {
                user_submits.1 = true;
            } else if !user_submits.1 {
                user_submits.2 += 1;
            }
        } else {
            // new user registration
            let mut new_user: HashMap<i32, (Vec<i32>, bool, i32)> = HashMap::new();
            let task_submit_ids = vec![submit.id];
            let mut ac_flag = false;
            let mut penarty_cnt = 0;

            if submit.status == correct_answer {
                ac_flag = true;
            } else {
                penarty_cnt += 1;
            }

            new_user.insert(task.id, (task_submit_ids, ac_flag, penarty_cnt));
            users_task_penarty.insert(user.id, new_user);
        }
    }

    let mut user_score_rank: HashMap<i32, i32> = HashMap::new(); // key: user_id value = score
    let mut rank_info_list: Vec<RankInfo> = Vec::new();

    for (user_id, user_submits) in users_task_penarty.iter() {
        let mut all_score = 0;
        let mut all_duration = 0;
        let mut all_penarty_cnt = 0;
        let mut user_progress: Vec<TaskInfo> = Vec::new();

        for (task_id, penarty_tuple) in user_submits.iter() {
            let mut each_task = TaskInfo {
                task_id: *task_id,
                score: task_info.get(&task_id).unwrap().0,
                duration: task_info.get(&task_id).unwrap().1 * penarty_tuple.2,
                penarty_count: penarty_tuple.2,
                submit_ids: penarty_tuple.0.clone(),
            };

            if !penarty_tuple.1 {
                each_task.score = 0;
                each_task.duration = 0;
            }

            all_score += each_task.score;
            all_duration += each_task.duration;
            all_penarty_cnt += each_task.penarty_count;
            user_progress.push(each_task);
        }

        let user_rank_info = RankInfo {
            rank: -1,
            user_id: *user_id,
            username: user_info.get(&user_id).unwrap().to_string(),
            score: all_score,
            duration: all_duration,
            penarty_count: all_penarty_cnt,
            task_info_list: user_progress,
        };

        rank_info_list.push(user_rank_info);
        user_score_rank.entry(*user_id).or_insert(all_score);
    }

    let mut vec: Vec<(&i32, &i32)> = user_score_rank.iter().collect();
    vec.sort_by(|a, b| (-a.1).cmp(&(-b.1)));

    let mut user_ranking: HashMap<i32, i32> = HashMap::new(); // key:user_id, value = ranking
    let mut ranking = 1;

    for (user_id, score) in vec {
        user_ranking.entry(*user_id).or_insert(ranking);
        ranking += 1;
    }

    for rank_info in rank_info_list.iter_mut() {
        rank_info.rank = *user_ranking.get(&rank_info.user_id).unwrap();
    }

    let res = RankingResponse::from_model(rank_info_list);

    Ok(HttpResponse::Ok().json(&res))
}
