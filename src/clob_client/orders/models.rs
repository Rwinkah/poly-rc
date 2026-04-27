use std::borrow::Cow;
use alloy::primitives::{Address, Signature, U256};
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
use crate::shared::constants::{LOT_SIZE_SCALE, ORDER_NAME, ORDER_VERSION, USDC_DECIMALS};
use rust_decimal::Decimal;

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


#[derive(Debug, Serialize, Deserialize)]
pub struct UsdcAmount(Decimal);

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareAmount(Decimal);

impl UsdcAmount {
    pub fn new(amount:Decimal) -> Result<Self, ApiError> {
        match amount.normalize().scale() > USDC_DECIMALS.into() {
            true => Err(ApiError::Decode(format!("{amount} can not be used as a USDC value, fractional part must be <= {USDC_DECIMALS} in length"))),
            false => Ok(UsdcAmount(amount)),
        }
    }
}

impl ShareAmount {
    pub fn new(amount:Decimal) -> Result<Self, ApiError> {
        match amount.normalize().scale() > LOT_SIZE_SCALE.into() {
            true => Err(ApiError::Decode(format!("{amount} can not be used as a Share value, fractional part must be <= {LOT_SIZE_SCALE} in length"))),
            false => Ok(ShareAmount(amount.normalize()))
        }
    }
}





pub enum OrderData {
    LimitData {
        token_id: U256,
        side: Side,
        price: f32,
        size: f32,
        nonce:Option<u64>,
        expiration: Option<chrono::DateTime<chrono::Utc>>,
        taker: Option<Address>,
        order_type: Option<OrderType>,
        post_only: Option<bool>,
        funder: Option<Address>,


    },
    MarketData {
        token_id: U256,
        amount: f32,
        size: f32,
        side: Side,
    },
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
