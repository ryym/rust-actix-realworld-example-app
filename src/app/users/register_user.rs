use diesel::{self, prelude::*};

use super::password::CanHashPassword;
use super::SignupUser;
use db::{self, HaveDb};
use hub::Hub;
use mdl::{NewCredential, NewUser, User};
use prelude::*;

impl RegisterUser for Hub {}

pub trait CanRegisterUser {
    fn register_user(&self, form: &SignupUser) -> Result<User>;
}

pub trait RegisterUser: HaveDb + CanHashPassword {}
impl<T: RegisterUser> CanRegisterUser for T {
    fn register_user(&self, form: &SignupUser) -> Result<User> {
        let user = self.use_db(|conn| {
            conn.transaction(|| {
                let user = insert_user(conn, &form)?;
                let password_hash = self.hash_password(&form.password)?;
                insert_credential(conn, user.id, password_hash)?;
                Ok(user)
            })
        })?;

        Ok(user)
    }
}

fn insert_user(conn: &db::Connection, form: &SignupUser) -> Result<User> {
    use schema::users::dsl::*;

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
    Ok(user)
}

fn insert_credential(conn: &db::Connection, user_id: i32, password_hash: String) -> Result<()> {
    use schema::credentials;

    let new_cred = NewCredential {
        user_id,
        password_hash,
    };
    diesel::insert_into(credentials::table)
        .values(&new_cred)
        .execute(conn)
        .context("register credential")?;
    Ok(())
}
