#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Request build error: {0}")]
    RequestBuildError(String),

    #[error("Service error: {0}")]
    ServiceError(String),
}
