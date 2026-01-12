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
        let client = self.get_gamma_client();
        let response = client.get(Some(path.as_str()), None).await?;
        let event_tag: Vec<EventTag> = response.json().await?;
        Ok(event_tag)
    }

    async fn get_event_by_slug(&self, slug: String) -> Result<EventInfo, ApiError> {
        let path = format!("{}{}", "/events/slug/", slug);
        let client = self.get_gamma_client();
        let response = client.get(Some(path.as_str()), None).await?; //
        let event: EventInfo = response.json().await?; //
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PubClient;

    #[tokio::test]
    async fn test_get_events() {
        let client = PubClient::new();

        let events_1 = client
            .get_events(EventDTO {
                ..Default::default()
            })
            .await;
        let events_2 = client
            .get_events(EventDTO {
                active: Some(true),
                ..Default::default()
            })
            .await;
        let events_3 = client
            .get_events(EventDTO {
                closed: Some(true),
                ..Default::default()
            })
            .await;
        let events_4 = client
            .get_events(EventDTO {
                limit: Some(5),
                ..Default::default()
            })
            .await;
        let events_5 = client
            .get_events(EventDTO {
                include_chat: Some(true),
                ..Default::default()
            })
            .await;
        let events_6 = client
            .get_events(EventDTO {
                include_template: Some(true),
                ..Default::default()
            })
            .await;

        assert!(events_1.is_ok());
        assert!(events_2.is_ok());
        assert!(events_3.is_ok());
        assert!(events_4.is_ok());
        assert!(events_5.is_ok());
        assert!(events_6.is_ok());

        // Assert actual Struct values
    }

    #[tokio::test]
    async fn test_get_event() {
        let client = PubClient::new();

        let event_1 = client
            .get_event(
                String::from("2909"),
                EventDTO {
                    include_chat: Some(true),
                    ..Default::default()
                },
            )
            .await;
        let event_2 = client
            .get_event(
                String::from("2909"),
                EventDTO {
                    include_template: Some(true),
                    ..Default::default()
                },
            )
            .await;

        assert!(event_1.is_ok());
        assert!(event_2.is_ok());
    }

    #[tokio::test]
    async fn test_get_event_tags() {
        let client = PubClient::new();

        let event_tags = client.get_event_tags(String::from("2909"));

        assert!(event_tags.await.is_ok());
    }

    #[tokio::test]
    async fn test_get_event_by_slug() {
        let client = PubClient::new();

        let event = client.get_event_by_slug(String::from(
            "will-surojit-chatterjee-or-matt-huang-win-in-their-cryptochamps-finals-chess-match",
        ));

        assert!(event.await.is_ok());
    }
}
