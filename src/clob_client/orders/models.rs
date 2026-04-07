use std::borrow::Cow;
use alloy::primitives::{Signature, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use dotenv::dotenv;
use crate::clob_client::ClobClient;
use crate::shared::{ApiError, Side};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use crate::clob_client::config::Chains;
use crate::shared::constants::{ORDER_NAME, ORDER_VERSION};

pub struct CreateOrderDTO {
    order: Order,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderCreateStatus {
    Live,
    Matched,
    Delayed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    GTC,
    FOK,
    GTD,
    FAK,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderDetails {
//     pub maker: String,
//     pub signer: String,
//     pub taker: String,
//     pub token_id: String,
//     pub maker_amount: String,
//     pub taker_amount: String,
//     pub side: Side,
//     pub expiration: String,
//     pub nonce: String,
//     pub fee_rate_bps: String,
//     pub signature: Option<String>,
//     pub salt: String,
//     pub signature_type: String,
// }

sol! {

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug)]
    struct Order {
        address maker;
        address signer;
        address taker;
        #[serde_as(as = "DisplayFromStr")]
        uint256 tokenId;
        #[serde_as(as = "DisplayFromStr")]
        uint256 makerAmount;
        #[serde_as(as = "DisplayFromStr")]
        uint256 takerAmount;
        #[serde_as(as = "DisplayFromStr")]
        uint256 expiration;
        #[serde_as(as = "DisplayFromStr")]
        uint256 nonce;
        #[serde_as(as = "DisplayFromStr")]
        uint256 feeRateBps;
        uint8 side;
        uint8 signatureType;
        uint256 salt;
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SignedOrder {
    #[serde(flatten)]
    order: Order,
    signature: Signature
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateDTO {
    pub order: SignedOrder,
    pub owner: String,
    pub order_type: OrderType,
    pub defer_exec: bool,
}


impl OrderCreateDTO {
    pub async fn new(order: Order, order_type: OrderType, owner: String, defer_exec: bool, domain: Eip712Domain, signer: &PrivateKeySigner) -> Result<Self, ApiError> {
        let order_signature = signer.sign_hash(&order.eip712_signing_hash(&domain)).await.unwrap();
        let signed_order = SignedOrder {
            order,
            signature: order_signature
        };

        Ok(
            Self {
                order: signed_order,
                owner,
                order_type,
                defer_exec,
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateResponse {
    pub success: bool,
    pub order_id: String,
    pub status: OrderCreateStatus,
    pub making_amount: String,
    pub taking_amount: String,
    pub transaction_hashes: Vec<String>,
    pub trade_ids: Vec<String>,
    pub error_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegRiskResponse {
    pub neg_risk: bool,
}
