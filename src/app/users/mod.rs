mod authenticate;
mod password;
mod register_user;
mod update_user;
mod validate_signup;

use actix_web::{Json, State};

use self::{
    authenticate::CanAuthenticate,
    register_user::CanRegisterUser,
    update_user::{CanUpdateUser, UserChanges},
    validate_signup::CanValidateSignup,
};
use auth::Auth;
use jwt::CanGenerateJwt;
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
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserChange {
    pub email: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserChangeForm {
    user: UserChange,
}

impl User {
    fn from_model(token: String, user: mdl::User) -> User {
        User {
            token,
            username: user.username,
            email: user.email,
            bio: user.bio,
            image: user.image,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: User,
}

pub fn sign_up<S>((form, hub): (Json<signup::Form>, State<S>)) -> Result<Json<UserResponse>>
where
    S: CanValidateSignup + CanRegisterUser + CanGenerateJwt,
{
    debug!("sign up: {:?}", form);

    let form = form.into_inner().user;
    hub.validate_signup(&form)?;

    let user = hub.register_user(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn sign_in<S>((form, hub): (Json<signin::Form>, State<S>)) -> Result<Json<UserResponse>>
where
    S: CanAuthenticate + CanGenerateJwt,
{
    let form = form.into_inner().user;
    let user = hub.authenticate(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn get_user(auth: Auth) -> Result<Json<UserResponse>> {
    let user = User::from_model(auth.token, auth.user);
    Ok(Json(UserResponse { user }))
}

pub fn update_user<S>(
    (hub, form, auth): (State<S>, Json<UserChangeForm>, Auth),
) -> Result<Json<UserResponse>>
where
    S: CanUpdateUser,
{
    // TODO: Validate input.
    let form = form.into_inner().user;
    let user = hub.update_user(
        auth.user,
        UserChanges {
            user: mdl::UserChange {
                username: form.username,
                email: form.email,
                bio: Some(form.bio),
                image: Some(form.image),
            },
            new_password: form.password,
        },
    )?;

    let user = User::from_model(auth.token, user);
    Ok(Json(UserResponse { user }))
}
