use crate::config::{Config, HaveConfig};
use crate::db;
use crate::prelude::*;

pub trait Store<S> {
    fn hub(&self) -> Result<S>;
}

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

// XXX: Temporary.
impl Store<Hub> for Hub {
    fn hub(&self) -> Result<Hub> {
        unimplemented!()
    }
}

// XXX: Temporary.
impl db::HaveConn for Hub {
    fn conn(&self) -> &db::Conn {
        unimplemented!()
    }
}
