use core::fmt;

//use actix_web::{http::{Error, StatusCode}, HttpResponse, ResponseError};

use actix_web::Error as ActixError;
use backtrace::Backtrace;
use bson::ser::Error as BsonSerError;
use mongodb::error::Error as MongoError;
use std::io::Error as IOError;

use crate::{enums::app_error_type::AppErrorType, models::app_error::AppError};

impl From<MongoError> for AppError {
    fn from(error: MongoError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<IOError> for AppError {
    fn from(error: IOError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<ActixError> for AppError {
    fn from(error: ActixError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<BsonSerError> for AppError {
    fn from(error: BsonSerError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
