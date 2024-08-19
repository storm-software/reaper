use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EthExecutionRequest {
  pub symbol: String,
  pub exchange_buy: String,
  pub exchange_sell: String,
}
