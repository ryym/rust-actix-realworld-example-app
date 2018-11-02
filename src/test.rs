//! This module provides utilities for unit tests.

use crate::db;
use crate::prelude::*;
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
