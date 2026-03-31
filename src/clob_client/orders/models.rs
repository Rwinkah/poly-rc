use crate::clob_client::ClobClient;
use crate::shared::Side;
use alloy::sol;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

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
#[serde(rename_all = "camelCase")]
pub struct OrderCreateDTO {
    pub order: Order,
    pub owner: String,
    pub order_type: Option<OrderType>,
    pub defer_exec: bool,
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
