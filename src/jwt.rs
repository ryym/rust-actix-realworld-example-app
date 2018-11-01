use frank_jwt as jwt;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::config::HaveConfig;
use crate::hub::Hub;
use crate::prelude::*;

impl GenerateJwt for Hub {}
impl DecodeJwt for Hub {}

fn default_alg() -> jwt::Algorithm {
    jwt::Algorithm::HS256
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: i32,
}

pub trait CanGenerateJwt {
    fn generate_jwt(&self, user_id: i32) -> Result<String>;
}

pub trait GenerateJwt: HaveConfig {}
impl<T: GenerateJwt> CanGenerateJwt for T {
    fn generate_jwt(&self, user_id: i32) -> Result<String> {
        let secret_key = &self.config().jwt_secret_key;

        // frank_jwt sets the header values automatically.
        let header = json!({});
        // TODO: Add expiration.
        let payload = serde_json::to_value(Payload { id: user_id }).context("serialize payload")?;
        let jwt = jwt::encode(header, secret_key, &payload, default_alg())?;
        Ok(jwt)
    }
}

pub enum Decoded<T> {
    Ok(T),
    Invalid(jwt::Error),
}

pub trait CanDecodeJwt {
    fn decode_jwt<V: DeserializeOwned>(&self, token: &String) -> Result<Decoded<V>>;
}

pub trait DecodeJwt: HaveConfig {}
impl<T: DecodeJwt> CanDecodeJwt for T {
    fn decode_jwt<V: DeserializeOwned>(&self, token: &String) -> Result<Decoded<V>> {
        use frank_jwt::Error as E;

        let secret_key = &self.config().jwt_secret_key;

        match jwt::decode(token, secret_key, default_alg()) {
            Ok((_header, payload)) => {
                let payload = serde_json::from_value(payload).context("decode JWT payload")?;
                Ok(Decoded::Ok(payload))
            }
            Err(err) => match err {
                E::IoError(_) | E::OpenSslError(_) | E::ProtocolError(_) => Err(err.into()),
                err => Ok(Decoded::Invalid(err)),
            },
        }
    }
}
