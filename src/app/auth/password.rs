use pbkdf2::{pbkdf2_check, pbkdf2_simple};

use hub::Hub;
use prelude::*;

impl CanHashPassword for Hub {}
impl CanCheckPassword for Hub {}

const HASH_ITERATION_COUNT: u32 = 10000;

pub trait CanHashPassword {
    fn hash_password(&self, password: &str) -> Result<String> {
        let hash = pbkdf2_simple(password, HASH_ITERATION_COUNT).context("hash password")?;
        Ok(hash)
    }
}

pub trait CanCheckPassword {
    fn is_correct_password(&self, input: &str, hashed_value: &str) -> bool {
        pbkdf2_check(input, hashed_value).is_ok()
    }
}