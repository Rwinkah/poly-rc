use crate::clob_client::ClobClient;
use crate::clob_client::config::Chains;
use crate::clob_client::orders::models::{Order, OrderCreateDTO, OrderCreateResponse, OrderType};
use crate::public::{ApiError, AsyncHttpClient, PubClient, TokenId};
use crate::shared::constants::{ORDER_NAME, ORDER_VERSION};
use alloy::dyn_abi::Eip712Domain;
use alloy::primitives::{Address, U256};
use alloy::signers::k256::Secp256k1;
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::signers::Signer;
use reqwest::Response;
use crate::public::orderbook::OrderBook;

pub mod models;

pub trait Orders {
    fn get_private_clob_client(&self) -> &ClobClient;
    fn get_public_client(&self) -> PubClient;

    async fn post_order(&self, body: Order, defer_exec:bool, order_type: OrderType, owner: String) -> Result<OrderCreateResponse, ApiError> {

        let private_client = self.get_private_clob_client();
        let pub_client = self.get_public_client();

        let neg_risk = pub_client.get_neg_risk(TokenId {token_id:body.tokenId.to_string()}).await?;
        let domain = self.build_order_domain(private_client.signer.clone(), neg_risk.neg_risk ).await?;

        let signed_order = OrderCreateDTO::new(body.clone(), order_type, owner,defer_exec, domain, &private_client.signer).await;

        let str_body = serde_json::to_string(&body)?;

        let l2_headers = private_client
            .generate_l2_headers("POST", "/order", Some(str_body.as_str()))
            .await?;
        let response: Response = private_client
            .client
            .post(Some("/order"), Some(signed_order), None, Some(l2_headers.into()))
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
