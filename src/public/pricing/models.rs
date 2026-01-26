use std::collections::HashMap;

use crate::shared::{QueryParams, Side};
use serde::{Deserialize, Serialize, Serializer};

/// Represent a Set of Market Prices where each key is the token id
/// and their values are a hashmap of the buy and sell position at that moment
pub type MarketPriceSet = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPriceDTO {
    pub token_id: String,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricesHistory {
    pub history: Vec<HistoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub t: u128,
    pub p: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidpointPrice {
    pub mid: String,
}

/// Represents bid-ask spreads for multiple token IDs
/// Maps token ID (String) to spread value (String)
/// This is a type alias for clarity - the API returns a JSON object
pub type BidAskSpreads = HashMap<String, String>;

#[derive(Debug, Clone)]
pub enum PriceInterval {
    Minute1,
    Hour1,
    Hour6,
    Week1,
    Day1,
    Max,
}
impl PriceInterval {
    pub fn as_str(&self) -> &str {
        match self {
            PriceInterval::Minute1 => "1m",
            PriceInterval::Hour1 => "1h",
            PriceInterval::Hour6 => "6h",
            PriceInterval::Week1 => "1w",
            PriceInterval::Day1 => "1d",
            PriceInterval::Max => "max",
        }
    }
}

impl Serialize for PriceInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for PriceInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "1m" => Ok(PriceInterval::Minute1),
            "1h" => Ok(PriceInterval::Hour1),
            "6h" => Ok(PriceInterval::Hour6),
            "1w" => Ok(PriceInterval::Week1),
            "1d" => Ok(PriceInterval::Day1),
            "max" => Ok(PriceInterval::Max),
            _ => Err(serde::de::Error::custom("Invalid PriceInterval")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistoryDTO {
    pub market: String,
    pub start_ts: u128,
    pub end_ts: u128,
    pub interval: Option<PriceInterval>,
    pub fidelity: Option<u128>,
}

impl QueryParams for MarketPriceDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        HashMap::from([
            ("token_id".to_string(), self.token_id.clone()),
            ("side".to_string(), self.side.as_str().to_string()),
        ])
    }
}

impl QueryParams for PriceHistoryDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        params.insert("market".to_string(), self.market.clone());
        params.insert("startTs".to_string(), self.start_ts.to_string());
        params.insert("endTs".to_string(), self.end_ts.to_string());

        if let Some(interval) = &self.interval {
            params.insert("interval".to_string(), interval.as_str().to_string());
        }

        if let Some(fidelity) = self.fidelity {
            params.insert("fidelity".to_string(), fidelity.to_string());
        }

        params
    }
}
