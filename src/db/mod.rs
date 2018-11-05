pub mod articles;
pub mod followers;
pub mod users;

use diesel::{
    self,
    r2d2::{self, ConnectionManager, PooledConnection},
    result::Error as DieselError,
};

use crate::prelude::*;

pub type Conn = diesel::pg::PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub fn new_pool<S: Into<String>>(db_url: S) -> Result<Pool> {
    let manager = ConnectionManager::<Conn>::new(db_url.into());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .context("build DB pool")?;
    Ok(pool)
}

pub fn get_conn(pool: &Pool) -> Result<PooledConn> {
    let conn = pool.get().context("obtain DB connection")?;
    Ok(conn)
}

pub trait HaveConn {
    fn conn(&self) -> &Conn;
}

/// Ignores diesel's `QueryBuilderError` silently. This error could occur when
/// you run an update with a changeset whose all fields are `None`.
/// In that case, this returns `Ok(None)`.
pub fn may_update<T>(result: Result<T, DieselError>) -> Result<Option<T>, DieselError> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(err) => match err {
            DieselError::QueryBuilderError(_) => Ok(None),
            err => Err(err),
        },
    }
}

/// Return `Some(new)` if `new` is not equal to `old`, otherwise `None`.
/// This is useful to set up diesel's `AsChangeset` struct to update
/// only the changed columns.
pub fn if_changed<T: PartialEq>(new: Option<T>, old: &T) -> Option<T> {
    new.and_then(|new| if new != *old { Some(new) } else { None })
}
