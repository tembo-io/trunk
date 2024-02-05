//! Custom errors types for extension registry
use actix_multipart::MultipartError;
use actix_web::error;
use actix_web::http::header::ToStrError;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::PutObjectError;
use reqwest::StatusCode;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error;
use url::ParseError;

pub type Result<T = ()> = std::result::Result<T, ExtensionRegistryError>;

// Use default implementation for `error_response()` method
impl actix_web::error::ResponseError for ExtensionRegistryError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ExtensionRegistryError::ErrorBadRequest(_) => StatusCode::BAD_REQUEST,
            ExtensionRegistryError::AuthorizationError(_) => StatusCode::UNAUTHORIZED,
            ExtensionRegistryError::ResourceNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Error, Debug)]
pub enum ExtensionRegistryError {
    /// a url parsing error
    #[error("url parsing error: {0}")]
    UrlParsingError(#[from] ParseError),

    /// a database error
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// a response error
    #[error("response error")]
    ResponseError(),

    /// an authorization error
    #[error("authorization error: {0}")]
    AuthorizationError(String),

    /// a payload error
    #[error("payload error: {0}")]
    PayloadError(#[from] error::PayloadError),

    /// a bad request error
    #[error("bad request error: {0}")]
    ErrorBadRequest(#[from] error::Error),

    /// a serde json error
    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// a multipart error
    #[error("multipart error: {0}")]
    MultipartError(#[from] MultipartError),

    /// a reqwest error
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// a std io error
    #[error("std io error: {0}")]
    StdIoError(#[from] std::io::Error),

    /// a put object error
    #[error("put object error: {0}")]
    PutObjectError(#[from] SdkError<PutObjectError>),

    #[error("token error: {0}")]
    TokenError(String),

    #[error("byte error: {0}")]
    ByteError(#[from] Utf8Error),

    #[error("to str error: {0}")]
    ToStrError(#[from] ToStrError),

    #[error("failed to convert bytes to UTF-8: {0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("received malformed JWT")]
    MalformedJwt,

    #[error("resource not found")]
    ResourceNotFound,

    #[error("Invalid base64: {0}")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("Not a GitHub repository: {0}")]
    InvalidGithubRepo(String),

    #[error("Failed to decompress Trunk archive")]
    ArchiveError,
}
