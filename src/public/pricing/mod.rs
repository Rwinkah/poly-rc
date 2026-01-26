use crate::shared::{ApiError, QueryParams, TokenId, client::AsyncHttpClient};
use async_trait::async_trait;

pub mod models;
use models::{
    BidAskSpreads, MarketPrice, MarketPriceDTO, MarketPriceSet, MidpointPrice, PriceHistoryDTO,
    PricesHistory,
};

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
    ) -> Result<MarketPriceSet, ApiError> {
        let client = self.get_clob_client();
        let response = client.post(Some("/prices"), Some(data)).await?;
        let prices: MarketPriceSet = response.json().await?;
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
    async fn get_price_history(&self, data: PriceHistoryDTO) -> Result<PricesHistory, ApiError> {
        let client = self.get_clob_client();
        let query = data.as_query_params();
        let response = client.get(Some("/prices-history"), Some(query)).await?;
        let prices: PricesHistory = response.json().await?;
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
    use crate::shared::Side;
    use chrono::Utc;

    #[tokio::test]
    async fn test_get_market_price() {
        let client = PubClient::new();

        let market_price_1 = client
            .get_market_price(MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::BUY,
            })
            .await;
        let market_price_2 = client
            .get_market_price(MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::SELL,
            })
            .await;

        assert!(market_price_1.is_ok());
        assert!(market_price_2.is_ok());
    }

    #[tokio::test]
    async fn test_post_market_prices() {
        let client = PubClient::new();

        let data = vec![
            MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::BUY,
            },
            MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::SELL,
            },
        ];

        let market_prices = client.post_market_prices(data).await;

        dbg!(market_prices.as_ref().err());

        assert!(market_prices.is_ok());
    }

    #[tokio::test]
    async fn test_get_midpoint_price() {
        let client = PubClient::new();

        let data = TokenId {
            token_id:
                "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                    .to_string(),
        };

        let mid_point_price = client.get_midpoint_price(data).await;

        assert!(mid_point_price.is_ok());
    }

    #[tokio::test]
    async fn test_get_price_history() {
        let client = PubClient::new();

        let start_ts = Utc::now().timestamp();
        let data = PriceHistoryDTO {
            market: "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                .to_string(),
            start_ts: start_ts as u128,
            end_ts: (start_ts + 10) as u128,
            ..Default::default()
        };

        let price_history = client.get_price_history(data).await;
        dbg!(price_history.as_ref().err());
        assert!(price_history.is_ok());
    }

    #[tokio::test]
    async fn test_post_bid_ask_spreads() {
        let client = PubClient::new();

        let data = vec![
            MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::BUY,
            },
            MarketPriceDTO {
                token_id:
                    "85229865481166262443616698813899475047082678584551624516576861283095641108073"
                        .to_string(),
                side: Side::SELL,
            },
        ];

        let bid_ask_spread = client.post_bid_ask_spreads(data).await;
        assert!(bid_ask_spread.is_ok());
    }
}
