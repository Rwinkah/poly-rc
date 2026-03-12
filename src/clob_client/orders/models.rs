use crate::clob_client::ClobClient;
use crate::shared::Side;
use serde;
use serde::{Deserialize, Serialize};

pub trait Orders {
    fn get_private_clob_client(&self) -> ClobClient;

    fn post_new_order(&self) {}
}

pub struct CreateOrderDTO {
    order: OrderDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    maker: String,
    signer: String,
    taker: String,
    token_id: String,
    maker_amount: String,
    taker_amount: String,
    side: Side,
    expiration: String,
    nonce: String,
    fee_rate_bps: String,
    signature: String,
    salt: String,
    signature_type: String,
}
