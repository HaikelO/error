use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use backtrace::Backtrace;
use serde::Serialize;

use crate::enums::app_error_type::AppErrorType;

#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
    pub backtrace: Backtrace,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
    pub status_code: u16,
    pub backtrace: String,
}

impl AppError {
    pub fn new(cause: Option<String>, message: Option<String>, error_type: AppErrorType) -> Self {
        AppError {
            cause,
            message,
            error_type,
            backtrace: Backtrace::new(),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::SystemError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::AlreadyExistError => StatusCode::BAD_REQUEST,
            AppErrorType::UnauthorizedOperation => StatusCode::NOT_ACCEPTABLE
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            cause: self.cause.clone(),
            message: self.message.clone(),
            error_type: self.error_type.clone(),
            status_code: self.status_code().as_u16(),
            backtrace: format!("{:?}", self.backtrace),
        })
    }
}
