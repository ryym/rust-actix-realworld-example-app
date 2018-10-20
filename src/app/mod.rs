use actix_web::{middleware::Logger, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn create() -> App {
    let app = App::new()
        .middleware(Logger::default())
        .resource("/", |r| r.f(index));
    app
}
