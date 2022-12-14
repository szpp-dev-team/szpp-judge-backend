use crate::db::{
    model::{
        contest::{Contest, ContestTask},
        task::{NewTask, Task},
    },
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*, update};

pub trait TaskRepository {
    fn insert_task(&mut self, new_task: &NewTask) -> Result<Task>;
    fn update_task(&mut self, task_id: i32, new_task: &NewTask) -> Result<Task>;
    fn fetch_task(&mut self, task_id: i32) -> Result<Task>;
    fn fetch_contest_tasks(&mut self, contest_id: i32)
        -> Result<Vec<(ContestTask, Contest, Task)>>;
}

impl TaskRepository for PgPooledConn {
    fn insert_task(&mut self, new_task: &NewTask) -> Result<Task> {
        use crate::schema::tasks;
        let res = insert_into(tasks::table)
            .values(new_task)
            .get_result(self)?;
        Ok(res)
    }

    fn update_task(&mut self, task_id: i32, new_task: &NewTask) -> Result<Task> {
        use crate::schema::tasks;
        use crate::schema::tasks::dsl::*;
        let res = update(tasks::table)
            .filter(id.eq(task_id))
            .set((
                name.eq(&new_task.name),
                statement.eq(&new_task.statement),
                part_score.eq(&new_task.part_score),
                constraints.eq(&new_task.constraints),
                input.eq(&new_task.input),
                output.eq(&new_task.output),
                score.eq(&new_task.score),
                time_limit.eq(&new_task.time_limit),
                memory_limit.eq(&new_task.memory_limit),
                updated_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(self)?;
        Ok(res)
    }

    fn fetch_task(&mut self, task_id: i32) -> Result<Task> {
        use crate::schema::tasks;
        let res = tasks::table
            .filter(tasks::id.eq(task_id))
            .get_result(self)?;
        Ok(res)
    }

    fn fetch_contest_tasks(
        &mut self,
        contest_id2: i32,
    ) -> Result<Vec<(ContestTask, Contest, Task)>> {
        use crate::schema::{contest_tasks::dsl::*, contests, tasks};
        let res = contest_tasks
            .filter(contest_id.eq(contest_id2))
            .inner_join(contests::table)
            .inner_join(tasks::table)
            .load(self)?;
        Ok(res)
    }
}
