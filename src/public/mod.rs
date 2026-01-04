pub mod orderbook;
pub mod pricing;
pub mod sports;
pub mod spreads;

use orderbook::OrderBook;
use pricing::Pricing;

pub use crate::shared::{ApiError, HttpError, TokenId, client::AsyncHttpClient};

pub struct PubClient {
    clob_client: AsyncHttpClient,
}

impl PubClient {
    pub fn new() -> Self {
        Self {
            clob_client: AsyncHttpClient::new("https://clob.polymarket.com".to_string(), None),
        }
    }
}

impl OrderBook for PubClient {
    fn get_clob_client(&self) -> &AsyncHttpClient {
        &self.clob_client
    }
}

impl Pricing for PubClient {
    fn get_clob_client(&self) -> &AsyncHttpClient {
        &self.clob_client
    }
}
