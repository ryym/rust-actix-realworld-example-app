use crate::config::{Config, HaveConfig};
use crate::db;
use crate::prelude::*;
use std::rc::Rc;

pub trait Store {
    type Svc;
    fn service(&self) -> Result<Self::Svc>;
}

pub struct AppStore {
    config: Rc<Config>,
    db_pool: db::Pool,
}

impl AppStore {
    pub fn create(config: Config, db_pool: db::Pool) -> AppStore {
        AppStore {
            config: Rc::new(config),
            db_pool,
        }
    }
}

impl Store for AppStore {
    type Svc = Hub;
    fn service(&self) -> Result<Self::Svc> {
        let conn = db::get_conn(&self.db_pool)?;
        Ok(Hub {
            config: Rc::clone(&self.config),
            conn,
        })
    }
}

pub struct Hub {
    config: Rc<Config>,
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

macro_rules! register_service {
    ($trait:ident) => {
        impl $trait for crate::hub::Hub {}
    };
}
