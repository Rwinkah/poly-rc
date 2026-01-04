#![allow(unused_imports)]

use poly_rc::public::{
    OrderbookRequestDTO, PubClient, TokenId,
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
                "3528283556348201539631245411187096877510646355957147907959655671383904729512"
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
