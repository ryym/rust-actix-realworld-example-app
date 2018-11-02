use crate::config::{Config, HaveConfig};
use crate::db::{self, Connection, HaveDb, Pool};
use crate::prelude::*;

pub struct Hub {
    config: Config,
    db_pool: Pool,
}

impl Hub {
    pub fn create(config: Config, db_pool: Pool) -> Hub {
        Hub { config, db_pool }
    }
}

impl HaveConfig for Hub {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl HaveDb for Hub {
    fn use_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = db::get_conn(&self.db_pool)?;
        f(&conn)
    }
}

impl db::HaveConn for Hub {
    fn conn(&self) -> Result<db::PooledConn> {
        let conn = self.db_pool.get().context("obtain DB connection")?;
        Ok(conn)
    }
}
