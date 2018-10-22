use super::{password::CanCheckPassword, signin::UserForm};
use db::HaveDb;
use hub::Hub;
use mdl::{Credential, User};
use prelude::*;

impl Authenticate for Hub {}

pub trait CanAuthenticate {
    fn authenticate(&self, form: &UserForm) -> Result<User>;
}

pub trait Authenticate: HaveDb + CanCheckPassword {}
impl<T: Authenticate> CanAuthenticate for T {
    fn authenticate(&self, form: &UserForm) -> Result<User> {
        let user_cred = self.use_db(|conn| {
            use diesel::prelude::*;
            use schema::{credentials::dsl::*, users::dsl::*};

            let data = users
                .inner_join(credentials)
                .filter(email.eq(&form.email))
                .first::<(User, Credential)>(conn)
                .optional()?;

            Ok(data)
        })?;

        if let Some((user, cred)) = user_cred {
            if self.is_correct_password(&form.password, &cred.password_hash) {
                return Ok(user);
            }
        }

        Err(ErrorKind::Validation(vec!["email or password is invalid".to_owned()]).into())
    }
}
