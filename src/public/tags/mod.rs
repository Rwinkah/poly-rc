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

        let tags = client
            .get_tags(TagDTO {
                limit: None,
                offset: None,
                order: None,
                ascending: None,
                include_template: None,
                is_carousel: None,
            })
            .await;

        if let Err(e) = &tags {
            eprintln!("Get tags error: {:?}", e);
        }
        assert!(tags.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_with_params() {
        let client = PubClient::new();

        let tags = client
            .get_tags(TagDTO {
                limit: Some(10),
                offset: Some(0),
                order: Some(vec![String::from("name"), String::from("id")]),
                ascending: Some(true),
                include_template: Some(true),
                is_carousel: Some(false),
            })
            .await;

        if let Err(e) = &tags {
            eprintln!("Get tags with params error: {:?}", e);
        }
        assert!(tags.is_ok());
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
    async fn test_get_tag_by_id_without_template() {
        let client = PubClient::new();

        let tag = client
            .get_tag_by_id(
                1,
                TagIdDTO {
                    include_template: None,
                },
            )
            .await;

        if let Err(e) = &tag {
            eprintln!("Get tag by id without template error: {:?}", e);
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

        if let Err(e) = &relationships {
            eprintln!("Get tags relationship error: {:?}", e);
        }
        assert!(relationships.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_relationship_all_status() {
        let client = PubClient::new();

        let relationships = client
            .get_tags_relationship(
                1,
                TagRelationshipDTO {
                    omit_empty: Some(false),
                    status: Some(TagStatus::All),
                },
            )
            .await;

        if let Err(e) = &relationships {
            eprintln!("Get tags relationship all status error: {:?}", e);
        }
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

        if let Err(e) = &relationships {
            eprintln!("Get related tags by slug error: {:?}", e);
        }
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

        if let Err(e) = &tags {
            eprintln!("Get tags related to slug error: {:?}", e);
        }
        assert!(tags.is_ok());
    }

    // Unit tests for query parameter generation
    #[test]
    fn test_tag_dto_query_params() {
        let dto = TagDTO {
            limit: Some(10),
            offset: Some(5),
            order: Some(vec![String::from("name"), String::from("id")]),
            ascending: Some(true),
            include_template: Some(true),
            is_carousel: Some(false),
        };

        let params = dto.as_query_params();

        assert_eq!(params.get("limit"), Some(&"10".to_string()));
        assert_eq!(params.get("offset"), Some(&"5".to_string()));
        assert_eq!(params.get("order"), Some(&"name,id".to_string()));
        assert_eq!(params.get("ascending"), Some(&"true".to_string()));
        assert_eq!(params.get("include_template"), Some(&"true".to_string()));
        assert_eq!(params.get("is_carousel"), Some(&"false".to_string()));
    }

    #[test]
    fn test_tag_dto_query_params_empty() {
        let dto = TagDTO {
            limit: None,
            offset: None,
            order: None,
            ascending: None,
            include_template: None,
            is_carousel: None,
        };

        let params = dto.as_query_params();

        assert!(params.is_empty());
    }

    #[test]
    fn test_tag_dto_query_params_single_order() {
        let dto = TagDTO {
            limit: None,
            offset: None,
            order: Some(vec![String::from("name")]),
            ascending: None,
            include_template: None,
            is_carousel: None,
        };

        let params = dto.as_query_params();

        assert_eq!(params.get("order"), Some(&"name".to_string()));
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_tag_id_dto_query_params() {
        let dto = TagIdDTO {
            include_template: Some(true),
        };

        let params = dto.as_query_params();

        assert_eq!(params.get("include_template"), Some(&"true".to_string()));
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_tag_id_dto_query_params_none() {
        let dto = TagIdDTO {
            include_template: None,
        };

        let params = dto.as_query_params();

        assert!(params.is_empty());
    }

    #[test]
    fn test_tag_relationship_dto_query_params() {
        let dto = TagRelationshipDTO {
            omit_empty: Some(true),
            status: Some(TagStatus::Active),
        };

        let params = dto.as_query_params();

        assert_eq!(params.get("omit_empty"), Some(&"true".to_string()));
        assert_eq!(params.get("status"), Some(&"active".to_string()));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_tag_relationship_dto_query_params_all_status() {
        let dto = TagRelationshipDTO {
            omit_empty: None,
            status: Some(TagStatus::All),
        };

        let params = dto.as_query_params();

        assert_eq!(params.get("status"), Some(&"all".to_string()));
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_tag_status_as_str() {
        assert_eq!(TagStatus::Active.as_str(), "active");
        assert_eq!(TagStatus::Closed.as_str(), "closed");
        assert_eq!(TagStatus::All.as_str(), "all");
    }

    #[test]
    fn test_tag_status_display() {
        assert_eq!(format!("{}", TagStatus::Active), "active");
        assert_eq!(format!("{}", TagStatus::Closed), "closed");
        assert_eq!(format!("{}", TagStatus::All), "all");
    }
}
