use std::collections::HashMap;

use crate::shared::QueryParams;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum EventField {
    Active(Option<bool>),
    Closed(Option<bool>),
    Limit(Option<u32>),
    IncludeChat(Option<bool>),
    IncludeTemplate(Option<bool>),
}

impl EventField {
    fn as_string(&self) -> String {
        match self {
            Self::Active(value) => {
                let value = if value.is_none() {
                    String::from("")
                } else {
                    value.unwrap().to_string()
                };
                return value;
            }
            Self::Closed(value) => {
                let value = if value.is_none() {
                    String::from("")
                } else {
                    value.unwrap().to_string()
                };
                return value;
            }
            Self::Limit(value) => {
                let value = if value.is_none() {
                    String::from("")
                } else {
                    value.unwrap().to_string()
                };
                return value;
            }
            Self::IncludeChat(value) => {
                let value = if value.is_none() {
                    String::from("")
                } else {
                    value.unwrap().to_string()
                };
                return value;
            }
            Self::IncludeTemplate(value) => {
                let value = if value.is_none() {
                    String::from("")
                } else {
                    value.unwrap().to_string()
                };
                return value;
            }
        }
    }

    fn is_acceptable_query(&self) -> bool {
        match self {
            Self::Active(value) => {
                if value.is_none() {
                    return false;
                } else {
                    return true;
                }
            }
            Self::Closed(value) => {
                if value.is_none() {
                    return false;
                } else {
                    return true;
                }
            }
            Self::Limit(value) => {
                if value.is_none() {
                    return false;
                } else {
                    return true;
                }
            }
            Self::IncludeChat(value) => {
                if value.is_none() {
                    return false;
                } else {
                    return true;
                }
            }
            Self::IncludeTemplate(value) => {
                if value.is_none() {
                    return false;
                } else {
                    return true;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventDTO {
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub limit: Option<u32>,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
}

impl EventDTO {
    fn get(&self, field: &str) -> Option<EventField> {
        match field {
            "active" => Some(EventField::Active(self.active)),
            "closed" => Some(EventField::Closed(self.closed)),
            "limit" => Some(EventField::Limit(self.limit)),
            "include_chat" => Some(EventField::IncludeChat(self.include_chat)),
            "include_template" => Some(EventField::IncludeTemplate(self.include_template)),
            _ => None,
        }
    }
}

impl QueryParams for EventDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let fields = [
            "active",
            "closed",
            "limit",
            "include_chat",
            "include_template",
        ];

        let mut query_params = HashMap::new();
        for field in fields {
            let event_field = self.get(field);
            if event_field.is_none() {
                continue;
            }

            let event_field_instance = event_field.unwrap();
            // Only include the value in the query params if it's not None
            if event_field_instance.is_acceptable_query() {
                query_params.insert(field.to_owned(), event_field_instance.as_string());
            }
        }
        query_params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInfo {
    pub id: String,
    pub slug: String,
    pub active: bool,
    pub closed: bool,
    // pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTag {
    pub id: String,
    pub label: String,
    pub slug: String,
    pub force_show: bool,
    pub requires_translation: bool,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}
