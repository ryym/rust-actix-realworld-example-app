use actix_web::{
    http::StatusCode, middleware::Response, Error as ActixError, HttpRequest, HttpResponse,
    ResponseError,
};

use crate::error::ErrorKindAuth;
use crate::prelude::*;

#[derive(Debug, Serialize)]
struct ErrorData {
    body: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    errors: ErrorData,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log_error(&self);

        match self.kind() {
            ErrorKind::Validation(msgs) => error_res(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorData {
                    body: msgs.to_vec(),
                },
            ),
            ErrorKind::Auth(kind) => error_res(
                match kind {
                    ErrorKindAuth::Forbidden => StatusCode::FORBIDDEN,
                    _ => StatusCode::UNAUTHORIZED,
                },
                ErrorData {
                    body: vec![self.to_string()],
                },
            ),
            ErrorKind::Db => error_res(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorData {
                    body: vec![self.to_string()],
                },
            ),
            ErrorKind::NotFound => error_res(
                StatusCode::NOT_FOUND,
                ErrorData {
                    body: vec![self.to_string()],
                },
            ),
            ErrorKind::Misc(msg) => error_res(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorData {
                    body: vec![msg.clone()],
                },
            ),
        }
    }
}

fn error_res(code: StatusCode, errors: ErrorData) -> HttpResponse {
    HttpResponse::new(code)
        .into_builder()
        .json(ErrorResponse { errors })
}

fn log_error(err: &Error) {
    if let Some(c) = err.cause() {
        let causes = c
            .iter_chain()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        log::error!("ERROR: {}\nCAUSE: {}", err, causes);
    } else {
        log::error!("ERROR: {}", err);
    };
}

pub fn not_found<S>(_: &HttpRequest<S>, res: HttpResponse) -> Result<Response, ActixError> {
    let res = res.into_builder().json(ErrorResponse {
        errors: ErrorData {
            body: vec!["not found".to_owned()],
        },
    });
    Ok(Response::Done(res))
}
