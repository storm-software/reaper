use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
pub struct EthExecutionRequest {
  pub symbol: String,
  pub exchange_buy: String,
  pub exchange_sell: String,
}
