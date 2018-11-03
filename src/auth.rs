use actix_web::{http::header::AUTHORIZATION, FromRequest, HttpRequest};

use crate::db;
use crate::error::ErrorKindAuth;
use crate::hub::{Hub, Store};
use crate::jwt::{CanDecodeJwt, Decoded, Payload};
use crate::mdl::User;
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Token ";

add_hub_trait!(Authenticate);

pub trait CanAuthenticate {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth>;
}

pub trait Authenticate: db::HaveConn + CanDecodeJwt {}
impl<T: Authenticate> CanAuthenticate for T {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth> {
        let token = match req.headers().get(AUTHORIZATION) {
            None => return Err(ErrorKindAuth::NoAuthToken.into()),
            Some(token) => token
                .to_str()
                .context("read authorization header")?
                .to_owned(),
        };

        if !token.starts_with(TOKEN_PREFIX) {
            return Err(ErrorKindAuth::InvalidToken.into());
        }
        let token = token.replacen(TOKEN_PREFIX, "", 1);

        let payload: Payload = match self.decode_jwt(&token)? {
            Decoded::Ok(payload) => payload,
            Decoded::Invalid(err) => return Err(err.context(ErrorKindAuth::InvalidToken).into()),
        };

        let user = find_user(self.conn(), payload.id)?;
        Ok(Auth { user, token })
    }
}

fn find_user(conn: &db::Conn, id: i32) -> Result<User> {
    use crate::schema::users;
    use diesel::prelude::*;

    let user = users::table
        .find(id)
        .first(conn)
        .context(ErrorKindAuth::InvalidUser)?;
    Ok(user)
}

/// Extract a payload from JWT in the Authorization header and load a user.
/// You can use this for handlers that require authentication.
/// If the authentication is optinal, use `Option<Auth>` instead of `Auth`.
#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

// XXX: We cannot use trait bound for the content of Store.
// Should we use associated type instead...?
// https://users.rust-lang.org/t/need-help-with-unconstrained-type-parameter/13173
impl<S: Store<Hub>> FromRequest<S> for Auth {
    type Config = ();
    type Result = Result<Self>;

    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        req.state().hub()?.authenticate(req)
    }
}
