use dotenv::dotenv;

use alloy::primitives::{Address, U256};
use alloy::signers::local::PrivateKeySigner;
use poly_rc::clob_client::ClobClient;
use poly_rc::clob_client::models::ClobClientArgs;
use std::env;
use std::str::FromStr;
use std::time::Instant;
use poly_rc::clob_client::orders::models::Order;
use poly_rc::clob_client::orders::Orders;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let clob_url = env::var("CLOB_URL").expect("CLOB_URL must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer: PrivateKeySigner = private_key.parse().unwrap();
    let numbe: U256 = U256::from(100);

    let args = ClobClientArgs {
        clob_url,
        auto_retry: None,
        private_key: Some(private_key),
    };
    let new_client: ClobClient = ClobClient::new(args).await.unwrap();
    let credentials = new_client.l2_credentials();

    let order_body = new_client.build_order();

    let response = new_client.post_order(

    )

    let duration = start.elapsed();
    println!("{:?}", credentials);
    println!("{:?}", duration);
    println!("{:?}", signer.address());
}
