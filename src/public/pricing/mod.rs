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
    use crate::shared::Side;
    use models::PriceInterval;

    #[tokio::test]
    async fn test_get_market_price() {
        let client = PubClient::new();

        let price_1 = client
            .get_market_price(MarketPriceDTO {
                token_id: String::from("test_token_id"),
                side: Side::BUY,
            })
            .await;
        if let Err(e) = &price_1 {
            eprintln!("Market price error: {:?}", e);
        }
        let price_2 = client
            .get_market_price(MarketPriceDTO {
                token_id: String::from("test_token_id"),
                side: Side::SELL,
            })
            .await;

        assert!(price_1.is_ok());
        assert!(price_2.is_ok());
    }

    #[tokio::test]
    async fn test_post_market_prices() {
        let client = PubClient::new();

        let prices = client
            .post_market_prices(vec![
                MarketPriceDTO {
                    token_id: String::from("test_token_id_1"),
                    side: Side::BUY,
                },
                MarketPriceDTO {
                    token_id: String::from("test_token_id_2"),
                    side: Side::SELL,
                },
            ])
            .await;
        if let Err(e) = &prices {
            eprintln!("Post market prices error: {:?}", e);
        }

        assert!(prices.is_ok());
    }

    #[tokio::test]
    async fn test_get_midpoint_price() {
        let client = PubClient::new();

        let price = client
            .get_midpoint_price(TokenId {
                token_id: String::from("test_token_id"),
            })
            .await;
        if let Err(e) = &price {
            eprintln!("Midpoint price error: {:?}", e);
        }

        assert!(price.is_ok());
    }

    #[tokio::test]
    async fn test_get_price_history() {
        let client = PubClient::new();

        let history_1 = client
            .get_price_history(PriceHistoryDTO {
                market: String::from("test_market"),
                start_ts: None,
                end_ts: None,
                interval: None,
                fidelity: None,
            })
            .await;
        if let Err(e) = &history_1 {
            eprintln!("Price history error: {:?}", e);
        }
        let history_2 = client
            .get_price_history(PriceHistoryDTO {
                market: String::from("test_market"),
                start_ts: Some(1000000),
                end_ts: Some(2000000),
                interval: Some(PriceInterval::Hour1),
                fidelity: Some(1000),
            })
            .await;

        assert!(history_1.is_ok());
        assert!(history_2.is_ok());
    }

    #[tokio::test]
    async fn test_post_bid_ask_spreads() {
        let client = PubClient::new();

        let spreads = client
            .post_bid_ask_spreads(vec![
                MarketPriceDTO {
                    token_id: String::from("test_token_id_1"),
                    side: Side::BUY,
                },
                MarketPriceDTO {
                    token_id: String::from("test_token_id_2"),
                    side: Side::SELL,
                },
            ])
            .await;

        assert!(spreads.is_ok());
    }
}
