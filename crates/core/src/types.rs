use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TradePrice {
  pub average: f64,
  pub min: f64,
  pub max: f64,
}

impl TradePrice {
  pub fn new(average: f64, min: f64, max: f64) -> Self {
    TradePrice { average, min, max }
  }

  pub fn new_empty() -> Self {
    TradePrice { average: 0.0, min: 0.0, max: 0.0 }
  }

  pub fn from_f64(value: f64) -> Self {
    TradePrice { average: value, min: value, max: value }
  }

  pub fn from_min_max(min: f64, max: f64) -> Self {
    TradePrice { average: (min + max) / 2.0, min, max }
  }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeSymbol {
  pub id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradePairs {
  pub currency_id1: String,
  pub currency_id2: String,
}

impl TradePairs {
  pub fn new(currency_id1: &str, currency_id2: &str) -> Self {
    TradePairs { currency_id1: currency_id1.to_string(), currency_id2: currency_id2.to_string() }
  }

  pub fn from_str(value: &str) -> Self {
    if value.contains("/") {
      return TradePairs { currency_id1: "".to_string(), currency_id2: "".to_string() };
    }

    let parts: Vec<&str> = value.split("/").collect();
    if parts.len() == 2 {
      TradePairs { currency_id1: parts[0].to_string(), currency_id2: parts[1].to_string() }
    } else {
      TradePairs { currency_id1: "".to_string(), currency_id2: "".to_string() }
    }
  }

  pub fn to_string(&self) -> String {
    format!("{}/{}", self.currency_id1, self.currency_id2)
  }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradePosition {
  pub symbol: TradeSymbol,
  pub exchange_id: String,
  pub price: TradePrice,
  pub volume: f64,
}
