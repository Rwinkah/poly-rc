// use reqwest;

use crate::public::client::AsyncHttpClient;
use crate::shared::{ApiError, QueryParams, TokenId};
pub mod models;
use async_trait::async_trait;
pub use models::{Order, OrderbookRequestDTO, OrderbookSummary};

#[async_trait]
pub trait OrderBook {
    fn get_clob_client(&self) -> &AsyncHttpClient;
    /// Get the orderbook summary for a given token id
    /// # Arguments
    /// * `data` - The token id to get the orderbook summary for
    /// # Returns
    /// * `Result<OrderbookSummary, ApiError>` - The orderbook summary for the given token id
    async fn get_orderbook_summary(&self, data: TokenId) -> Result<OrderbookSummary, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/book"), Some(query)).await?;
        let text = response.text().await?;
        let orderbook: OrderbookSummary = serde_json::from_str(&text).unwrap();
        Ok(orderbook)
    }

    /// Get the orderbook summaries for a given list of token ids
    /// # Arguments
    /// * `data` - The list of token ids to get the orderbook summaries for
    /// # Returns
    /// * `Result<Vec<OrderbookSummary>, ApiError>` - The orderbook summaries for the given list of token ids
    async fn post_orderbook_summaries(
        &self,
        data: Vec<TokenId>,
    ) -> Result<Vec<OrderbookSummary>, ApiError> {
        let client = self.get_clob_client();
        let response = client.post(Some("/books"), Some(data)).await?;
        let orderbook: Vec<OrderbookSummary> = response.json().await?;
        Ok(orderbook)
    }
}
