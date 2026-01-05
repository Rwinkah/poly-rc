use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::QueryParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportsTeamsDTO {
    pub limit: Option<i16>,
    pub offset: Option<i16>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub league: Option<Vec<String>>,
    pub name: Option<Vec<String>>,
    pub abbreviation: Option<Vec<String>>,
}

impl QueryParams for SportsTeamsDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit {
            query.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = self.offset {
            query.insert("offset".to_string(), offset.to_string());
        }
        if let Some(order) = &self.order {
            query.insert("order".to_string(), order.to_string());
        }
        if let Some(ascending) = self.ascending {
            query.insert("ascending".to_string(), ascending.to_string());
        }

        if let Some(league) = &self.league {
            for leag in league {
                query.insert("league".to_string(), leag.to_string());
            }
        }
        if let Some(name) = &self.name {
            for name in name {
                query.insert("name".to_string(), name.to_string());
            }
        }
        if let Some(abbreviation) = &self.abbreviation {
            for abbreviation in abbreviation {
                query.insert("abbreviation".to_string(), abbreviation.to_string());
            }
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportsTeam {
    pub id: i32,
    pub name: Option<String>,
    pub league: Option<String>,
    pub record: Option<String>,
    pub logo: Option<String>,
    pub abbreviation: Option<String>,
    pub alias: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportsMetadata {
    pub sport: String,
    pub image: String,
    pub resolution: String,
    pub ordering: String,
    pub tags: String,
    pub series: String,
}
