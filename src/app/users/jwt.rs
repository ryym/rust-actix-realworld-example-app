use frank_jwt as jwt;

use config::HaveConfig;
use hub::Hub;
use prelude::*;

impl GenerateJwt for Hub {}

pub trait CanGenerateJwt {
    fn generate_jwt(&self, user_id: i32) -> Result<String>;
}

pub trait GenerateJwt: HaveConfig {}
impl<T: GenerateJwt> CanGenerateJwt for T {
    fn generate_jwt(&self, user_id: i32) -> Result<String> {
        let secret_key = &self.config().jwt_secret_key;

        // frank_jwt sets the header values automatically.
        let header = json!({});
        let payload = json!({ "id": user_id });
        let jwt = jwt::encode(header, secret_key, &payload, jwt::Algorithm::HS256)?;
        Ok(jwt)
    }
}
