use std::{collections::HashMap, fmt};

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
            let order_len = order.len();
            for (i, order_item) in order.iter().enumerate() {
                order_str.push_str(order_item);
                if i < order_len - 1 {
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
    pub label: Option<String>,
    pub slug: Option<String>,
    pub force_show: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagIdDTO {
    pub include_template: Option<bool>,
}

impl QueryParams for TagIdDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(include_template) = self.include_template {
            query.insert("include_template".to_string(), include_template.to_string());
        }
        query
    }
}

pub type TagSlugDTO = TagIdDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagStatus {
    Active,
    Closed,
    All,
}

impl fmt::Display for TagStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TagStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TagStatus::Active => "active",
            TagStatus::Closed => "closed",
            TagStatus::All => "all",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRelationshipDTO {
    pub omit_empty: Option<bool>,
    pub status: Option<TagStatus>,
}

impl QueryParams for TagRelationshipDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(omit_empty) = self.omit_empty {
            query.insert("omit_empty".to_string(), omit_empty.to_string());
        }
        if let Some(status) = &self.status {
            query.insert("status".to_string(), status.to_string());
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRelationshipItem {
    pub id: String,
    pub tag_id: Option<i32>,
    pub related_tag_id: Option<i32>,
    pub rank: Option<i32>,
}
