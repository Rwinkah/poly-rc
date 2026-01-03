#![allow(unused_imports)]

use poly_rc::public::{
    OrderbookRequestDTO, PubClient, TokenId,
    orderbook::OrderBook,
    pricing::{
        Pricing,
        models::{MarketPriceDTO, Side},
    },
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let pub_client = PubClient::new();
    let result = pub_client
        .get_market_price(MarketPriceDTO {
            token_id:
                "74693001438530122232203015312493762287298251343693104958670789864026566743517"
                    .to_string(),
            side: Side::BUY,
        })
        .await;

    match result {
        Ok(res) => {
            println!("found orderbook summaries: {}", res.price);
        }
        Err(err) => {
            println!("error: could not get orderbook summaries");
            println!("{}", err.body);
            println!("{}", err.status);
            println!("{}", err.url.unwrap());
        }
    }
}
