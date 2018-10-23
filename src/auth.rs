use actix_web::{http::header::AUTHORIZATION, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;

use db::HaveDb;
use error::JwtError;
use hub::Hub;
use jwt::{CanDecodeJwt, Decoded, Payload};
use mdl::User;
use prelude::*;

const TOKEN_PREFIX: &str = "Token ";

impl DecodeAuthToken for Hub {}
impl Authenticate for Hub {}

pub trait CanDecodeAuthToken {
    fn decode_auth_token<T: DeserializeOwned>(&self, token: &str) -> Result<T>;
}

pub trait DecodeAuthToken: CanDecodeJwt {}
impl<T: DecodeAuthToken> CanDecodeAuthToken for T {
    fn decode_auth_token<U: DeserializeOwned>(&self, token: &str) -> Result<U> {
        if !token.starts_with(TOKEN_PREFIX) {
            return Err(ErrorKind::Unauthorized.into());
        }
        let token = token.replacen(TOKEN_PREFIX, "", 1);

        match self.decode_jwt(&token)? {
            Decoded::Ok(payload) => Ok(payload),
            Decoded::Invalid(err) => Err(JwtError(err).context(ErrorKind::Unauthorized).into()),
        }
    }
}

pub trait CanAuthenticate {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth>;
}

pub trait Authenticate: CanDecodeAuthToken + HaveDb {}
impl<T: Authenticate> CanAuthenticate for T {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth> {
        let token = match req.headers().get(AUTHORIZATION) {
            None => return Err(ErrorKind::Unauthorized.into()),
            Some(token) => token
                .to_str()
                .context("read authorization header")?
                .to_owned(),
        };

        let payload = self.decode_auth_token::<Payload>(&token)?;

        let user = self.use_db(|conn| {
            use diesel::prelude::*;
            use schema::users::dsl::*;
            let user = users.find(payload.id).first(conn).optional()?;
            Ok(user)
        })?;

        match user {
            Some(user) => Ok(Auth { user, token }),
            None => Err(ErrorKind::Unauthorized.into()),
        }
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
