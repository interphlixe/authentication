use actix_web::{http::StatusCode, ResponseError, HttpResponse as Response, body::BoxBody as Body};
use std::error::Error as StdError;
use serde::Serialize;
use std::fmt::*;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    message: String,
    #[serde(skip)]
    code: StatusCode,
    #[serde(skip)]
    content: Option<String>,
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}


impl ResponseError for Error {
    fn error_response(&self) -> Response<Body> {
        Response::build(self.code).json(self)
    }
}


impl<T: StdError> From<T> for Error {
    fn from(err: T) -> Self {
        let message = format!("{}", err);
        let code = StatusCode::BAD_REQUEST;
        let content = None;
        Error{message, code, content}
    }
}