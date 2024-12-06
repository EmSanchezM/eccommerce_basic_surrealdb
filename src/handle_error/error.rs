use thiserror::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub code: u16,
  pub message: String,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("SurrealDB error: {0}")]
  SurrealDBError(String),

  #[error("Configuration error: {0}")]
  ConfigurationError(String),
}

impl From<surrealdb::Error> for Error {
  fn from(err: surrealdb::Error) -> Self {
    match err {
      surrealdb::Error::Db(db_err) => Error::SurrealDBError(db_err.to_string()),
      surrealdb::Error::Api(api_err) => Error::SurrealDBError(api_err.to_string()),
    }
  }
}

impl From<std::env::VarError> for Error {
  fn from(err: std::env::VarError) -> Self {
    Error::ConfigurationError(err.to_string())
  }
}

impl actix_web::error::ResponseError for Error {
  fn error_response(&self) -> actix_web::HttpResponse {
    let error_response = ErrorResponse {
      code: self.status_code().as_u16(),
      message: self.to_string(),
    };

    actix_web::HttpResponse::build(self.status_code())
      .json(error_response)
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    use actix_web::http::StatusCode;
    
    match *self {
      Error::SurrealDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}