//! This module defines models and headers used for the ClobClient.
//!
//! # Types
//! - `L1Headers`: Headers used for generating L2 credentials. Required for authentication.
//! - `L2Headers`: Headers generated from L2 credentials, used for authenticated requests.
//! - `L2Credentials`: Holds API key, secret, and passphrase for L2 authentication.
//! - `ClobClientArgs`: Arguments needed to construct a `ClobClient`.
//! - `ClobAuth`: EIP-712 typed message for signing.

use crate::public::ApiError;
use alloy::primitives::Address;
use alloy::sol;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// L1 authentication headers for generating L2 Auth credentials requests.
///
/// These headers are used to generate L2 credentials, which are required for
/// performing authenticated API requests.
///
/// Internally this is a thin wrapper around [`HeaderMap`] to enforce
/// type safety and ensure the required authentication headers are present.
#[derive(Clone)]
pub struct L1Headers(HeaderMap);

/// L2 authentication headers
///
/// These headers must be present in any authenticated request to the clob api.
///
/// They are generated using the [`L2Credentials`] returned from the
/// authentication flow.
#[derive(Clone)]
pub struct L2Headers(HeaderMap);

impl L1Headers {
    /// Creates a new `L1Headers` instance.
    ///
    /// # Parameters
    /// - `address`: The wallet address for authentication.
    /// - `signature`: `EIP712` signature string generated for the request.
    /// - `timestamp`: Server timestamp.
    /// - `nonce`: Optional nonce. Defaults to `0` if not provided.
    ///
    /// # Returns
    /// `Ok(L1Headers)` if all header values are valid.
    ///
    /// # Errors
    /// Returns an [`ApiError`] if any header value fails to convert into a valid
    /// HTTP header representation.
    pub fn new(
        address: Address,
        signature: String,
        timestamp: u64,
        nonce: Option<u64>,
    ) -> Result<Self, ApiError> {
        let valid_nonce = nonce.unwrap_or_else(|| 0);
        let mut headers = HeaderMap::new();
        headers.insert(
            "POLY_ADDRESS",
            HeaderValue::from_str(address.to_string().as_str())?,
        );
        headers.insert("POLY_SIGNATURE", HeaderValue::from_str(signature.as_str())?);
        headers.insert(
            "POLY_TIMESTAMP",
            HeaderValue::from_str(timestamp.to_string().as_str())?,
        );
        headers.insert(
            "POLY_NONCE",
            HeaderValue::from_str(valid_nonce.to_string().as_str())?,
        );

        Ok(Self(headers))
    }
}

/// Allows transparent access to the underlying [`HeaderMap`].
///
/// This enables `L1Headers` to be used anywhere a `HeaderMap` reference
/// is expected.
impl Deref for L1Headers {
    type Target = HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Consumes `L1Headers` and returns the underlying [`HeaderMap`].
///
/// This is useful when passing headers into HTTP clients such as `reqwest`.
impl From<L1Headers> for HeaderMap {
    fn from(headers: L1Headers) -> Self {
        headers.0
    }
}

impl L2Headers {
    /// Creates a new `L2Header` instance
    ///
    /// # Parameters
    /// - `address`: The wallet address for authentication
    /// - `signature`: `Hmac` signature generated for the request
    /// - `timestamp`: Server timestamp
    /// - `passphrase`: Passphrase from [`L2Credentials`]
    ///
    /// # Errors
    /// Returns an [`ApiError`] if any value cannot be converted into a valid
    /// HTTP header.
    pub fn new(
        address: Address,
        signature: &str,
        timestamp: u64,
        passphrase: &str,
    ) -> Result<Self, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert("POLY_ADDRESS", HeaderValue::from_str(&address.to_string())?);
        headers.insert("POLY_SIGNATURE", HeaderValue::from_str(signature)?);
        headers.insert(
            "POLY_TIMESTAMP",
            HeaderValue::from_str(&timestamp.to_string())?,
        );
        headers.insert("POLY_PASSPHRASE", HeaderValue::from_str(passphrase)?);

        Ok(Self(headers))
    }
}

/// Consumes `L2Headers` and returns the underlying [`HeaderMap`].
///
/// This allows easy integration with HTTP clients that expect
/// standard header maps.
impl From<L2Headers> for HeaderMap {
    fn from(headers: L2Headers) -> Self {
        headers.0
    }
}

/// Allows transparent access to the underlying [`HeaderMap`].
///
/// This enables `L2Headers` to behave like a normal header map
/// when interacting with HTTP clients.
impl Deref for L2Headers {
    type Target = HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// L2 authentication credentials.
///
/// These credentials are derived after successful L1 authentication
/// and are required for all authenticated API calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2Credentials {
    /// API key used to identify the authenticated client.
    ///
    /// The API may return this field as `apiKey`, therefore
    /// an alias is provided for deserialization compatibility.
    #[serde(alias = "apiKey")]
    pub api_key: String,

    /// Secret key used to generate HMAC signatures
    /// for authenticated requests.
    pub secret: String,

    /// Passphrase associated with the API key.
    ///
    /// This must be included in the `POLY_PASSPHRASE` header
    /// when generating authenticated requests.
    pub passphrase: String,
}

/// Arguments used to construct a [`ClobClient`].
///
/// These parameters configure how the client connects
/// to the CLOB API and performs authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobClientArgs {
    /// Base URL of the CLOB API endpoint.
    pub clob_url: String,

    /// Optional flag enabling automatic retry logic
    /// for failed HTTP requests.
    pub auto_retry: Option<bool>,

    /// Optional private key used for signing authentication
    /// messages during the L1 authentication flow.
    pub private_key: Option<String>,
}

sol! {
    struct ClobAuth {
        address address;
        string timestamp;
        uint256 nonce;
        string message;
    }
}
