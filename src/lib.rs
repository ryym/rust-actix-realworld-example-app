//! This is my hobby implementation of <https://github.com/gothinkster/realworld>.

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate failure;

extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate frank_jwt;

mod app;
mod error;

mod prelude {
    use super::error;
    use std::result;

    pub use error::{Error, ErrorKind};
    pub use failure::Fail;

    pub type Result<T, E = error::Error> = result::Result<T, E>;
}

use actix_web::server;
use std::env;

pub fn run() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    let port = env::var("PORT").unwrap_or("3000".to_owned());
    info!("Starting server at 127.0.0.1:{}", port);

    server::new(|| app::create())
        .bind(format!("127.0.0.1:{}", port))
        .expect("start server")
        .run();
}
