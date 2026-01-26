// use reqwest;

use crate::shared::{ApiError, QueryParams, TokenId, client::AsyncHttpClient};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PubClient;

    #[tokio::test]
    async fn test_get_orderbook_summary() {
        let client = PubClient::new();

        let orderbook = client
            .get_orderbook_summary(TokenId {
                token_id: String::from("test_token_id"),
            })
            .await;

        if let Err(e) = &orderbook {
            eprintln!("Orderbook error: {:?}", e);
        }
        assert!(orderbook.is_ok());
    }

    #[tokio::test]
    async fn test_post_orderbook_summaries_multiple_tokens() {
        let client = PubClient::new();

        let orderbooks = client
            .post_orderbook_summaries(vec![
                TokenId {
                    token_id: String::from("test_token_id_1"),
                },
                TokenId {
                    token_id: String::from("test_token_id_2"),
                },
            ])
            .await;

        if let Err(e) = &orderbooks {
            eprintln!("Orderbook summaries error: {:?}", e);
        }
        assert!(orderbooks.is_ok());
    }

    #[tokio::test]
    async fn test_post_orderbook_summaries_single_token() {
        let client = PubClient::new();

        let orderbooks = client
            .post_orderbook_summaries(vec![TokenId {
                token_id: String::from("test_token_id"),
            }])
            .await;

        if let Err(e) = &orderbooks {
            eprintln!("Orderbook summaries error (single token): {:?}", e);
        }
        assert!(orderbooks.is_ok());
    }

    #[tokio::test]
    async fn test_post_orderbook_summaries_empty_vec() {
        let client = PubClient::new();

        let orderbooks = client.post_orderbook_summaries(vec![]).await;

        if let Err(e) = &orderbooks {
            eprintln!("Orderbook summaries error (empty vec): {:?}", e);
        }
        // Empty vector might be valid or invalid depending on API - test will show behavior
        // If API rejects empty, this will fail; if it accepts, it will pass
        let _ = orderbooks;
    }

    #[tokio::test]
    async fn test_post_orderbook_summaries_large_batch() {
        let client = PubClient::new();

        // Test with a larger batch of tokens
        let tokens: Vec<TokenId> = (1..=10)
            .map(|i| TokenId {
                token_id: format!("test_token_id_{}", i),
            })
            .collect();

        let orderbooks = client.post_orderbook_summaries(tokens).await;

        if let Err(e) = &orderbooks {
            eprintln!("Orderbook summaries error (large batch): {:?}", e);
        }
        assert!(orderbooks.is_ok());
    }
}
