use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::QueryParams;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarketDTO {}

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
    pub clob_token_ids: f64,
}
