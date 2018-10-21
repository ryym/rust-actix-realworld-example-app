use diesel::{
    self,
    r2d2::{self, ConnectionManager, PooledConnection},
};

use prelude::*;

pub type Connection = diesel::pg::PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn new_pool<S: Into<String>>(db_url: S) -> Result<Pool> {
    let manager = ConnectionManager::<Connection>::new(db_url.into());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .context("build DB pool")?;
    Ok(pool)
}

pub fn get_conn(pool: &Pool) -> Result<PooledConnection<ConnectionManager<Connection>>> {
    let conn = pool.get().context("obtain DB connection")?;
    Ok(conn)
}

pub trait HaveDb {
    fn use_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>;
}
