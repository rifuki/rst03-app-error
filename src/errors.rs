use thiserror::Error as ThisError;
use serde_json::{
    Value as JsonValue,
    json,
};
use actix_web::{
    ResponseError,
    HttpResponse,
    body::BoxBody
};
use serde::Serialize;
use sqlx::Error as SqlxError;
use validator::ValidationErrors;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),
    #[error("Internal Server Error: {0}")]
    InternalServerError(JsonValue),
    #[error("Conflict: {0}")]
    Conflict(JsonValue),
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue)
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::UnprocessableEntity(ref message) => HttpResponse::UnprocessableEntity().json(message),
            Self::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            Self::Conflict(ref message) => HttpResponse::Conflict().json(message),
            Self::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message)
        }
    }
}

pub struct JsonErrorMessage<T> {
    pub code: i32,
    pub message: String,
    pub details: Option<T>
}

impl<T> From<JsonErrorMessage<T>> for JsonValue
where T: Serialize 
{
    fn from(value: JsonErrorMessage<T>) -> Self {
        json!({
            "error": json!({
                "code": value.code,
                "message": value.message,
                "details": Some(value.details)
            })
        })
    }
}

impl From<SqlxError> for AppError {
    fn from(value: SqlxError) -> Self {
        let json_error_message = JsonErrorMessage {
            code: 500,
            message: String::from("database error"),
            details: Some(
                value.to_string()
            )
        };
        Self::InternalServerError(json_error_message.into())
    }
}

impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        let mut cleaned_errors = Vec::new();
        for (field, field_errors) in value.field_errors().iter() {
            let cleaned_field_errors = field_errors   
                .iter()
                .map(|error| {
                    json!(error.message)
                })
                .collect::<Vec<JsonValue>>();
            cleaned_errors.push(
                json!({
                    field.to_string(): JsonValue::Array(cleaned_field_errors)
                })
            );
        }
        let json_error_message = JsonErrorMessage {
            code: 422,
            message: String::from("validation error"),
            details: Some(cleaned_errors)
        };
        AppError::UnprocessableEntity(json_error_message.into())
    }
}