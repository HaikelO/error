use core::fmt::{self};

use actix_web::{HttpResponse, ResponseError};
use mongodb::error::Error as MongoError;
use serde::Serialize;
use std::io::Error as IOError;
use actix_web::Error as ActixError;
use minio::s3::error::Error as MinioError;

#[derive(Debug, Serialize, Clone)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    SystemError,
    AlreadyExistError,
    UnauthorizedOperation,
    MinioError,
}

impl From<MongoError> for AppErrorType {
    fn from(_error: MongoError) -> AppErrorType {
        AppErrorType::DbError
    }
}

impl From<IOError> for AppErrorType {
    fn from(_error: IOError) -> AppErrorType {
        AppErrorType::SystemError
    }
}

impl From<ActixError> for AppErrorType {
    fn from(_error: ActixError) -> AppErrorType {
        AppErrorType::SystemError
    }
}


impl From<MinioError> for AppErrorType {
    fn from(_error: MinioError) -> AppErrorType {
        AppErrorType::MinioError
    }
}

impl ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}


impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

