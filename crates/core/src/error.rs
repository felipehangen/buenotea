use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("API error from {0}: {1}")]
    ApiError(String, String),

    #[error("Rate limit exceeded for {0}")]
    RateLimitExceeded(String),

    #[error("Invalid API response format from {0}: {1}")]
    InvalidResponseFormat(String, String),

    #[error("Missing API key for {0}")]
    MissingApiKey(String),

    #[error("Data validation failed: {message}")]
    ValidationError { message: String },

    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
