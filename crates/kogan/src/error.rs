use thiserror::Error;

#[derive(Error, Debug)]
pub enum KoganError {
    #[error("invalid kogan env string: {0}")]
    InvalidKoganEnvString(String),
    #[error("invalid kogan credential: {0}={1}")]
    InvalidKoganCredential(&'static str, String),
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("request error: {0:?}, body = {1:?}")]
    RequestError(reqwest::StatusCode, String),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T, E = KoganError> = std::result::Result<T, E>;
