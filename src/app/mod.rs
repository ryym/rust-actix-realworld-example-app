use actix_web::{middleware::Logger, App, HttpRequest};

mod auth;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn create() -> App {
    let app = App::new()
        .middleware(Logger::default())
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {
            scope.resource("users", |r| r.post().with(auth::sign_up))
        });
    app
}
