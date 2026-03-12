pub mod models;

use async_trait::async_trait;
use models::{Market, MarketDTO};

use crate::shared::{ApiError, QueryParams, client::AsyncHttpClient};

#[async_trait]
pub trait Markets {
    fn get_gamma_client(&self) -> &AsyncHttpClient;

    async fn get_markets(&self, data: MarketDTO) -> Result<Vec<Market>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client.get(Some("/markets"), Some(query), None).await?;
        let events: Vec<Market> = response.json().await?;
        Ok(events)
    }

    async fn get_market_by_slug(&self, slug: String) -> Result<Market, ApiError> {
        let path = format!("{}{}", "/markets/slug/", slug);
        let client = self.get_gamma_client();
        let response = client.get(Some(path.as_str()), None, None).await?;
        let market: Market = response.json().await?;
        Ok(market)
    }
}
