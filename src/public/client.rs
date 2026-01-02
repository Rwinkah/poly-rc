use reqwest::{Client as ReqwestClient, Response, Error, StatusCode};
use std::collections::HashMap;
use std::time::Duration;
use serde::Serialize;


/// An asynchronous HTTP client for making requests to the API
pub struct AsyncHttpClient {
    client: ReqwestClient,
    base_url: String,
    _auto_retry: bool,
}




impl AsyncHttpClient {
    pub fn new(base_url: String, auto_retry: Option<bool>) -> Self {
        let client = ReqwestClient::new();
        Self { client, base_url, _auto_retry: auto_retry.unwrap_or(false) }
    }


    /// Send a GET request to the API
    pub async fn get(&self, path: Option<&str>, query: Option<HashMap<&str, &str>>) -> Result<Response, Error> {
       let  url = if let Some(p) = path {
        format!("{}{}", self.base_url, p)
       } else {
        self.base_url.clone()
       };
       
       let mut request = self.client.get(&url);

       if let Some(query_params) = query {
        request = request.query(&query_params);
       }

      let response = request.send().await?;
      response.error_for_status()
    
    }

    /// Send a POST request to the API
    pub async fn post<T: Serialize>(&self, path: Option<&str>, body: Option<T>) -> Result<Response, Error> {
        let url = format!("{}{}", self.base_url, path.unwrap_or(""));
        let mut request = self.client.post(&url);

        if let Some(body_params) = body {
            request = request.json(&body_params);
        }

        let response = request.send().await?;
        response.error_for_status()
    }

    /// Send a PUT request to the API
    pub async fn put<T: Serialize>(&self, path: Option<&str>, body: Option<T>) -> Result<Response, Error> {
        let url = format!("{}{}", self.base_url, path.unwrap_or(""));
        let mut request = self.client.put(&url);

        if let Some(body_params) = body {
            request = request.json(&body_params);
        }

        let response = request.send().await?;
        response.error_for_status()
    }
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

