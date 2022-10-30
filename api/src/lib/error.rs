use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::errors::Error as JwtError;
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum Error {
    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {0}")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized(message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(message) => {
                HttpResponse::UnprocessableEntity().json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        Error::Unauthorized(json!({"error": "An issue was found with the token provided"}))
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors.iter().map(|error| json!(error.message)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::UnprocessableEntity(json!({ "errors": err_map }))
    }
}
