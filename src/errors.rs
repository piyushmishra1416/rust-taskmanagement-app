use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum AppError {
    UserNotFound,
    TaskNotFound,
    InvalidInput,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::UserNotFound => write!(f, "User not found"),
            AppError::TaskNotFound => write!(f, "Task not found"),
            AppError::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let response = ErrorResponse {
            error: error_message.clone(),
        };
        match *self {
            AppError::UserNotFound => HttpResponse::NotFound().json(response),
            AppError::TaskNotFound => HttpResponse::NotFound().json(response),
            AppError::InvalidInput => HttpResponse::BadRequest().json(response),
        }
    }
}
