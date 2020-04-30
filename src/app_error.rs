use log::{error};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Error, ResponseError};
use core::fmt;

#[derive(Debug)]
pub struct AppError {
    cause: String,
    status: StatusCode,
}

impl AppError {
    pub fn new(cause: String, status: StatusCode) -> AppError {
        AppError { cause, status }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl ResponseError for AppError {}

// impl Into<actix_web::Error> for AppError {
//     fn into(self) -> Error {
//         Error::
//     }
// }

impl From<AppError> for HttpResponse {
    fn from(err: AppError) -> Self {
        HttpResponse::with_body(err.status, err.cause.into())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        error!("{}", err);
        AppError { cause: "Internal server error".to_string(), status: StatusCode::from_u16(500).unwrap() }
    }
}