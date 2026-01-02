use poly_rc::public::orderbook::{TokenId, get_orderbook_summary};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let result = get_orderbook_summary(TokenId { token_id: "1".to_string() }).await;
    println!("{:?}", result);
}