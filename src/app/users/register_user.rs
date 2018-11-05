use super::SignupUser;
use crate::db;
use crate::mdl::{NewUser, User};
use crate::password::CanHashPassword;
use crate::prelude::*;

register_service!(RegisterUser);

pub trait CanRegisterUser {
    fn register_user(&self, form: &SignupUser) -> Result<User>;
}

pub trait RegisterUser: db::HaveConn + CanHashPassword {}
impl<T: RegisterUser> CanRegisterUser for T {
    fn register_user(&self, form: &SignupUser) -> Result<User> {
        let new_user = NewUser {
            username: form.username.clone(),
            email: form.email.clone(),
            bio: None,
            image: None,
        };
        let password_hash = self.hash_password(&form.password)?;
        db::users::insert(self.conn(), &new_user, password_hash)
    }
}
