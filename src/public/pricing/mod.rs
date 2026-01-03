use crate::public::{
    TokenId,
    client::{AsyncHttpClient, HttpError},
};
use async_trait::async_trait;
pub mod models;
use serde_json::{Value, json};
use std::collections::HashMap;

use models::{MarketPrice, MarketPriceDTO, PriceHistoryDTO};

#[async_trait]
pub trait Pricing {
    fn get_clob_client(&self) -> &AsyncHttpClient;

    /// Get the market price for a given token id and side
    /// # Arguments
    /// * `data` - The token id and side to get the market price for
    /// # Returns
    /// * `Result<MarketPrice, HttpError>` - The market price for the given token id and side
    async fn get_market_price(&self, data: MarketPriceDTO) -> Result<MarketPrice, HttpError> {
        let client = self.get_clob_client();
        let mut query = HashMap::new();
        query.insert("token_id", data.token_id.as_str());
        query.insert("side", data.side.as_str());
        let response = client.get(Some("/price"), Some(query)).await?;
        let price: MarketPrice = response.json().await?;
        Ok(price)
    }

    /// Post the market prices for a given list of token ids and sides
    /// # Arguments
    /// * `data` - The list of token ids and sides to post the market prices for
    /// # Returns
    /// * `Result<Vec<MarketPrice>, HttpError>` - The market prices for the given list of token ids and sides
    async fn post_market_prices(
        &self,
        data: Vec<MarketPriceDTO>,
    ) -> Result<Vec<MarketPrice>, HttpError> {
        let client = self.get_clob_client();
        let response = client.post(Some("/prices"), Some(data)).await?;
        let prices: Vec<MarketPrice> = response.json().await?;
        Ok(prices)
    }

    /// Get the midpoint price for a given token id
    /// # Arguments
    /// * `data` - The token id to get the midpoint price for
    /// # Returns
    /// * `Result<MarketPrice, HttpError>` - The midpoint price for the given token id
    async fn get_midpoint_price(&self, data: TokenId) -> Result<MarketPrice, HttpError> {
        let client = self.get_clob_client();
        let query = HashMap::from([("token_id", data.token_id.as_str())]);
        let response = client.get(Some("/midpoint"), Some(query)).await?;
        let price: MarketPrice = response.json().await?;
        Ok(price)
    }

    async fn get_price_history(&self, data: PriceHistoryDTO) {
        let client = self.get_clob_client();
        let json_data = json!(data);
        let query = HashMap::from([("data", json_data.to_string().as_str())]);
    }
}
