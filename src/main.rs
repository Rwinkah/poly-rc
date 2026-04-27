use alloy::signers::Signer;
use dotenv::dotenv;
use alloy::primitives::{{Address, FixedBytes, U256}};
use alloy::signers::local::PrivateKeySigner;

use poly_rc::clob_client::{ClobClient, models::ClobClientArgs};
use poly_rc::clob_client::orders::{Orders,  models::{OrderCreateDTO, Order, UsdcAmount, OrderType}};
use poly_rc::shared::Side;

use std::io::Write;

use std::time::Instant;
use std::{env, io};
use rust_decimal::Decimal;

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
    let numbe: U256 = U256::from(100);

    let clob_client_args = ClobClientArgs {
        clob_url,
        auto_retry: None,
        private_key: Some(private_key),
    };

    let new_client = ClobClient::new(clob_client_args).await.unwrap();
    let credentials = new_client.l2_credentials();


    // let signature = new_client.
    let order = Order { 
        maker: signer.address(), 
        signer: signer.address(), 
        taker: todo!(),
        tokenId: todo!(),
        makerAmount: todo!(), 
        takerAmount: todo!(), 
        expiration: todo!(), 
        nonce: todo!(), 
        feeRateBps: todo!(), 
        side: todo!(), 
        signatureType: todo!(), 
        salt:  todo!()
    };

}
