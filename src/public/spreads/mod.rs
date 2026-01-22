use async_trait::async_trait;
pub mod models;
use crate::shared::{ApiError, QueryParams, client::AsyncHttpClient};
use models::*;
#[async_trait]
pub trait Spreads {
    fn get_clob_client(&self) -> &AsyncHttpClient;

    /// Get the spread for a given token id and side
    /// # Arguments
    /// * `data` - The token id and side to get the spread for
    /// # Returns
    /// * `Result<Spread, ApiError>` - The spread for the given token id and side
    async fn get_spread(&self, data: SpreadBidAskDTO) -> Result<Spread, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/spread"), Some(query)).await?;
        let spread: Spread = response.json().await?;
        Ok(spread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PubClient;
    use crate::shared::Side;

    #[tokio::test]
    async fn test_get_spread() {
        let client = PubClient::new();

        let spread_1 = client
            .get_spread(SpreadBidAskDTO {
                token_id: String::from("test_token_id"),
                side: Some(Side::BUY),
            })
            .await;
        if let Err(e) = &spread_1 {
            eprintln!("Get spread error: {:?}", e);
        }
        let spread_2 = client
            .get_spread(SpreadBidAskDTO {
                token_id: String::from("test_token_id"),
                side: Some(Side::SELL),
            })
            .await;
        let spread_3 = client
            .get_spread(SpreadBidAskDTO {
                token_id: String::from("test_token_id"),
                side: None,
            })
            .await;

        assert!(spread_1.is_ok());
        assert!(spread_2.is_ok());
        assert!(spread_3.is_ok());
    }
}
