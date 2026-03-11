use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::QueryParams;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarketDTO {
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub limit: Option<u32>,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
}

impl QueryParams for MarketDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    pub slug: String,
    pub question: String,
    pub best_bid: f64,
    pub best_ask: f64,
}
