pub mod orderbook;
pub mod pricing;
pub mod sports;
pub mod spreads;

use orderbook::OrderBook;
use pricing::Pricing;

use crate::public::sports::Sports;
pub use crate::shared::{
    ApiError, HttpError, TokenId,
    client::AsyncHttpClient,
    constants::{CLOB_ENDPOINT, GAMMA_ENDPOINT},
};

pub struct PubClient {
    clob_client: AsyncHttpClient,
    gamma_client: AsyncHttpClient,
}

impl PubClient {
    pub fn new() -> Self {
        Self {
            clob_client: AsyncHttpClient::new(CLOB_ENDPOINT.to_string(), None),
            gamma_client: AsyncHttpClient::new(GAMMA_ENDPOINT.to_string(), None),
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

impl Sports for PubClient {
    fn get_gamma_client(&self) -> &AsyncHttpClient {
        &self.gamma_client
    }
}
