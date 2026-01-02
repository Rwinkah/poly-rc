use reqwest;

use crate::public::client::{AsyncHttpClient, HttpError};
use std::collections::HashMap;
use std::sync::OnceLock;
pub mod models;
pub use models::{Order, OrderbookRequestDTO, OrderbookSummary, TokenId};

static CLIENT: OnceLock<AsyncHttpClient> = OnceLock::new();

fn get_client() -> &'static AsyncHttpClient {
    CLIENT.get_or_init(|| AsyncHttpClient::new("https://clob.polymarket.com".to_string(), None))
}

/// Get the orderbook summary for a given token id
/// # Arguments
/// * `data` - The token id to get the orderbook summary for
/// # Returns
/// * `Result<OrderbookSummary, reqwest::Error>` - The orderbook summary for the given token id
pub async fn get_orderbook_summary(data: TokenId) -> Result<OrderbookSummary, HttpError> {
    let client = get_client();
    let query = HashMap::from([("token_id", data.token_id.as_str())]);
    let response = client.get(Some("/book"), Some(query)).await?;
    let text = response.text().await?;
    let orderbook: OrderbookSummary = serde_json::from_str(&text).unwrap();
    Ok(orderbook)
}

/// Get the orderbook summaries for a given list of token ids
/// # Arguments
/// * `data` - The list of token ids to get the orderbook summaries for
/// # Returns
/// * `Result<OrderbookSummary, reqwest::Error>` - The orderbook summaries for the given list of token ids
pub async fn post_orderbook_summaries(
    data: OrderbookRequestDTO,
) -> Result<Vec<OrderbookSummary>, HttpError> {
    let client = get_client();
    let response = client.post(Some("/books"), Some(data.token_ids)).await?;
    let orderbook: Vec<OrderbookSummary> = response.json().await?;
    Ok(orderbook)
}
