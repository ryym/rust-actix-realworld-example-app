//! This module provides utilities for unit tests.

use crate::prelude::*;
use crate::{db, hub};
use diesel::Connection;

// You need to run `diesel setup` before running unit tests.
const DB_URL: &str = "postgres://ryu@localhost/conduit_test";

pub struct Test {
    pub database_url: String,
}

impl Test {
    /// Create a new connection for test.
    /// This connection's transaction will never be commited.
    pub fn db_conn(&self) -> Result<db::Conn> {
        let conn = db::Conn::establish(&self.database_url).context("establish connection")?;
        conn.begin_test_transaction()?;
        Ok(conn)
    }
}

pub fn init() -> Result<Test> {
    Ok(Test {
        database_url: DB_URL.to_owned(),
    })
}

pub struct Store<S>(pub S);

impl<S: Clone> hub::Store<S> for Store<S> {
    fn hub(&self) -> Result<S> {
        Ok(self.0.clone())
    }
}

#[derive(Clone)]
pub struct Mock {}

/// Implement db::HaveConn.
macro_rules! impl_have_conn {
    ($struct:ident($field:ident)) => {
        impl db::HaveConn for $struct {
            fn conn(&self) -> &db::Conn {
                &self.$field
            }
        }
    };
}
