use crate::shared::{Side, ToQueryParams};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadBidAskDTO {
    pub token_id: String,
    pub side: Option<Side>,
}

pub type Spread = HashMap<String, String>;

impl ToQueryParams for SpreadBidAskDTO {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        query.insert("token_id".to_string(), self.token_id.clone());
        if let Some(side) = &self.side {
            query.insert("side".to_string(), side.as_str().to_string());
        }
        query
    }
}
