use crate::shared::{ApiError, QueryParams, client::AsyncHttpClient};
use async_trait::async_trait;
pub mod models;

use models::{TagDTO, TagIdDTO, TagItem, TagRelationshipDTO, TagRelationshipItem, TagSlugDTO};
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

    /// Get the tag by id
    /// # Arguments
    /// * `id` - The id of the tag
    /// * `data` - The query parameters for the tag. See [`TagIdDTO`] for available options.
    /// # Returns
    /// * `Result<[`TagItem`], [`ApiError`]>` - The [`TagItem`] object, or an [`ApiError`] if the request fails
    async fn get_tag_by_id(&self, id: i32, data: TagIdDTO) -> Result<TagItem, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(Some(format!("/tags/{}", id).as_str()), Some(query))
            .await?;
        let tag: TagItem = response.json().await?;
        Ok(tag)
    }

    /// Get the tag by slug
    /// # Arguments
    /// * `slug` - The slug of the tag
    /// * `data` - The query parameters for the tag. See [`TagSlugDTO`] for available options.
    /// # Returns
    /// * `Result<[`TagItem`], [`ApiError`]>` - The [`TagItem`] object, or an [`ApiError`] if the request fails
    async fn get_tag_by_slug(&self, slug: String, data: TagSlugDTO) -> Result<TagItem, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(Some(format!("/tags/slug/{}", slug).as_str()), Some(query))
            .await?;
        let tag: TagItem = response.json().await?;
        Ok(tag)
    }

    /// Get the relationships for a given tag id
    /// # Arguments
    /// * `id` - The id of the tag
    /// * `data` - The query parameters for the relationships. See [`TagRelationshipDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`TagRelationshipItem`]>, [`ApiError`]>` - A vector of [`TagRelationshipItem`] objects, or an [`ApiError`] if the request fails
    async fn get_tags_relationship(
        &self,
        id: i32,
        data: TagRelationshipDTO,
    ) -> Result<Vec<TagRelationshipItem>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(
                Some(format!("/tags/{}/related-tags", id).as_str()),
                Some(query),
            )
            .await?;
        let tags: Vec<TagRelationshipItem> = response.json().await?;
        Ok(tags)
    }

    /// Get the relationships for a given tag slug
    /// # Arguments
    /// * `slug` - The slug of the tag
    /// * `data` - The query parameters for the relationships. See [`TagRelationshipDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`TagRelationshipItem`]>, [`ApiError`]>` - A vector of [`TagRelationshipItem`] objects, or an [`ApiError`] if the request fails
    async fn get_related_tags_by_slug(
        &self,
        slug: String,
        data: TagRelationshipDTO,
    ) -> Result<Vec<TagRelationshipItem>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(
                Some(format!("/tags/slug/{}/related-tags", slug).as_str()),
                Some(query),
            )
            .await?;
        let tags: Vec<TagRelationshipItem> = response.json().await?;
        Ok(tags)
    }

    /// Get tags related to the provided tag (specified by id)
    /// # Arguments
    /// # `id` - The id of the tag
    /// # `data` - The query parameters for the relationships. See [`TagRelationshipDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`TagItem`]>, [`ApiError`]>` - A vector of [`TagItem`] objects, or an [`ApiError`] if the request fails
    async fn get_tags_related_to_id(
        &self,
        id: i32,
        data: TagRelationshipDTO,
    ) -> Result<Vec<TagItem>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(
                Some(format!("/tags/{}/related-tags/tags", id).as_str()),
                Some(query),
            )
            .await?;
        let tags: Vec<TagItem> = response.json().await?;
        Ok(tags)
    }

    /// Get tags related to the provided tag (specified by slug)
    /// # Arguments
    /// # `slug` - The slug of the tag
    /// # `data` - The query parameters for the relationships. See [`TagRelationshipDTO`] for available options.
    /// # Returns
    /// * `Result<Vec<[`TagItem`]>, [`ApiError`]>` - A vector of [`TagItem`] objects, or an [`ApiError`] if the request fails
    async fn get_tags_related_to_slug(
        &self,
        slug: String,
        data: TagRelationshipDTO,
    ) -> Result<Vec<TagItem>, ApiError> {
        let client = self.get_gamma_client();
        let query = data.as_query_params();
        let response = client
            .get(
                Some(format!("/tags/slug/{}/related-tags/tags", slug).as_str()),
                Some(query),
            )
            .await?;
        let tags: Vec<TagItem> = response.json().await?;
        Ok(tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PubClient;
    use models::TagStatus;

    #[tokio::test]
    async fn test_get_tags() {
        let client = PubClient::new();

        let tags_1 = client
            .get_tags(TagDTO {
                limit: None,
                offset: None,
                order: None,
                ascending: None,
                include_template: None,
                is_carousel: None,
            })
            .await;
        if let Err(e) = &tags_1 {
            eprintln!("Get tags error: {:?}", e);
        }
        let tags_2 = client
            .get_tags(TagDTO {
                limit: Some(10),
                offset: Some(0),
                order: None,
                ascending: Some(true),
                include_template: Some(true),
                is_carousel: None,
            })
            .await;

        assert!(tags_1.is_ok());
        assert!(tags_2.is_ok());
    }

    #[tokio::test]
    async fn test_get_tag_by_id() {
        let client = PubClient::new();

        let tag = client
            .get_tag_by_id(
                1,
                TagIdDTO {
                    include_template: Some(true),
                },
            )
            .await;
        if let Err(e) = &tag {
            eprintln!("Get tag by id error: {:?}", e);
        }

        assert!(tag.is_ok());
    }

    #[tokio::test]
    async fn test_get_tag_by_slug() {
        let client = PubClient::new();

        let tag = client
            .get_tag_by_slug(
                String::from("test-slug"),
                TagSlugDTO {
                    include_template: Some(true),
                },
            )
            .await;
        if let Err(e) = &tag {
            eprintln!("Get tag by slug error: {:?}", e);
        }

        assert!(tag.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_relationship() {
        let client = PubClient::new();

        let relationships = client
            .get_tags_relationship(
                1,
                TagRelationshipDTO {
                    omit_empty: Some(true),
                    status: Some(TagStatus::Active),
                },
            )
            .await;

        assert!(relationships.is_ok());
    }

    #[tokio::test]
    async fn test_get_related_tags_by_slug() {
        let client = PubClient::new();

        let relationships = client
            .get_related_tags_by_slug(
                String::from("test-slug"),
                TagRelationshipDTO {
                    omit_empty: Some(false),
                    status: Some(TagStatus::Closed),
                },
            )
            .await;

        assert!(relationships.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_related_to_id() {
        let client = PubClient::new();

        let tags = client
            .get_tags_related_to_id(
                1,
                TagRelationshipDTO {
                    omit_empty: None,
                    status: Some(TagStatus::All),
                },
            )
            .await;
        if let Err(e) = &tags {
            eprintln!("Get tags related to id error: {:?}", e);
        }

        assert!(tags.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_related_to_slug() {
        let client = PubClient::new();

        let tags = client
            .get_tags_related_to_slug(
                String::from("test-slug"),
                TagRelationshipDTO {
                    omit_empty: Some(true),
                    status: None,
                },
            )
            .await;

        assert!(tags.is_ok());
    }
}
