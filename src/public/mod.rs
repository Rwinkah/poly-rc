use crate::public::{
    client::AsyncHttpClient,
    orderbook::{OrderBook, models},
    pricing::Pricing,
};

pub mod client;
pub mod orderbook;
pub mod pricing;

pub use client::HttpError;
pub use models::{Order, OrderbookRequestDTO, OrderbookSummary, TokenId};
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
