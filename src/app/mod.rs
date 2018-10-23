use actix_web::{middleware::Logger, App, HttpRequest};

use hub::Hub;

mod error;
mod users;

fn index(_req: &HttpRequest<Hub>) -> &'static str {
    "Hello world!"
}

pub fn create(hub: Hub) -> App<Hub> {
    let app = App::with_state(hub)
        .middleware(Logger::default())
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {
            scope
                .resource("users", |r| r.post().with(users::sign_up))
                .resource("users/login", |r| r.post().with(users::sign_in))
                .resource("user", |r| r.get().with(users::get_user))
        });
    app
}
