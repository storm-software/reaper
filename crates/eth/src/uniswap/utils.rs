//! Uniswap Utilities

use ethers::prelude::*;
use eyre::Result;
use hex::FromHex;

use crate::uniswap::constants::{UNIV2_FACTORY_ADDRESS, UNIV2_ROUTER_ADDRESS};
use crate::utils::convert;

/// Returns the Uniswap V2 Pair Contract Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
pub fn get_univ2_router_address() -> Address {
  convert::to_address(UNIV2_ROUTER_ADDRESS)
}

/// Returns the Uniswap V2 Factory Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
pub fn get_univ2_factory_address() -> Address {
  convert::to_address(UNIV2_FACTORY_ADDRESS)
}

/// Gets the Uniswap V2 Pair Contract Address given two token addresses
pub fn calculate_uniswap_v2_pair_address(a: &Address, b: &Address) -> Result<Address> {
  // Sort the tokens
  let mut tokens = vec![a, b];
  tokens.sort();

  // Copy the token addresses into a byte array
  let mut data = [0u8; 40];
  data[0..20].copy_from_slice(tokens[0].as_bytes());
  data[20..].copy_from_slice(tokens[1].as_bytes());

  // Hash the concatenated token address bytes
  let salt = ethers::utils::keccak256(data);

  // Get the init code
  let init_code =
    <[u8; 32]>::from_hex("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f")
      .map_err(|_| eyre::eyre!("Invalid init code hex"))?;

  // Get the uniswap factory address
  let factory = get_univ2_factory_address();

  // Compute the address with create2
  Ok(ethers::utils::get_create2_address_from_hash(factory, salt, init_code))
}

/// Returns how much output if we supply in
/// Follows: Uniswap v2; x * y = k formula
/// Accounts for a 0.3% fee
pub fn get_univ2_data_given_in(
  a_in: &U256,
  a_reserves: &U256,
  b_reserves: &U256,
) -> (U256, U256, U256) {
  // Calculate the output
  let a_in_with_fee: U256 = a_in * 997;
  let numerator: U256 = a_in_with_fee * b_reserves;
  let denominator: U256 = a_reserves * 1000 + a_in_with_fee;
  let b_out: U256 = numerator.checked_div(denominator).unwrap_or(U256::zero());

  // Calculate the new b reserves, accounting for underflow
  let new_b_reserves = b_reserves.checked_sub(b_out).unwrap_or(U256::one());

  // Calculate the new a reserves, accounting for overflow
  let new_a_reserves = a_reserves.checked_add(*a_in).unwrap_or(U256::MAX);

  // Return
  (b_out, new_a_reserves, new_b_reserves)
}

/// Returns how much output if we supply out
/// Follows: Uniswap v2; x * y = k formula
/// Accounts for a 0.3% fee
pub fn get_univ2_data_given_out(
  b_out: &U256,
  a_reserves: &U256,
  b_reserves: &U256,
) -> (U256, U256, U256) {
  // Calculate the new b reserves, accounting for underflow
  let new_b_reserves = b_reserves.checked_sub(*b_out).unwrap_or(U256::zero());

  // Calculate the amount in
  let numerator: U256 = a_reserves * b_out * 1000;
  let denominator: U256 = new_b_reserves * 997;
  let a_in = numerator.checked_div(denominator).unwrap_or(U256::MAX - 1) + 1;

  // Calculate the new a reserves, accounting for overflow
  let new_a_reserves = a_reserves.checked_add(a_in).unwrap_or(U256::MAX);

  // Return
  (a_in, new_a_reserves, new_b_reserves)
}
