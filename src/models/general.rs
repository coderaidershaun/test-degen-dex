use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Side {
  Buy,
  Sell
}

impl Side {
  pub fn to_string(&self) -> String {
    match &self {
      Side::Buy => "Buy".to_string(),
      Side::Sell => "Sell".to_string()
    }
  }
}
