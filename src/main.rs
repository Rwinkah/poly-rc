#![allow(unused_imports)]

use std::{collections::btree_set::SymmetricDifference, hash::DefaultHasher, os::unix::fs};

use poly_rc::public::{
    PubClient, TokenId,
    events::Events,
    events::models::EventDTO,
    orderbook::OrderBook,
    pricing::{Pricing, models::MarketPriceDTO},
    sports::{Sports, models::SportsTeamsDTO},
};

use poly_rc::public::events;
use poly_rc::shared::Side;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
