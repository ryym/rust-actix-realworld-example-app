use actix_web::{http::header::AUTHORIZATION, FromRequest, HttpRequest};

use crate::db::HaveDb;
use crate::error::ErrorKindAuth;
use crate::hub::Hub;
use crate::jwt::{CanDecodeJwt, Decoded, Payload};
use crate::mdl::User;
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Token ";

impl Authenticate for Hub {}

pub trait CanAuthenticate {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth>;
}

pub trait Authenticate: CanDecodeJwt + HaveDb {}
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

        let user = self.use_db(|conn| {
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;

            let user = users
                .find(payload.id)
                .first(conn)
                .context(ErrorKindAuth::InvalidUser)?;
            Ok(user)
        })?;

        Ok(Auth { user, token })
    }
}

/// Extract a payload from JWT in the Authorization header and load a user.
/// You can use this for handlers that require authentication.
/// If the authentication is optinal, use `Option<Auth>` instead of `Auth`.
#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

impl<S> FromRequest<S> for Auth
where
    S: CanAuthenticate,
{
    type Config = ();
    type Result = Result<Self>;

    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        req.state().authenticate(req)
    }
}
