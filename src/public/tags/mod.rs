use crate::shared::{ApiError, QueryParams, client::AsyncHttpClient};
use async_trait::async_trait;
pub mod models;

use models::{TagDTO, TagItem};
#[async_trait]
pub trait Tags {
    fn get_gamma_client(&self) -> &AsyncHttpClient;

    /// Get the tags
    /// # Arguments
    /// * `data` - The query parameters for the tags. See [`TagDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`TagItem`]>, [`ApiError`]>` - A vector of [`TagItem`] objects, or an [`ApiError`] if the request fails
    async fn get_tags(&self, data: TagDTO) -> Result<Vec<TagItem>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client.get(Some("/tags"), Some(query)).await?;
        let tags: Vec<TagItem> = response.json().await?;
        Ok(tags)
    }
}
