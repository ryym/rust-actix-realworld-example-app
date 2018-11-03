use chrono::{Duration, Utc};
use jsonwebtoken::{self as jwt, errors as jwt_errors};
use serde::de::DeserializeOwned;

use crate::config::HaveConfig;
use crate::prelude::*;

add_hub_trait!(GenerateJwt);
add_hub_trait!(DecodeJwt);

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: i32,
    pub exp: i64,
}

pub trait CanGenerateJwt {
    fn generate_jwt(&self, user_id: i32) -> Result<String>;
}

pub trait GenerateJwt: HaveConfig {}
impl<T: GenerateJwt> CanGenerateJwt for T {
    fn generate_jwt(&self, user_id: i32) -> Result<String> {
        let exp = (Utc::now() + Duration::days(21)).timestamp();
        let payload = Payload { id: user_id, exp };

        // The default algorithm is HS256.
        let header = jwt::Header::default();
        let secret_key = &self.config().jwt_secret_key;
        let token = jwt::encode(&header, &payload, secret_key.as_ref()).context("generate JWT")?;

        Ok(token)
    }
}

pub enum Decoded<T> {
    Ok(T),
    Invalid(jwt_errors::Error),
}

pub trait CanDecodeJwt {
    fn decode_jwt<V: DeserializeOwned>(&self, token: &String) -> Result<Decoded<V>>;
}

pub trait DecodeJwt: HaveConfig {}
impl<T: DecodeJwt> CanDecodeJwt for T {
    fn decode_jwt<V: DeserializeOwned>(&self, token: &String) -> Result<Decoded<V>> {
        use jsonwebtoken::errors::ErrorKind as E;

        let secret_key = &self.config().jwt_secret_key;

        // It validates the algorithm and exp claims automatically.
        let validation = jwt::Validation::default();

        match jwt::decode(token, secret_key.as_ref(), &validation) {
            Ok(jwt::TokenData { claims, .. }) => Ok(Decoded::Ok(claims)),
            Err(err) => match err.kind() {
                E::Base64(_) | E::Json(_) | E::Utf8(_) => Err(err.context("decode JWT").into()),
                _ => Ok(Decoded::Invalid(err)),
            },
        }
    }
}
