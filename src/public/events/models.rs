use std::collections::HashMap;

use crate::shared::{QueryParams, Side};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum EventField {
    Active(Option<bool>),
    Closed(Option<bool>),
    Limit(Option<u32>),
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDTO {
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub limit: Option<u32>,
}

impl EventDTO {
    fn get(&self, field: &str) -> Option<EventField> {
        match field {
            "active" => Some(EventField::Active(self.active)),
            "closed" => Some(EventField::Closed(self.closed)),
            "limit" => Some(EventField::Limit(self.limit)),
            _ => None,
        }
    }
}

impl QueryParams for EventDTO {
    fn as_query_params(&self) -> HashMap<String, String> {
        let fields = ["active", "closed", "limit"];

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
pub struct EventInfo {}
