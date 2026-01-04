#![allow(unused_imports)]

use poly_rc::public::{
    PubClient, TokenId,
    orderbook::OrderBook,
    pricing::{Pricing, models::MarketPriceDTO},
};

use poly_rc::shared::Side;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let pub_client = PubClient::new();
    let result = pub_client
        .get_midpoint_price(TokenId {
            token_id:
                "3754334285616101662116579198768376424388375814028174583822278684167937945318"
                    .to_string(),
        })
        .await;

    match result {
        Ok(res) => {
            println!("found orderbook summaries: {:?}", res);
        }
        Err(err) => {
            println!("error: could not get midpoint price");
            println!("error: {:?}", err);
        }
    }
}
