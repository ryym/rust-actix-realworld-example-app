use diesel::prelude::*;

use super::SignupUser;
use crate::db;
use crate::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

register_service!(ValidateSignup);

// username
//  - length: 1..=20
//  - must be unique
//  - can't be blank
//  - must match /^[a-zA-Z0-9]+$/
// email
//   - must be unique
//   - can't be blank
//   - must math /\S+@\S+\.\S+/
// password
//   - length: 8..=72

pub trait CanValidateSignup {
    fn validate_signup(&self, form: &SignupUser) -> Result<()>;
}

pub trait ValidateSignup: db::HaveConn + CanValidateSignup {}
impl<T: ValidateSignup> CanValidateSignup for T {
    // XXX: We should implement some generic validation module
    // or find a crate to avoid manual if-else validation.
    fn validate_signup(&self, form: &SignupUser) -> Result<()> {
        use crate::schema::users;
        use diesel::dsl::{exists, select};

        lazy_static! {
            static ref RE_NAME: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
            static ref RE_EMAIL: Regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
        }

        let mut errs = Vec::new();

        // Username
        let name = form.username.trim();
        if name.is_empty() {
            errs.push("username can't be blank".to_owned());
        }
        if name.len() > 20 {
            errs.push("username is too long (max 20 character)".to_owned());
        }
        if !RE_NAME.is_match(name) {
            errs.push("username is invalid format".to_owned());
        }

        let found = select(exists(users::table.filter(users::username.eq(name))))
            .get_result(self.conn())?;
        if found {
            errs.push(format!("username {} already exists", name));
        }

        // Email
        let email = form.email.trim();
        if email.is_empty() {
            errs.push("email can't be blank".to_owned());
        }
        if !RE_EMAIL.is_match(email) {
            errs.push("email is invalid format".to_owned());
        }

        let found =
            select(exists(users::table.filter(users::email.eq(email)))).get_result(self.conn())?;
        if found {
            errs.push(format!("email {} already exists", email));
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::Validation(errs).into())
        }
    }
}
