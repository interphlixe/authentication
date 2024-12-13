use actix_web::{HttpResponse, http::StatusCode, ResponseError};
use std::fmt::{Display, Formatter, Result};
use std::error::Error as StdError;
use sqlx::Error as SqlxError;
use serde_json::json;

type DefaultError = Box<dyn StdError>;

#[derive(Debug)]
pub enum Error {
    UserWithEmailExists,
    UserNotFound,
    InternalServerError(Option<DefaultError>),
    Custom(StatusCode, DefaultError)
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Error::*;
        match self {
            UserWithEmailExists => write!(f, "user with the same email already exists."),
            UserNotFound => write!(f, "user not found"),
            InternalServerError(err) => write!(f, "{}", err.as_ref().unwrap_or(&"internal server error".into())),
            Custom(_, err) => write!(f, "custom: {}", err)
        }
    }
}


impl StdError for Error {}


impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        use Error::*;
        match self {
            UserWithEmailExists => HttpResponse::Conflict().json(json!({"message": "user with the same email already exists"})),
            UserNotFound => HttpResponse::NotFound().json(json!({"message": "user not found"})),
            InternalServerError(_) => HttpResponse::InternalServerError().json(json!({"message": "internal server error"})),
            Custom(status, _) => HttpResponse::build(*status).json(json!({"message": "unexpected error. will be worked on soon"}))
        }
    }
}


impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}


impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}