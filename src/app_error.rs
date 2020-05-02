use core::fmt;

use actix_web::{Error, HttpResponse, ResponseError};
use actix_web::{
    error::{BlockingError},
};
use actix_web::http::StatusCode;
use log::*;
use log::error;
use serde::{Deserialize, Serialize};
// use diesel::{
//     r2d2::PoolError,
//     result::{DatabaseErrorKind, Error as DBError},
// };
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum AppError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            AppError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            AppError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            }
            AppError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> AppError {
        error!("Pool Error Error {:?}", error);
        AppError::PoolError(error.to_string())
    }
}
impl From<postgres::Error> for AppError {
    fn from(error: postgres::Error) -> AppError {
        error!("Pool Error Error {:?}", error);
        AppError::InternalServerError("Internal Server error".into())
    }
}

// impl From<DBError> for AppError {
//     fn from(error: DBError) -> AppError {
//         error!("Database Error {:?}", error);
//         match error {
//             DBError::DatabaseError(kind, info) => {
//                 if let DatabaseErrorKind::UniqueViolation = kind {
//                     let message = info.details().unwrap_or_else(|| info.message()).to_string();
//                     return AppError::BadRequest(message);
//                 }
//                 AppError::InternalServerError("Unknown database error".into())
//             }
//             _ => AppError::InternalServerError("Unknown database error".into()),
//         }
//     }
// }
// impl From<embedded_migrations::diesel::result::Error> for AppError {
//     fn from(error: embedded_migrations::diesel::result::Error) -> AppError {
//         error!("Pool Error Error {:?}", error);
//         AppError::PoolError(error.to_string())
//     }
// }
// impl From<sqlx::Error> for AppError {
//     fn from(err: sqlx::Error) -> Self {
//         error!("{}", err);
//         AppError::InternalServerError("Unknown database error".into())
//     }
// }

// impl From<ParseError> for AppError {
//     fn from(error: ParseError) -> AppError {
//         error!("Parse Error {:?}", error);
//         AppError::ParseError(error.to_string())
//     }
// }

/// Convert Thread BlockingErrors to ApiErrors
impl From<BlockingError<AppError>> for AppError {
    fn from(error: BlockingError<AppError>) -> AppError {
        error!("Blocking Error {:?}", error);
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => AppError::BlockingError("Thread blocking error".into()),
        }
    }
}
