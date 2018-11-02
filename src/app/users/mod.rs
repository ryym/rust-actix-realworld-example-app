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
use super::res::{User, UserResponse};
use crate::auth::Auth;
use crate::db::HaveDb;
use crate::jwt::CanGenerateJwt;
use crate::mdl;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

#[derive(Debug, Deserialize)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserChange {
    pub email: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}

pub fn sign_up<S>((form, hub): (Json<In<SignupUser>>, State<S>)) -> Result<Json<UserResponse>>
where
    S: HaveDb + CanValidateSignup + CanRegisterUser + CanGenerateJwt,
{
    let form = form.into_inner().user;

    let user = hub.use_db(|conn| {
        hub.validate_signup(conn, &form)?;
        hub.register_user(conn, &form)
    })?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn sign_in<S>((form, hub): (Json<In<SigninUser>>, State<S>)) -> Result<Json<UserResponse>>
where
    S: HaveDb + CanAuthenticate + CanGenerateJwt,
{
    let form = form.into_inner().user;
    let user = hub.use_db(|conn| hub.authenticate(conn, &form))?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn get_current(auth: Auth) -> Result<Json<UserResponse>> {
    let user = User::from_model(auth.token, auth.user);
    Ok(Json(UserResponse { user }))
}

pub fn update<S>(
    (hub, form, auth): (State<S>, Json<In<UserChange>>, Auth),
) -> Result<Json<UserResponse>>
where
    S: HaveDb + CanUpdateUser,
{
    // TODO: Validate input.
    let form = form.into_inner().user;

    hub.use_db(|conn| {
        let user = hub.update_user(
            &conn,
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
    })
}
