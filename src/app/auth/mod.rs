mod jwt;
mod register_user;

use actix_web::{Json, State};

use self::{jwt::CanGenerateJwt, register_user::CanRegisterUser};
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

    #[derive(Debug, Serialize)]
    pub struct User {
        pub email: String,
        pub token: String,
        pub username: String,
        pub bio: Option<String>,
        pub image: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct Success {
        pub user: User,
    }
}

pub fn sign_up<S>((form, hub): (Json<signup::Form>, State<S>)) -> Result<Json<signup::Success>>
where
    S: CanRegisterUser + CanGenerateJwt,
{
    debug!("sign up: {:?}", form);

    // TODO: Validate form.
    let form = form.into_inner().user;

    let user = hub.register_user(&form)?;
    let token = hub.generate_jwt(user.id)?;

    let user = signup::User {
        token,
        username: user.username,
        email: user.email,
        bio: user.bio,
        image: user.image,
    };
    Ok(Json(signup::Success { user }))
}
