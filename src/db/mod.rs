use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub mod model;
pub mod repository;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn new_pg_pool(db_url: &str) -> Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let db_pool = Pool::builder().max_size(4).build(manager)?;
    Ok(db_pool)
}
