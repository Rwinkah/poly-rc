use crate::clob_client::ClobClient;
use crate::clob_client::config::Chains;
use crate::clob_client::orders::models::{Order, OrderCreateDTO, OrderCreateResponse};
use crate::public::{ApiError, AsyncHttpClient};
use crate::shared::constants::{ORDER_NAME, ORDER_VERSION};
use alloy::dyn_abi::Eip712Domain;
use alloy::primitives::{Address, U256};
use alloy::signers::k256::Secp256k1;
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use dotenv::dotenv;
use reqwest::Response;
use std::env;

pub mod models;

pub trait Orders {
    fn get_private_clob_client(&self) -> ClobClient;

    async fn post_order(&self, body: OrderCreateDTO) -> Result<OrderCreateResponse, ApiError> {
        dotenv().ok();
        let private_client = self.get_private_clob_client();

        let str_body = serde_json::to_string(&body)?;

        let l2_headers = private_client
            .generate_l2_headers("POST", "/order", Some(str_body.as_str()))
            .await?;
        let response: Response = private_client
            .client
            .post(Some("/order"), Some(body), None, Some(l2_headers.into()))
            .await?;
        Ok(response.json().await?)
    }

    async fn build_order_domain(
        &self,
        signer: LocalSigner<SigningKey>,
        neg_risk: bool,
    ) -> Result<Eip712Domain, ApiError> {
        let exchange_contract: Address;

        match signer.chain_id() {
            Some(137) => {
                let config = Chains::Polygon.config();
                if neg_risk {
                    exchange_contract = config.neg_risk_config.exchange_contract
                } else {
                    exchange_contract = config.standard_config.exchange_contract
                }
            }
            Some(80002) => {
                let config = Chains::Amoy.config();
                if neg_risk {
                    exchange_contract = config.neg_risk_config.exchange_contract
                } else {
                    exchange_contract = config.standard_config.exchange_contract
                }
            }
            _ => {
                return Err(ApiError::Unexpected(
                    format!("Unknown chain id {}", signer.chain_id().unwrap()).to_string(),
                ));
            }
        }
        Ok(Eip712Domain {
            name: Some(ORDER_NAME.into()),
            version: Some(ORDER_VERSION.into()),
            chain_id: Some(U256::from(Chains::Polygon.id())),
            verifying_contract: Some(exchange_contract),
            ..Eip712Domain::default()
        })
    }
}
