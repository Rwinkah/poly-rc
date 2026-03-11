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
    pub clob_token_ids: String,
}

impl Market {
    pub fn clob_token_ids(&self) -> (String, String) {
        let clean = self
            .clob_token_ids
            .trim_matches(|c| c == '[' || c == ']' || c == '"');

        // 2. Split by the delimiter ", "
        let vec: Vec<String> = clean.split("\", \"").map(|s| s.to_string()).collect();

        (vec[0].clone(), vec[1].clone())
    }
}
