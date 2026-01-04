pub mod models;

use crate::public::client::AsyncHttpClient;
use crate::shared::{ApiError, QueryParams, TokenId};
use async_trait::async_trait;
use models::{EventDTO, EventInfo};

#[async_trait]
pub trait Events {
    fn get_client(&self) -> &AsyncHttpClient;

    fn get_clob_client(&self) -> &AsyncHttpClient;

    async fn get_events(&self, data: EventDTO) -> Result<Vec<EventInfo>, ApiError> {
        let client = self.get_client();
        let query = data.as_query_params();
        let response = client.get(Some("/events"), Some(query)).await?;
        let events: Vec<EventInfo> = response.json().await?;
        Ok(events)
    }
}
