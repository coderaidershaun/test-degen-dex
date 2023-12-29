use crate::models::traits::TimeBars;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DollarBar {
  pub datetime: String,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64
}

impl TimeBars for DollarBar {
  fn new() -> Self {
    Self { datetime: "".to_string(), open: 0.0, high: 0.0, low: 0.0, close: 0.0 }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeBar {
  pub datetime: String,
  pub volume_buys: f64,
  pub volume_sells: f64,
}

impl TimeBars for VolumeBar {
  fn new() -> Self { 
    Self { datetime: "".to_string(), volume_buys: 0.0, volume_sells: 0.0 }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PnlBar {
  pub datetime: String,
  pub internal_realized_pnl: f64,
  pub external_realized_pnl: f64
}

impl TimeBars for PnlBar {
  fn new() -> Self { 
    Self { datetime: "".to_string(), internal_realized_pnl: 0.0, external_realized_pnl: 0.0 }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeDetail{
  pub tx_hash: f64,
  pub block_num: f64,
  pub block_time: f64,
  pub side: Side,
  pub volume_base: f64,
  pub volume_quote: f64,
  pub price_quote: f64,
  pub account_addr: f64,
  pub account_buys: f64,
  pub account_sells: f64,
  pub account_unrealized_pnl: f64,
  pub account_realized_pnl: f64,
  pub account_external_pnl: f64
}
