//! This is my hobby implementation of <https://github.com/gothinkster/realworld>.

// https://github.com/diesel-rs/diesel/issues/1785
// TODO: Remove this after diesel published 1.4.
#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate log;

extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate jsonwebtoken;
extern crate pbkdf2;
extern crate rand;
extern crate serde_json;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate r2d2;

#[cfg(test)]
#[macro_use]
mod test;

mod app;
mod auth;
mod config;
mod db;
mod error;
mod hub;
mod jwt;
mod mdl;
mod schema;

pub mod prelude {
    use super::error;
    use std::result;

    pub use crate::error::{Error, ErrorKind};
    pub use failure::{Fail, ResultExt};

    pub type Result<T, E = error::Error> = result::Result<T, E>;
}

use actix_web::server;
use std::env;

pub fn run() -> Result<(), error::Error> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    let jwt_secret_key = must_get_env_var("JWT_SECRET_KEY");
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let config = config::Config {
        jwt_secret_key,
        frontend_origin,
    };

    let db_url = must_get_env_var("DATABASE_URL");
    let db_pool = db::new_pool(db_url)?;

    let port = env::var("PORT").unwrap_or("3000".to_owned());
    log::info!("Starting server at 127.0.0.1:{}", port);

    server::new(move || {
        let store = hub::AppStore::create(config.clone(), db_pool.clone());
        app::create(store, &config)
    }).bind(format!("127.0.0.1:{}", port))
    .expect("start server")
    .run();

    Ok(())
}

fn must_get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}
