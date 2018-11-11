// https://github.com/rust-lang-nursery/failure
// https://rust-lang-nursery.github.io/failure/error-errorkind.html

use diesel::result::Error as DieselError;
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Clone, Fail)]
pub enum ErrorKind {
    #[fail(display = "validation failure")]
    Validation(Vec<String>),

    #[fail(display = "authentication failure: {}", _0)]
    Auth(ErrorKindAuth),

    #[fail(display = "database operation failure")]
    Db,

    #[fail(display = "record not found")]
    NotFound,

    #[fail(display = "{}", _0)]
    Misc(String),
}

#[derive(Debug, Clone)]
pub enum ErrorKindAuth {
    NoAuthToken,
    InvalidToken,
    InvalidUser,
    Forbidden,
}

impl Display for ErrorKindAuth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ErrorKindAuth::NoAuthToken => "no auth token",
            ErrorKindAuth::InvalidToken => "invalid token",
            ErrorKindAuth::InvalidUser => "invalid user",
            ErrorKindAuth::Forbidden => "forbidden",
        };
        write!(f, "{}", s)
    }
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

impl From<ErrorKindAuth> for Error {
    fn from(kind: ErrorKindAuth) -> Error {
        Error {
            inner: Context::new(ErrorKind::Auth(kind)),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<Context<ErrorKindAuth>> for Error {
    fn from(ctx: Context<ErrorKindAuth>) -> Error {
        Error {
            inner: ctx.map(|k| ErrorKind::Auth(k)),
        }
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

// We need to be able to convert diesel Error to our Error type implicitly
// to satisfy the trait bound of diesel's Connection::transaction.
impl From<DieselError> for Error {
    fn from(err: DieselError) -> Error {
        match err {
            DieselError::NotFound => err.context(ErrorKind::NotFound),
            _ => err.context(ErrorKind::Db),
        }.into()
    }
}

#[cfg(test)]
impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::Misc(err.to_string())),
        }
    }
}
