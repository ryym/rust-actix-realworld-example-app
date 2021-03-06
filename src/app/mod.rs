use actix_web::{
    http::{header, StatusCode},
    middleware::{cors::Cors, ErrorHandlers, Logger},
    App, HttpRequest,
};

use crate::config::Config;
use crate::hub::AppStore;

mod articles;
mod error;
mod profiles;
mod res;
mod tags;
mod users;

fn index(_req: &HttpRequest<AppStore>) -> &'static str {
    "Hello world!"
}

pub fn create(store: AppStore, conf: &Config) -> App<AppStore> {
    App::with_state(store)
        .middleware(Logger::default())
        .middleware(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error::not_found))
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {
            let scope = match conf.frontend_origin {
                Some(ref origin) => scope.middleware(enable_cors(origin)),
                None => scope,
            };

            // Users
            let scope = scope
                .resource("users", |r| r.post().with(users::sign_up))
                .resource("users/login", |r| r.post().with(users::sign_in))
                .resource("user", |r| {
                    r.get().with(users::get_current);
                    r.put().with(users::update)
                });

            // Profiles
            let scope = scope
                .resource("profiles/{username}", |r| r.get().with(profiles::get))
                .resource("profiles/{username}/follow", |r| {
                    r.post().with(profiles::follow);
                    r.delete().with(profiles::unfollow)
                });

            // Articles
            let scope = scope
                .resource("articles", |r| {
                    r.get().with(articles::list);
                    r.post().with(articles::create)
                }).resource("articles/feed", |r| r.get().with(articles::feed))
                .resource("articles/{slug}", |r| {
                    r.get().with(articles::get);
                    r.put().with(articles::update);
                    r.delete().with(articles::delete)
                }).resource("articles/{slug}/favorite", |r| {
                    r.post().with(articles::favorite);
                    r.delete().with(articles::unfavorite);
                }).resource("articles/{slug}/comments", |r| {
                    r.get().with(articles::comments::list);
                    r.post().with(articles::comments::add)
                }).resource("articles/{slug}/comments/{id}", |r| {
                    r.delete().with(articles::comments::delete)
                });

            // Tags
            let scope = scope.resource("tags", |r| r.get().with(tags::list));

            scope
        })
}

fn enable_cors(origin: &str) -> Cors {
    // Though the API document seems to recommend to construct the whole App using Cors::for_app(),
    // I don't understand why... The doc says 'you have to use Cors::for_app() method to
    // support preflight OPTIONS request', but it looks like we can handle preflight requests just by
    // app.middleware(cors), as far as I read the source code. And this works fine in fact.
    // ref: https://actix.rs/api/actix-web/stable/actix_web/middleware/cors/index.html
    Cors::build()
        .allowed_origin(origin)
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(3600)
        .finish()
}
