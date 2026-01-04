use reqwest::{Client as ReqwestClient, Error, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// An asynchronous HTTP client for making requests to the API
pub struct AsyncHttpClient {
    client: ReqwestClient,
    base_url: String,
    _auto_retry: bool,
}

/// Represents an error from the API
/// # Variants
/// * `Http(HttpError)` - An HTTP error
/// * `Decode(serde_json::Error)` - A JSON decoding error
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

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
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

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
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

impl AsyncHttpClient {
    pub fn new(base_url: String, auto_retry: Option<bool>) -> Self {
        let client = ReqwestClient::new();
        Self {
            client,
            base_url,
            _auto_retry: auto_retry.unwrap_or(false),
        }
    }

    /// Send a GET request to the API
    pub async fn get(
        &self,
        path: Option<&str>,
        query: Option<HashMap<String, String>>,
    ) -> Result<Response, ApiError> {
        let url = if let Some(p) = path {
            format!("{}{}", self.base_url, p)
        } else {
            self.base_url.clone()
        };

        let mut request = self.client.get(&url);

        if let Some(query_params) = query {
            // Convert HashMap<String, String> to HashMap<&str, &str> for reqwest
            let query_refs: HashMap<&str, &str> = query_params
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            request = request.query(&query_refs);
        }

        let response = request.send().await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let url = response.url().clone();
            let error_body = response.text().await?;
            return Err(ApiError::Http(HttpError {
                status: status.as_u16(),
                url: Some(url),
                body: error_body,
            }));
        }

        Ok(response)
    }

    /// Send a POST request to the API
    pub async fn post<T: Serialize>(
        &self,
        path: Option<&str>,
        body: Option<T>,
    ) -> Result<Response, ApiError> {
        let url = format!("{}{}", self.base_url, path.unwrap_or(""));
        let mut request = self.client.post(&url);

        if let Some(body_params) = body {
            request = request.json(&body_params);
        }

        let response = request.send().await?;
        Ok(response.error_for_status()?)
    }

    /// Send a PUT request to the API
    pub async fn put<T: Serialize>(
        &self,
        path: Option<&str>,
        body: Option<T>,
    ) -> Result<Response, ApiError> {
        let url = format!("{}{}", self.base_url, path.unwrap_or(""));
        let mut request = self.client.put(&url);

        if let Some(body_params) = body {
            request = request.json(&body_params);
        }

        let response = request.send().await?;
        Ok(response.error_for_status()?)
    }
}

/// Trait for converting DTOs to query parameters
pub trait ToQueryParams {
    fn to_query_params(&self) -> HashMap<String, String>;
}

pub trait Retryable {
    fn is_retryable(&self) -> bool;
    fn retry_delay(&self) -> Option<Duration>;
}

impl Retryable for StatusCode {
    fn is_retryable(&self) -> bool {
        match self {
            &StatusCode::TOO_MANY_REQUESTS => true,
            &StatusCode::INTERNAL_SERVER_ERROR => true,
            &StatusCode::BAD_GATEWAY => true,
            &StatusCode::SERVICE_UNAVAILABLE => true,
            &StatusCode::GATEWAY_TIMEOUT => true,
            _ => false,
        }
    }
    fn retry_delay(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
}

impl Retryable for Error {
    fn is_retryable(&self) -> bool {
        self.is_timeout() || self.is_connect()
    }

    fn retry_delay(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
}
