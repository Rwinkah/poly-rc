use crate::public::client::AsyncHttpClient;
use async_trait::async_trait;
pub mod models;
use crate::shared::{ApiError, QueryParams};
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
