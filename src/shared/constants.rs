use crate::clob_client::config::{ChainConfig, ContractConfig};
use alloy::json_abi::InternalType::Contract;
use alloy::primitives::{Address, address};
use alloy::signers::k256::elliptic_curve::weierstrass::add;
use std::io::Chain;

pub const CLOB_ENDPOINT: &str = "https://clob.polymarket.com";
pub const GAMMA_ENDPOINT: &str = "https://gamma-api.polymarket.com";

pub const ORDER_NAME: &str = "Polymarket CTF Exchange";
pub const ORDER_VERSION: &str = "1";
