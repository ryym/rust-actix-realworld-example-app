use crate::config::{Config, HaveConfig};
use crate::db;
use crate::prelude::*;

pub trait Store<S> {
    fn hub(&self) -> Result<S>;
}

pub struct AppStore {
    config: Config,
    db_pool: db::Pool,
}

impl AppStore {
    pub fn create(config: Config, db_pool: db::Pool) -> AppStore {
        AppStore { config, db_pool }
    }
}

impl Store<Hub> for AppStore {
    fn hub(&self) -> Result<Hub> {
        let conn = db::get_conn(&self.db_pool)?;
        Ok(Hub {
            config: self.config.clone(),
            conn,
        })
    }
}

pub struct Hub {
    config: Config,
    conn: db::PooledConn,
}

impl HaveConfig for Hub {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl db::HaveConn for Hub {
    fn conn(&self) -> &db::Conn {
        &self.conn
    }
}
