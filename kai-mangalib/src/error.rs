#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("HTTP request failed: {}", .0)]
    HttpError(#[from] reqwest::Error),

    #[error("Tower error: {}", .0)]
    TowerError(#[from] tower::BoxError),
}

// todo: In error.rs: Consider adding variants for API-specific errors (rate limited, unauthorized, etc.) rather than lumping everything into HttpError.