use serde::{Deserialize, Serialize};

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
