// https://github.com/rust-lang-nursery/failure
// https://rust-lang-nursery.github.io/failure/error-errorkind.html

use failure::{Backtrace, Context, Fail};
use frank_jwt as jwt;
use std::{
    error,
    fmt::{self, Debug, Display},
};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Misc(String),
    // Auth, Validation, etc
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl<S> From<Context<S>> for Error
where
    S: Into<String> + Display + Sync + Send,
{
    fn from(ctx: Context<S>) -> Error {
        Error {
            inner: ctx.map(|s| ErrorKind::Misc(s.into())),
        }
    }
}

// TODO: Update frank_jwt to the next version when released.
// In v3.0.2, frank_jwt's Error does not implement std::error::Error
// but it was fixed in the master branch.
#[derive(Debug)]
pub struct JwtError(jwt::Error);

impl Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl error::Error for JwtError {}

impl From<jwt::Error> for Error {
    fn from(err: jwt::Error) -> Error {
        From::from(JwtError(err).context("JWT error"))
    }
}
