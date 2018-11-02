use diesel::prelude::*;

use super::SignupUser;
use crate::db;
use crate::hub::Hub;
use crate::prelude::*;

impl ValidateSignup for Hub {}

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
    // XXX: We should implement some generic validation module
    // or find a crate to avoid manual if-else validation.
    // TODO: Implement all validations.
    fn validate_signup(&self, conn: &db::Conn, form: &SignupUser) -> Result<()> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::{exists, select};

        let mut errs = Vec::new();

        // Username
        let name = form.username.trim();
        if name.is_empty() {
            errs.push("username can't be blank".to_owned());
        }
        if name.len() > 20 {
            errs.push("username is too long (max 20 character)".to_owned());
        }

        let found = select(exists(users.filter(username.eq(name)))).get_result(conn)?;
        if found {
            errs.push(format!("username {} already exists", name));
        }

        // Email
        let form_email = form.email.trim();
        if form_email.is_empty() {
            errs.push("email can't be blank".to_owned());
        }

        let found = select(exists(users.filter(email.eq(form_email)))).get_result(conn)?;
        if found {
            errs.push(format!("email {} already exists", form_email));
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::Validation(errs).into())
        }
    }
}

pub trait ValidateSignup {}
impl<T: ValidateSignup> CanValidateSignup for T {}
