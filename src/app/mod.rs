use actix_web::{middleware::Logger, App, HttpRequest};

use hub::Hub;

mod auth;
mod error;

fn index(_req: &HttpRequest<Hub>) -> &'static str {
    "Hello world!"
}

pub fn create(hub: Hub) -> App<Hub> {
    let app = App::with_state(hub)
        .middleware(Logger::default())
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {
            scope.resource("users", |r| r.post().with(auth::sign_up))
        });
    app
}
