use super::{password::CanCheckPassword, SigninUser};
use crate::db;
use crate::mdl::{Credential, User};
use crate::prelude::*;

register_service!(Authenticate);

pub trait CanAuthenticate {
    fn authenticate(&self, form: &SigninUser) -> Result<User>;
}

pub trait Authenticate: db::HaveConn + CanCheckPassword {}
impl<T: Authenticate> CanAuthenticate for T {
    fn authenticate(&self, form: &SigninUser) -> Result<User> {
        let user_cred = find_user_with_cred(self.conn(), &form.email)?;

        if let Some((user, cred)) = user_cred {
            if self.is_correct_password(&form.password, &cred.password_hash) {
                return Ok(user);
            }
        }

        Err(ErrorKind::Validation(vec!["email or password is invalid".to_owned()]).into())
    }
}

fn find_user_with_cred(conn: &db::Conn, email: &str) -> Result<Option<(User, Credential)>> {
    use crate::schema::{credentials, users};
    use diesel::prelude::*;

    let user_cred = users::table
        .inner_join(credentials::table)
        .filter(users::email.eq(email))
        .first::<(User, Credential)>(conn)
        .optional()?;

    Ok(user_cred)
}
