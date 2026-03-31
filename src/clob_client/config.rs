use alloy::primitives::{address, Address};


pub struct ContractConfig {
  pub exchange_contract: Address,
  pub collateral_contract: Address,
  pub ctf_contract: Address,
  pub neg_risk_contract: Option<Address>,
}

pub struct ChainConfig {
  pub neg_risk_config: ContractConfig,
  pub standard_config: ContractConfig,
}



#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum Chains {
  Polygon = 137,
  Amoy = 80002,
}


impl Chains {
  pub fn id(self) -> u64 {
    self as u64
  }

  const POLYGON_COLLATERAL_CONTRACT:Address = address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174");
  const POLYGON_CTF_CONTRACT:Address = address!("0x4D97DCd97eC945f40cF65F87097ACe5EA0476045");
  const AMOY_COLLATERAL_CONTRACT:Address = address!("0x9c4e1703476e875070ee25b56a58b008cfb8fa78");
  const AMOY_CTF_CONTRACT:Address = address!("0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB");


  const POLYGON_NEG_RISK_CONFIG:ContractConfig = ContractConfig {
    exchange_contract: address!("0xC5d563A36AE78145C45a50134d48A1215220f80a"),
    collateral_contract: Self::POLYGON_COLLATERAL_CONTRACT,
    ctf_contract: Self::POLYGON_CTF_CONTRACT,
    neg_risk_contract: Some(address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296"))
  };



  const AMOY_NEG_RISK_CONFIG:ContractConfig = ContractConfig {
    exchange_contract: address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296"),
    collateral_contract: Self::AMOY_COLLATERAL_CONTRACT,
    ctf_contract: Self::AMOY_CTF_CONTRACT,
    neg_risk_contract: Some(address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296")),
  };


  const POLYGON_STD_CONFIG:ContractConfig = ContractConfig {
    exchange_contract: address!("0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"),
    collateral_contract: Self::POLYGON_COLLATERAL_CONTRACT,
    ctf_contract: Self::POLYGON_CTF_CONTRACT,
    neg_risk_contract: None
  };

  const AMOY_STD_CONFIG:ContractConfig = ContractConfig {
    exchange_contract: address!("0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40"),
    collateral_contract: Self::AMOY_COLLATERAL_CONTRACT,
    ctf_contract: Self::AMOY_CTF_CONTRACT,
    neg_risk_contract: None
  };


  pub fn config(self)-> ChainConfig {
    match self {
      Chains::Polygon => {
        ChainConfig {
          standard_config: Self::POLYGON_STD_CONFIG,
          neg_risk_config: Self::POLYGON_NEG_RISK_CONFIG,
        }
      },
      Chains::Amoy => {
        ChainConfig {
          standard_config: Self::POLYGON_STD_CONFIG,
          neg_risk_config: Self::POLYGON_NEG_RISK_CONFIG,
        }
      }
    }
  }







}

