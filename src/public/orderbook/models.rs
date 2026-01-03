use crate::public::client::ToQueryParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single order (bid or ask)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: String,
    pub size: String,
}

/// Represents the orderbook summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookSummary {
    pub market: String,
    pub asset_id: String,
    pub timestamp: String,
    pub hash: String,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub min_order_size: String,
    pub tick_size: String,
    pub neg_risk: bool,
}

/// Represents a token identifier for orderbook queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenId {
    pub token_id: String,
}

/// Request DTO for orderbook queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookRequestDTO {
    pub token_ids: Vec<TokenId>,
}

impl ToQueryParams for TokenId {
    fn to_query_params(&self) -> HashMap<String, String> {
        HashMap::from([("token_id".to_string(), self.token_id.clone())])
    }
}
