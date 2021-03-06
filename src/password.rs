use pbkdf2::{pbkdf2_check, pbkdf2_simple};

use crate::prelude::*;

register_service!(CanHashPassword);
register_service!(CanCheckPassword);

const HASH_ITERATION_COUNT: u32 = 10000;

pub struct HashedPassword(String);

impl HashedPassword {
    #[cfg(test)]
    pub fn new<S: Into<String>>(hashed: S) -> Self {
        HashedPassword(hashed.into())
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        HashedPassword("dummy".to_owned())
    }
}

impl Into<String> for HashedPassword {
    fn into(self) -> String {
        self.0
    }
}

pub trait CanHashPassword {
    fn hash_password(&self, password: &str) -> Result<HashedPassword> {
        let hash = pbkdf2_simple(password, HASH_ITERATION_COUNT).context("hash password")?;
        Ok(HashedPassword(hash))
    }
}

pub trait CanCheckPassword {
    fn is_correct_password(&self, input: &str, hashed_value: &str) -> bool {
        pbkdf2_check(input, hashed_value).is_ok()
    }
}
