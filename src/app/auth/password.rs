use pbkdf2::pbkdf2_simple;

use prelude::*;
use hub::Hub;

impl HashPassword for Hub {}

const HASH_ITERATION_COUNT: u32 = 10000;

pub trait CanHashPassword {
    fn hash_password(&self, password: &str) -> Result<String>;
}

pub trait HashPassword {}
impl<T: HashPassword> CanHashPassword for T {
    fn hash_password(&self, password: &str) -> Result<String> {
        let hash = pbkdf2_simple(password, HASH_ITERATION_COUNT).context("hash password")?;
        Ok(hash)
    }
}
