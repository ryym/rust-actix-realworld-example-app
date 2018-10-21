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
            ErrorKind::Misc(msg) => {
                log_error(&self);
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .into_builder()
                    .json(ErrorResponse {
                        errors: ErrorData {
                            body: vec![msg.clone()],
                        },
                    })
            }
        }
    }
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
