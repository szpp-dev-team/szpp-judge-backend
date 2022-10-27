use std::collections::HashMap;

use crate::{
    db::{
        repository::{submit::SubmitRepository},
        PgPool, model::task::Task,
    },
    gcs::Client,
};

use diesel::result::Error as DieselError;

use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post, put,
    web::Path,
    web::{Data, Json},
    HttpResponse,
};

#[get("/contests/{contest_id}/ranking")]
pub async fn handle_get_ranking(
    path: Path<i32>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user_info = conn.fetch_submits_by_contest_id(*path).map_err(|e| {
        if let DieselError::NotFound = e.downcast_ref::<DieselError>().unwrap() {
            ErrorNotFound(e)
        } else {
            ErrorInternalServerError(e)
        }
    })?;

    let mut submit_ids :Vec<i32>;
    let mut duration: i32;
    let mut all_score: i32;
    let mut all_penarty_count: i32;
    let mut penarty_count: i32;
    let mut ac_flags: HashMap<i32,String>::new();
    let mut users_task_sbumits: HashMap<i32,HashMap<i32,Vec<i32>>>::new();   // key:user_id value:(key:task_id,value:vec<submit_id>)

    for(submit,contest,user,task) in user_info {

        if let Some(user_task_info) = users_task_sbumits.get_mut(user.id){
            // not exist
            let mut submits = vec![submit.id];
            let mut task_submits: HashMap<i32,Vec<i32>>::new();  task_submits.insert(task.id,submits);
            users_task_sbumits.insert(task_submits);
        }
        else{
            let mut task_submits = users_task_sbumits.get(&task.id);
            users_task_sbumits.insert();
        }

        if ac_flags.get(submit.statement){
            duration += contest.penalty;
            penarty_count += 1;
        }

    }

    Ok(HttpResponse::Ok().json(&user_info))

}