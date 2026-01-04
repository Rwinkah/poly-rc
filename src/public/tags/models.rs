use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::QueryParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDTO {
    pub limit: Option<i16>,
    pub offset: Option<i16>,
    pub order: Option<Vec<String>>,
    pub ascending: Option<bool>,
    pub include_template: Option<bool>,
    pub is_carousel: Option<bool>,
}

impl QueryParams for TagDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit {
            query.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = self.offset {
            query.insert("offset".to_string(), offset.to_string());
        }
        if let Some(order) = &self.order {
            let mut order_str = String::new();
            for (i, order) in order.iter().enumerate() {
                order_str.push_str(order);
                if i < order.len() - 1 {
                    order_str.push(',');
                }
            }
            query.insert("order".to_string(), order_str);
        }
        if let Some(ascending) = self.ascending {
            query.insert("ascending".to_string(), ascending.to_string());
        }
        if let Some(include_template) = self.include_template {
            query.insert("include_template".to_string(), include_template.to_string());
        }
        if let Some(is_carousel) = self.is_carousel {
            query.insert("is_carousel".to_string(), is_carousel.to_string());
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagItem {
    pub id: String,
    pub label: String,
    pub slug: String,
    pub force_show: bool,
    pub published_at: String,
    pub created_by: i32,
    pub updated_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub force_hide: bool,
    pub is_carousel: bool,
}
