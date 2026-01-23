use reqwest::{Error as ReqwestError, StatusCode, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// This should be a central place for keep all our shared Errors

#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{:?} ({status:}, {body})", url.as_ref().unwrap().path())]
pub struct HttpError {
    pub status: u16,
    pub body: String,
    pub url: Option<Url>,
}

impl From<ReqwestError> for HttpError {
    fn from(error: ReqwestError) -> Self {
        HttpError {
            status: error
                .status()
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                .as_u16(),
            url: error.url().cloned(),
            body: error.to_string(),
        }
    }
}

pub enum ApiError {
    Http(HttpError),
    Decode(String),
    Unexpected(String),
}

impl From<HttpError> for ApiError {
    fn from(error: HttpError) -> Self {
        ApiError::Http(error)
    }
}

impl From<String> for ApiError {
    fn from(error: String) -> Self {
        ApiError::Unexpected(error)
    }
}

impl From<ReqwestError> for ApiError {
    fn from(error: ReqwestError) -> Self {
        // Only convert to HttpError if there's an actual HTTP status code
        if let Some(_status) = error.status() {
            // This is a real HTTP error (4xx, 5xx)
            ApiError::Http(HttpError::from(error))
        } else if error.is_decode() {
            // Response body decode error (not JSON deserialization)
            ApiError::Decode(format!("Response decoding error: {:?}", error))
        } else if error.is_timeout() {
            ApiError::Unexpected("Request timeout".to_string())
        } else if error.is_connect() {
            ApiError::Unexpected("Connection error".to_string())
        } else {
            // Other errors (TLS, request building, redirect loops, etc.)
            ApiError::Unexpected("Request error".to_string())
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::Decode(format!("JSON decode error: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_error_string_form() {
        let http_error = HttpError {
            status: 400,
            body: "Test".to_string(),
            url: Some(Url::parse("https://brianobot.github.io").unwrap()),
        };

        let string_form = http_error.to_string();
        assert_eq!(
            string_form,
            format!(
                "{:?} ({}, {})",
                &http_error.url.unwrap().path(),
                &http_error.status,
                &http_error.body
            )
        )
    }
}
