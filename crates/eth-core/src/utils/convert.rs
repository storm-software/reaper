use ethers::prelude::*;

/// Converts &str to Address.
pub fn to_address(address: &str) -> Address {
  address.parse::<Address>().unwrap()
}

/// Converts normal input into 1e18.
pub fn to_1e18(input: u64) -> U256 {
  let ether: U256 = U256::exp10(18);
  let parsed: U256 = input.into();
  parsed * ether
}
