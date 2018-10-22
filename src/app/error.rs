use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use prelude::*;

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
        match self.kind() {
            ErrorKind::Validation(msgs) => error_res(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorData {
                    body: msgs.to_vec(),
                },
            ),
            ErrorKind::Db => {
                log_error(&self);
                error_res(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorData {
                        body: vec![format!("{}", self)],
                    },
                )
            }
            ErrorKind::Misc(msg) => {
                log_error(&self);
                error_res(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorData {
                        body: vec![msg.clone()],
                    },
                )
            }
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
        error!("ERROR: {}\nCAUSE: {}", err, causes);
    } else {
        error!("ERROR: {}", err);
    };
}
