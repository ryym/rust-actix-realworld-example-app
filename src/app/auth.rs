use actix_web::Json;
use frank_jwt as jwt;

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

pub fn sign_up(form: Json<SignUpForm>) -> Result<Json<SignUpResponse>> {
    debug!("sign up: {:?}", form);

    // TODO: Validate form.
    // TODO: Register user to DB.

    let token = generate_jwt(1)?;
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

fn generate_jwt(user_id: u32) -> Result<String, jwt::Error> {
    // frank_jwt sets the header values automatically.
    let header = json!({});
    let payload = json!({ "id": user_id });
    // TODO: Load secret key from outside.
    let secret = "secret".to_owned();
    jwt::encode(header, &secret, &payload, jwt::Algorithm::HS256)
}
