use crate::shared::{ApiError, client::AsyncHttpClient};
pub mod models;
use crate::shared::QueryParams;
use async_trait::async_trait;
use models::{SportsMetadata, SportsTeam, SportsTeamsDTO};

#[async_trait]
pub trait Sports {
    fn get_gamma_client(&self) -> &AsyncHttpClient;

    /// Get the sports teams available
    /// # Arguments
    /// * `data` - The query parameters for the sports teams. See [`SportsTeamsDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`SportsTeam`]>, [`ApiError`]>` - A vector of [`SportsTeam`] objects, or an [`ApiError`] if the request fails
    async fn get_sports_teams(&self, data: SportsTeamsDTO) -> Result<Vec<SportsTeam>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client.get(Some("/teams"), Some(query)).await?;
        let sports_data: Vec<SportsTeam> = response.json().await?;
        Ok(sports_data)
    }

    /// Get the sports metadata
    /// # Arguments
    /// * `data` - The query parameters for the sports metadata. See [SportsMetadataDTO] for available options.
    /// # Returns
    /// * `Result<SportsMetadata, ApiError>` - The sports [SportsMetadata] object, or an [ApiError] if the request fails
    async fn get_sports_metadata(&self) -> Result<Vec<SportsMetadata>, ApiError> {
        let client = self.get_gamma_client();
        let response = client.get(Some("/sports"), None).await?;
        let sports_metadata: Vec<SportsMetadata> = response.json().await?;
        Ok(sports_metadata)
    }

    /// Get the sports market types
    /// # Returns
    /// * `Result<Vec<String>, ApiError>` - A vector of sports market types, or an [ApiError] if the request fails
    async fn get_sports_market_types(&self) -> Result<Vec<String>, ApiError> {
        let client = self.get_gamma_client();
        let response = client.get(Some("/sports/market-types"), None).await?;
        let sports_market_types: Vec<String> = response.json().await?;
        Ok(sports_market_types)
    }
}
