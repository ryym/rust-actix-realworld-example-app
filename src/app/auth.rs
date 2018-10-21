use actix_web::{Json, State};
use frank_jwt as jwt;

use config::HaveConfig;
use prelude::*;

#[derive(Debug, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignUpForm {
    user: NewUser,
}

// TODO: Define as a diesel model.
#[derive(Debug, Serialize)]
pub struct User {
    id: u32,
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignUpResponse {
    user: User,
}

pub fn sign_up<S: HaveConfig>(
    (form, state): (Json<SignUpForm>, State<S>),
) -> Result<Json<SignUpResponse>> {
    debug!("sign up: {:?}", form);

    // TODO: Validate form.
    // TODO: Register user to DB.

    let token = generate_jwt(&state.config().jwt_secret_key, 1)?;
    let user = User {
        token,
        id: 1,
        email: form.user.email.clone(),
        username: form.user.username.clone(),
        bio: None,
        image: None,
    };

    Ok(Json(SignUpResponse { user }))
}

fn generate_jwt(secret_key: &String, user_id: u32) -> Result<String, jwt::Error> {
    // frank_jwt sets the header values automatically.
    let header = json!({});
    let payload = json!({ "id": user_id });
    jwt::encode(header, secret_key, &payload, jwt::Algorithm::HS256)
}
