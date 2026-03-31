//! This module provides the `ClobClient`, an API client for interacting with
//! CLOB (Central Limit Order Book) service. It handles both L1 and L2 authentication,
//! including generating EIP-712 signatures, HMAC signatures, and managing
//! headers required for authenticated requests.
//!
//! # Overview
//!
//! - **L1 Authentication**: Used to derive L2 credentials. Involves signing
//!   an EIP-712 message with the wallet's private key.
//! - **L2 Authentication**: Uses API key, secret, and passphrase to generate
//!   HMAC signatures for authenticated requests.
//!
//! # Key Types
//!
//! - [`ClobClientArgs`]: Arguments needed to construct a `ClobClient`.
//! - [`ClobClient`]: Main client type with methods to interact with CLOB API.
//! - [`L1Headers`]: Headers required for generating L2 credentials.
//! - [`L2Credentials`]: API key, secret, and passphrase used for L2 authentication.
//! - [`L2Headers`]: Headers constructed from L2 credentials for authenticated requests.

pub mod config;
pub mod models;
pub mod orders;

use crate::clob_client::models::{ClobAuth, ClobClientArgs, L1Headers, L2Credentials, L2Headers};
use crate::public::AsyncHttpClient;
use crate::shared::ApiError;
use alloy::dyn_abi::Eip712Domain;
use alloy::primitives::{Address, U256};
use alloy::signers::{Signer, local::PrivateKeySigner};
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use reqwest::Response;
use reqwest::header::HeaderMap;
use sha2::Sha256;
use std::borrow::Cow;
use std::env;

/// Main client for interacting with the CLOB API.
///
/// Handles authentication, header generation, and provides access to
/// both L1 and L2 credentials. All network operations are performed
/// asynchronously.
pub struct ClobClient {
    client: AsyncHttpClient,
    l2_credentials: L2Credentials,
    signer: PrivateKeySigner,
    l1_headers: HeaderMap,
}

impl ClobClient {
    /// Create a new `ClobClient` with authentication credentials available.
    ///
    /// Performs the full authentication flow:
    /// 1. Generates a `PrivateKeySigner`.
    /// 2. Initializes the HTTP client.
    /// 3. Generates L1 headers.
    /// 4. Derives or creates L2 credentials.
    ///
    /// # Arguments
    /// - `args`: [`ClobClientArgs`] containing connection parameters and optional private key.
    ///
    /// # Returns
    /// A result containing a fully initialized `ClobClient` or an `ApiError`.
    pub async fn new(args: ClobClientArgs) -> Result<Self, ApiError> {
        let signer = Self::generate_signer(args.private_key);

        let client = AsyncHttpClient::new(args.clob_url, args.auto_retry);

        let l1_headers = Self::generate_l1_headers(&signer, &client).await?;

        // Generate L2 credentials and headers here if needed
        let l2_credentials = Self::generate_l2_credentials(&client, l1_headers.clone()).await?;

        Ok(Self {
            client,
            l2_credentials,
            signer,
            l1_headers,
        })
    }

    /// Generates a signer from an optional private key.
    ///
    /// If no key is provided, attempts to read the `PRIVATE_KEY` environment variable.
    ///
    /// # Panic condition
    /// Panics if no key is provided and `PRIVATE_KEY` is not set, or if the key
    /// cannot be parsed into a valid `PrivateKeySigner`.
    fn generate_signer(key: Option<String>) -> PrivateKeySigner {
        let private_key = key.unwrap_or_else(|| {
            env::var("PRIVATE_KEY")
                .expect("No private key provided and PRIVATE_KEY not found in environment")
        });

        private_key
            .parse::<PrivateKeySigner>()
            .expect("Invalid private key format")
    }

    /// Fetches the server time from the API.
    ///
    /// # Arguments
    /// - `client`: Reference to the HTTP client.
    ///
    /// # Returns
    /// A `u64` representing the server timestamp or an `ApiError`.
    async fn get_server_time(client: &AsyncHttpClient) -> Result<u64, ApiError> {
        let response: Response = client.get(Some("/time"), None, None).await?;
        let timestamp: u64 = response.json().await?;
        Ok(timestamp)
    }

    /// Generates L1 headers including EIP-712 signature.
    ///
    /// # Arguments
    /// - `signer`: Wallet signer.
    /// - `client`: HTTP client reference.
    ///
    /// # Returns
    /// A [`HeaderMap`] containing L1 authentication headers or an `ApiError`.
    async fn generate_l1_headers(
        signer: &PrivateKeySigner,
        client: &AsyncHttpClient,
    ) -> Result<HeaderMap, ApiError> {
        let (timestamp, signature) = Self::generate_li_signature(signer, client).await?;
        Ok(L1Headers::new(signer.address(), signature, timestamp, None)?.into())
    }

    /// Generates an EIP-712 typed data signature and returns `(timestamp, signature)`.
    async fn generate_li_signature(
        signer: &PrivateKeySigner,
        client: &AsyncHttpClient,
    ) -> Result<(u64, String), ApiError> {
        // Use static get_server_time instead of calling HTTP inline
        let timestamp = Self::get_server_time(client).await?;

        // construct EIP-712 domain
        let domain = Eip712Domain {
            name: Some(Cow::from("ClobAuthDomain")),
            version: Some(Cow::from("1")),
            chain_id: Some(U256::from(137)),
            verifying_contract: None,
            salt: None,
        };

        // construct typed message
        let message = ClobAuth {
            address: signer.address(),
            timestamp: timestamp.to_string(),
            nonce: U256::from(0),
            message: "This message attests that I control the given wallet".to_string(),
        };

        // sign the typed data
        let sig = signer.sign_typed_data(&message, &domain).await?;

        Ok((timestamp, sig.to_string()))
    }

    /// Returns a reference to the L1 headers.
    pub fn l1_headers(&self) -> &HeaderMap {
        &self.l1_headers
    }

    /// Generates or derives L2 credentials from the API.
    ///
    /// # Arguments
    /// - `client`: HTTP client reference.
    /// - `l1_headers`: L1 authentication headers.
    ///
    /// # Returns
    /// A [`L2Credentials`] instance or an `ApiError`.
    async fn generate_l2_credentials(
        client: &AsyncHttpClient,
        l1_headers: HeaderMap,
    ) -> Result<L2Credentials, ApiError> {
        // Try deriving credentials first
        let derive_credentials = client
            .get(Some("/auth/derive-api-key"), None, Some(l1_headers.clone()))
            .await;

        match derive_credentials {
            Ok(response) => {
                let text = response.text().await?;
                println!("RAW RESPONSE: {}", text);

                let credentials: L2Credentials = serde_json::from_str(&text)?;
                Ok(credentials)
            }

            Err(_) => {
                println!("Creating new credentials");

                let response = client
                    .post::<()>(Some("/auth/api-key"), None, None, Some(l1_headers))
                    .await?;

                let text = response.text().await?;
                println!("RAW RESPONSE: {}", text);

                let credentials: L2Credentials = serde_json::from_str(&text)?;
                Ok(credentials)
            }
        }
    }

    /// Returns a clone of the L2 credentials.
    pub fn l2_credentials(&self) -> L2Credentials {
        self.l2_credentials.clone()
    }

    /// Generates an HMAC signature for L2 authentication requests.
    ///
    /// # Arguments
    /// - `secret`: L2 secret key.
    /// - `method`: HTTP method (GET, POST, etc.).
    /// - `request_path`: API endpoint path.
    /// - `body`: Optional request body string.
    ///
    /// # Returns
    /// A tuple `(timestamp, signature)` where `signature` is URL-safe base64.
    async fn generate_l2_hmac_signature(
        &self,
        secret: &str,
        method: &str,
        request_path: &str,
        body: Option<&str>,
    ) -> Result<(u64, String), ApiError> {
        let timestamp = Self::get_server_time(&self.client).await?;

        let key_bytes = general_purpose::URL_SAFE.decode(secret)?;

        let mut message = format!("{}{}{}", timestamp, method, request_path);

        if let Some(body_str) = body {
            message.push_str(body_str);
        };

        let mut mac = Hmac::<Sha256>::new_from_slice(&key_bytes)?;
        mac.update(message.as_bytes());

        let result = mac.finalize();
        let bytes = result.into_bytes();

        let sig_base64 = general_purpose::STANDARD.encode(&bytes);

        let sig_url_safe = sig_base64.replace('+', "-").replace('/', "_");

        Ok((timestamp, sig_url_safe))
    }

    /// Generates L2 headers for an authenticated request.
    ///
    /// # Arguments
    /// - `method`: HTTP method (GET, POST, etc.).
    /// - `request_path`: API endpoint path.
    /// - `body`: Optional request body string.
    ///
    /// # Returns
    /// A [`L2Headers`] instance containing the necessary headers for the request.
    pub async fn generate_l2_headers(
        &self,
        method: &str,
        request_path: &str,
        body: Option<&str>,
    ) -> Result<L2Headers, ApiError> {
        let address: Address = self.signer.address();
        let (timestamp, signature) = self
            .generate_l2_hmac_signature(&self.l2_credentials.secret, method, request_path, body)
            .await?;

        Ok(L2Headers::new(
            address,
            &signature.as_str(),
            timestamp,
            &self.l2_credentials.passphrase,
            &self.l2_credentials.api_key,
        )?)
    }
}

struct Order {}
