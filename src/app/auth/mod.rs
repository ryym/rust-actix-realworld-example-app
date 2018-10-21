use actix_web::{Json, State};
use diesel;
use frank_jwt as jwt;
use pbkdf2::pbkdf2_simple;

use config::HaveConfig;
use db::HaveDb;
use mdl::{NewCredential, NewUser, User};
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

pub fn sign_up<S: HaveConfig + HaveDb>(
    (form, state): (Json<signup::Form>, State<S>),
) -> Result<Json<signup::Success>> {
    debug!("sign up: {:?}", form);

    // TODO: Validate form.
    let form = form.into_inner().user;

    let user = state.use_db(|conn| {
        use diesel::prelude::*;
        use schema::{credentials::dsl::*, users::dsl::*};
        conn.transaction(|| {
            let new_user = NewUser {
                username: form.username.clone(),
                email: form.email.clone(),
                bio: None,
                image: None,
            };
            let user = diesel::insert_into(users)
                .values(&new_user)
                .get_result::<User>(conn)
                .context("register user")?;

            let new_cred = NewCredential {
                user_id: user.id,
                password_hash: hash_password(&form.password)?,
            };
            diesel::insert_into(credentials)
                .values(&new_cred)
                .execute(conn)
                .context("register credential")?;

            Ok(user)
        })
    })?;

    let token = generate_jwt(&state.config().jwt_secret_key, user.id)?;
    let user = signup::User {
        token,
        username: user.username,
        email: user.email,
        bio: user.bio,
        image: user.image,
    };

    Ok(Json(signup::Success { user }))
}

fn generate_jwt(secret_key: &String, user_id: i32) -> Result<String, jwt::Error> {
    // frank_jwt sets the header values automatically.
    let header = json!({});
    let payload = json!({ "id": user_id });
    jwt::encode(header, secret_key, &payload, jwt::Algorithm::HS256)
}

fn hash_password(value: &str) -> Result<String> {
    let hash = pbkdf2_simple(value, 10000).context("hash password")?;
    Ok(hash)
}
