use crate::clob_client::ClobClient;
use crate::clob_client::orders::models::{OrderCreateDTO, OrderCreateResponse, OrderDetails};
use crate::public::{ApiError, AsyncHttpClient};
use alloy::signers::local::PrivateKeySigner;
use dotenv::dotenv;
use reqwest::Response;
use std::env;

pub mod models;

pub trait Orders {
    fn get_private_clob_client(&self) -> &ClobClient;

    #[allow(async_fn_in_trait)]
    async fn post_order(&self, body: OrderCreateDTO) -> Result<OrderCreateResponse, ApiError> {
        dotenv().ok();
        let private_client = self.get_private_clob_client();

        let l2_headers = private_client
            .generate_l2_headers("post", "/order", Some(""))
            .await?;
        let response: Response = private_client
            .client
            .post(Some("/order"), Some(body), None, Some(l2_headers.into()))
            .await?;
        Ok(response.json().await?)
    }
}
