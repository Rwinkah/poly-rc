use dotenv::dotenv;

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;

use poly_rc::clob_client::ClobClient;
use poly_rc::clob_client::models::ClobClientArgs;
use poly_rc::clob_client::orders::Orders;
use poly_rc::clob_client::orders::models::{OrderCreateDTO, OrderDetails, OrderType};
use poly_rc::shared::Side;

use std::io::Write;
use std::time::Instant;
use std::{env, io};

fn pause_exec(message: &str) {
    let mut buffer = String::new();

    print!("[{message}]\nEnter any command to continue: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer).unwrap();
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let clob_url = env::var("CLOB_URL").expect("CLOB_URL must be set");
    let signer: PrivateKeySigner = private_key.parse().unwrap();
    let maker = env::var("PROXY_ADDRESS").unwrap();

    let clob_client_args = ClobClientArgs {
        clob_url,
        auto_retry: None,
        private_key: Some(private_key),
    };

    let new_client = ClobClient::new(clob_client_args).await.unwrap();
    let credentials = new_client.l2_credentials();
    let token_id = String::from("");

    // let signature = new_client.
    let order = OrderDetails {
        maker: maker.clone(),
        signer: maker,
        taker: String::from("0x0000000000000000000000000000000000000000"),
        token_id,
        maker_amount: String::from("100000000"),
        taker_amount: String::from("200000000"),
        side: Side::BUY,
        expiration: String::from("1735699600"),
        nonce: String::from("0"),
        fee_rate_bps: String::from("30"),
        signature: String::from(""),
        salt: String::from("1234567890"),
        signature_type: String::from("1"),
    };

    let order_create_dto = OrderCreateDTO {
        order: todo!(),
        owner: new_client.l2_credentials().api_key.clone(),
        order_type: Some(OrderType::FOK),
        defer_exec: false,
    };

    let order = new_client.post_order(order_create_dto).await.unwrap();
}
