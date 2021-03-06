mod authenticate;
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
use crate::hub::Store;
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

pub fn sign_up<S>(
    (form, store): (Json<In<SignupUser>>, State<impl Store<Svc = S>>),
) -> Result<Json<UserResponse>>
where
    S: CanValidateSignup + CanRegisterUser + CanGenerateJwt,
{
    let hub = store.service()?;

    let form = form.into_inner().user;
    hub.validate_signup(&form)?;

    let user = hub.register_user(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn sign_in<S>(
    (form, store): (Json<In<SigninUser>>, State<impl Store<Svc = S>>),
) -> Result<Json<UserResponse>>
where
    S: CanAuthenticate + CanGenerateJwt,
{
    let hub = store.service()?;
    let form = form.into_inner().user;
    let user = hub.authenticate(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = User::from_model(token, user);
    Ok(Json(UserResponse { user }))
}

pub fn get_current(auth: Auth) -> Result<Json<UserResponse>> {
    let user = User::from_model(auth.token, auth.user);
    Ok(Json(UserResponse { user }))
}

pub fn update<S>(
    (store, form, auth): (State<impl Store<Svc = S>>, Json<In<UserChange>>, Auth),
) -> Result<Json<UserResponse>>
where
    S: CanUpdateUser,
{
    // TODO: Validate input.
    let form = form.into_inner().user;

    let hub = store.service()?;
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

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test::TestRequest, FromRequest, Json, State};
    use chrono::NaiveDate;
    use crate::mdl;
    use crate::test::{Mock, Store};

    #[test]
    fn sign_up_registers_user() -> Result<()> {
        impl CanValidateSignup for Mock {
            fn validate_signup(&self, _form: &SignupUser) -> Result<()> {
                Ok(())
            }
        }
        impl CanRegisterUser for Mock {
            fn register_user(&self, form: &SignupUser) -> Result<mdl::User> {
                let created_at = NaiveDate::from_ymd(2018, 1, 1).and_hms(9, 10, 11);
                let user = mdl::User {
                    id: 1,
                    username: form.username.clone(),
                    email: form.email.clone(),
                    bio: None,
                    image: None,
                    created_at,
                    updated_at: created_at,
                };
                Ok(user)
            }
        }
        impl CanGenerateJwt for Mock {
            fn generate_jwt(&self, user_id: i32) -> Result<String> {
                Ok(format!("jwt-token-{}", user_id))
            }
        }

        let store = Store(Mock {});
        let req = TestRequest::with_state(store).finish();
        let state = State::extract(&req);
        let form = Json(In {
            user: SignupUser {
                username: "hello".to_owned(),
                email: "a@b.cc".to_owned(),
                password: "password".to_owned(),
            },
        });

        let res = sign_up((form, state))?;
        assert_eq!(
            res.user,
            User {
                username: "hello".to_owned(),
                email: "a@b.cc".to_owned(),
                token: "jwt-token-1".to_owned(),
                bio: None,
                image: None,
            }
        );

        Ok(())
    }
}
