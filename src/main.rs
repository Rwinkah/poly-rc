use poly_rc::public::orderbook::{
    OrderbookRequestDTO, TokenId, get_orderbook_summary, post_orderbook_summaries,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let result = post_orderbook_summaries(OrderbookRequestDTO {
        token_ids: vec![TokenId {
            token_id:
                "97823728760211534985313322325333593565602743916910593548467619829591464019021"
                    .to_string(),
        }],
    })
    .await;

    match result {
        Ok(res) => {
            for item in res {
                println!(
                    "market:{}, min_order_size:{}",
                    item.market, item.min_order_size
                )
            }
        }
        Err(err) => {
            println!("{}", err.body);
            println!("{}", err.status);
            println!("{}", err.url.unwrap());
        }
    }
}
