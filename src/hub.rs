use config::{Config, HaveConfig};
use db::{self, Connection, HaveDb, Pool};

use prelude::*;

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
