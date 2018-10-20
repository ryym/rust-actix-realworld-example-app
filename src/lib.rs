//! This is my hobby implementation of <https://github.com/gothinkster/realworld>.

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;

extern crate actix_web;

mod app;

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
