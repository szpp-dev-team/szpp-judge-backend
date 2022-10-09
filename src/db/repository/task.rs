use crate::db::{
    model::task::{NewTask, Task},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};

pub trait TaskRepository {
    fn insert_task(&mut self, new_task: &NewTask) -> Result<Task>;
}

impl TaskRepository for PgPooledConn {
    fn insert_task(&mut self, new_task: &NewTask) -> Result<Task> {
        use crate::schema::tasks;
        let res = insert_into(tasks::table)
            .values(new_task)
            .get_result(self)?;
        Ok(res)
    }
}
