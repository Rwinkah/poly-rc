use reqwest::{Error as ReqwestError, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// Trait for converting DTOs to query parameters
pub trait ToQueryParams {
    fn to_query_params(&self) -> HashMap<String, String>;
}

/// Represents a token identifier used across multiple API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenId {
    pub token_id: String,
}

impl ToQueryParams for TokenId {
    fn to_query_params(&self) -> HashMap<String, String> {
        HashMap::from([("token_id".to_string(), self.token_id.clone())])
    }
}

/// Represents an error from the API
/// # Variants
/// * `Http(HttpError)` - An HTTP error
/// * `Decode(String)` - A JSON decoding error
/// * `Unexpected(String)` - An unexpected error
#[derive(Debug, Serialize, Deserialize)]
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
        ApiError::Decode(format!("JSON decode error: {}", error))
    }
}

/// Represents an HTTP error
/// # Fields
/// * `status` - The HTTP status code
/// * `url` - The URL of the request
/// * `body` - The body of the response
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub status: u16,
    pub url: Option<Url>,
    pub body: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    BUY,
    SELL,
}

impl Side {
    pub fn as_str(&self) -> &str {
        match self {
            Side::BUY => "BUY",
            Side::SELL => "SELL",
        }
    }
}
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::BUY => write!(f, "BUY"),
            Side::SELL => write!(f, "SELL"),
        }
    }
}
