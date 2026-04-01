use alloy::signers::Signer;
use dotenv::dotenv;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;

use poly_rc::clob_client::ClobClient;
use poly_rc::clob_client::models::ClobClientArgs;
use poly_rc::clob_client::orders::Orders;
use poly_rc::clob_client::orders::models::{OrderCreateDTO, Order, OrderType};
use poly_rc::shared::Side;

use std::io::Write;
use std::str::FromStr;
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
    
    // let signature = new_client.
    let order = Order { 
        maker: signer.address(), 
        signer: signer.address(), 
        taker: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        tokenId: U256::from(55284251473480771174001412691877696451865690949888921217752300450606336097674), 
        makerAmount: todo!(), 
        takerAmount: todo!(), 
        expiration: todo!(), 
        nonce: todo!(), 
        feeRateBps: todo!(), 
        side: todo!(), 
        signatureType: todo!(), 
        salt:  
    };

    let order_create_dto = OrderCreateDTO {
        order: order,
        owner: new_client.l2_credentials().api_key.clone(),
        order_type: Some(OrderType::FOK),
        defer_exec: false,
    };

    let order = new_client.post_order(order_create_dto).await.unwrap();
}
