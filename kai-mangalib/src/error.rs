#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("HTTP request failed: {}", .0)]
    HttpError(#[from] reqwest::Error),

    #[error("Tower error: {}", .0)]
    TowerError(#[from] tower::BoxError),

    #[error("Request build error: {0}")]
    RequestBuildError(String),

    /// The request was rejected before it reached the resource, so the body
    /// carries a validation report instead of the expected payload.
    #[error("API rejected the request ({status}): {message}")]
    ApiError { status: u16, message: String },

    /// The endpoint requires an account and none was configured.
    #[error("authentication required")]
    Unauthorized,

    /// Upstream asked us to slow down.
    #[error("rate limited by the API")]
    RateLimited,
}
