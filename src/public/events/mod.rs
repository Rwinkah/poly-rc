pub mod models;

use async_trait::async_trait;
use models::{EventDTO, EventInfo, EventTag};

use crate::shared::{ApiError, QueryParams, client::AsyncHttpClient};

#[async_trait]
pub trait Events {
    fn get_gamma_client(&self) -> &AsyncHttpClient;

    async fn get_events(&self, data: EventDTO) -> Result<Vec<EventInfo>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client.get(Some("/events"), Some(query)).await?;
        let events: Vec<EventInfo> = response.json().await?;
        Ok(events)
    }

    async fn get_event(&self, id: String, data: EventDTO) -> Result<EventInfo, ApiError> {
        let path = format!("{}{}", "/events/", id);
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client.get(Some(path.as_str()), Some(query)).await?;
        let event: EventInfo = response.json().await?;
        Ok(event)
    }

    async fn get_event_tags(&self, id: String) -> Result<Vec<EventTag>, ApiError> {
        let path = format!("{}{}/tags", "/events/", id);
        println!("🚀 {path}");
        let client = self.get_gamma_client();
        let response = client.get(Some(path.as_str()), None).await?;
        let event_tag: Vec<EventTag> = response.json().await?;
        Ok(event_tag)
    }

    async fn get_event_by_slug(&self, slug: String) -> Result<EventInfo, ApiError> {
        let path = format!("{}{}", "/events/slug/", slug);
        let client = self.get_gamma_client();
        let response = client.get(Some(path.as_str()), None).await?;
        let event: EventInfo = response.json().await?;
        Ok(event)
    }
}
