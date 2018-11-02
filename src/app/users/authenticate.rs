use super::{password::CanCheckPassword, SigninUser};
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Credential, User};
use crate::prelude::*;

impl Authenticate for Hub {}

pub trait CanAuthenticate {
    fn authenticate(&self, conn: &db::Conn, form: &SigninUser) -> Result<User>;
}

pub trait Authenticate: CanCheckPassword {}
impl<T: Authenticate> CanAuthenticate for T {
    fn authenticate(&self, conn: &db::Conn, form: &SigninUser) -> Result<User> {
        use crate::schema::{credentials::dsl::*, users::dsl::*};
        use diesel::prelude::*;

        let user_cred = users
            .inner_join(credentials)
            .filter(email.eq(&form.email))
            .first::<(User, Credential)>(conn)
            .optional()?;

        if let Some((user, cred)) = user_cred {
            if self.is_correct_password(&form.password, &cred.password_hash) {
                return Ok(user);
            }
        }

        Err(ErrorKind::Validation(vec!["email or password is invalid".to_owned()]).into())
    }
}
