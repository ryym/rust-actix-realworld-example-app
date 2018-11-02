use crate::config::{Config, HaveConfig};
use crate::db;
use crate::prelude::*;

pub struct Hub {
    config: Config,
    db_pool: db::Pool,
}

impl Hub {
    pub fn create(config: Config, db_pool: db::Pool) -> Hub {
        Hub { config, db_pool }
    }
}

impl HaveConfig for Hub {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl db::HaveDb for Hub {
    fn use_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&db::Conn) -> Result<T>,
    {
        let conn = db::get_conn(&self.db_pool)?;
        f(&conn)
    }
}
