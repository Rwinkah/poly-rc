#![allow(unused_imports)]

use poly_rc::public::{
    PubClient, TokenId,
    orderbook::OrderBook,
    pricing::{Pricing, models::MarketPriceDTO},
    sports::{Sports, models::SportsTeamsDTO},
};

use poly_rc::shared::Side;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let pub_client = PubClient::new();
    let result = pub_client
        .get_sports_teams(SportsTeamsDTO {
            limit: Some(1),
            offset: Some(0),
            order: Some("name".to_string()),
            ascending: Some(true),
            league: None,
            name: None,
            abbreviation: None,
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
