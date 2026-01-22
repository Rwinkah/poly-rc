use crate::shared::{ApiError, QueryParams, TokenId, client::AsyncHttpClient};
use async_trait::async_trait;
pub mod models;
use models::{BidAskSpreads, MarketPrice, MarketPriceDTO, MidpointPrice, PriceHistoryDTO};

#[async_trait]
pub trait Pricing {
    fn get_clob_client(&self) -> &AsyncHttpClient;

    /// Get the market price for a given token id and side
    /// # Arguments
    /// * `data` - The token id and side to get the market price for
    /// # Returns
    /// * `Result<MarketPrice, HttpError>` - The market price for the given token id and side
    async fn get_market_price(&self, data: MarketPriceDTO) -> Result<MarketPrice, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/price"), Some(query)).await?;
        let price: MarketPrice = response.json().await?;
        Ok(price)
    }

    /// Post the market prices for a given list of token ids and sides
    /// # Arguments
    /// * `data` - The list of token ids and sides to post the market prices for
    /// # Returns
    /// * `Result<Vec<MarketPrice>, ApiError>` - The market prices for the given list of token ids and sides
    async fn post_market_prices(
        &self,
        data: Vec<MarketPriceDTO>,
    ) -> Result<Vec<MarketPrice>, ApiError> {
        let client = self.get_clob_client();
        let response = client.post(Some("/prices"), Some(data)).await?;
        let prices: Vec<MarketPrice> = response.json().await?;
        Ok(prices)
    }

    /// Get the midpoint price for a given token id
    /// # Arguments
    /// * `data` - The token id to get the midpoint price for
    /// # Returns
    /// * `Result<MarketPrice, ApiError>` - The midpoint price for the given token id
    async fn get_midpoint_price(&self, data: TokenId) -> Result<MidpointPrice, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/midpoint"), Some(query)).await?;
        let price: MidpointPrice = response.json().await?;
        Ok(price)
    }

    /// Get the price history for a given market
    /// # Arguments
    /// * `data` - The price history query parameters
    /// # Returns
    /// * `Result<Vec<MarketPrice>, ApiError>` - The price history for the given market
    async fn get_price_history(&self, data: PriceHistoryDTO) -> Result<Vec<MarketPrice>, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/price/history"), Some(query)).await?;
        let prices: Vec<MarketPrice> = response.json().await?;
        Ok(prices)
    }

    /// Get bid-ask spreads for a list of token IDs and sides
    /// # Arguments
    /// * `data` - The list of token ids and sides to get spreads for
    /// # Returns
    /// * `Result<BidAskSpreads, ApiError>` - A map of token IDs to their spread values
    async fn post_bid_ask_spreads(
        &self,
        data: Vec<MarketPriceDTO>,
    ) -> Result<BidAskSpreads, ApiError> {
        let client = self.get_clob_client();
        let response = client.post(Some("/spreads"), Some(data)).await?;
        let spreads: BidAskSpreads = response.json().await?;
        Ok(spreads)
    }

    // async fn get_prices_history(&self, data: )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PubClient;

    #[tokio::test]
    async fn test_get_market_price() {
        let client = PubClient::new();

        let market_price_1 = client
            .get_market_price(MarketPriceDTO {
                token_id: (),
                side: (),
            })
            .await;
    }
}
