use actix_web::Error as ActixError;
use backtrace::Backtrace;
use bson::decimal128::ParseError;
use bson::ser::Error as BsonSerError;
use core::fmt;
use minio::s3::error::Error as MinioError;
use mongodb::error::Error as MongoError;
use std::io::Error as IOError;
use std::{
    error::Error as StdError,
    num::{ParseFloatError, ParseIntError},
};

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

impl From<MinioError> for AppError {
    fn from(error: MinioError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::MinioError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<Box<dyn StdError>> for AppError {
    fn from(error: Box<dyn StdError>) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::MinioError,
            backtrace: Backtrace::new(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        AppError {
            message: None,
            cause: Some(err.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<ParseFloatError> for AppError {
    fn from(err: ParseFloatError) -> Self {
        AppError {
            message: None,
            cause: Some(err.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError {
            message: None,
            cause: Some(err.to_string()),
            error_type: AppErrorType::SystemError,
            backtrace: Backtrace::new(),
        }
    }
}
