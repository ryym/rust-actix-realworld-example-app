mod authenticate;
mod jwt;
mod password;
mod register_user;
mod validate_signup;

use actix_web::{Json, State};

use self::{
    authenticate::CanAuthenticate, jwt::CanGenerateJwt, register_user::CanRegisterUser,
    validate_signup::CanValidateSignup,
};
use mdl;
use prelude::*;

mod signup {
    #[derive(Debug, Deserialize)]
    pub struct UserForm {
        pub username: String,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Form {
        pub user: UserForm,
    }
}

mod signin {
    #[derive(Debug, Deserialize)]
    pub struct UserForm {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Form {
        pub user: UserForm,
    }
}

#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl AuthUser {
    fn from_user(token: String, user: mdl::User) -> AuthUser {
        AuthUser {
            token,
            username: user.username,
            email: user.email,
            bio: user.bio,
            image: user.image,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthSuccess {
    pub user: AuthUser,
}

pub fn sign_up<S>((form, hub): (Json<signup::Form>, State<S>)) -> Result<Json<AuthSuccess>>
where
    S: CanValidateSignup + CanRegisterUser + CanGenerateJwt,
{
    debug!("sign up: {:?}", form);

    let form = form.into_inner().user;
    hub.validate_signup(&form)?;

    let user = hub.register_user(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = AuthUser::from_user(token, user);
    Ok(Json(AuthSuccess { user }))
}

pub fn sign_in<S>((form, hub): (Json<signin::Form>, State<S>)) -> Result<Json<AuthSuccess>>
where
    S: CanAuthenticate + CanGenerateJwt,
{
    let form = form.into_inner().user;
    let user = hub.authenticate(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = AuthUser::from_user(token, user);
    Ok(Json(AuthSuccess { user }))
}
