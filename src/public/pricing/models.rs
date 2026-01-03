use std::fmt;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    BUY,
    SELL,
}

impl Side {
    pub fn as_str(&self) -> &str {
        match self {
            Side::BUY => "BUY",
            Side::SELL => "SELL",
        }
    }
}
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::BUY => write!(f, "BUY"),
            Side::SELL => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPriceDTO {
    pub token_id: String,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    pub price: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistoryDTO {
    pub market: String,
    pub start_ts: Option<u128>,
    pub end_ts: Option<u128>,
    pub interval: Option<PriceInterval>,
    pub fidelity: Option<u128>,
}
