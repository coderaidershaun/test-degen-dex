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
pub struct TradeTx{
  pub tx_hash: String,
  pub block_num: u64,
  pub block_time: String,
  pub side: Side,
  pub volume_base: f64,
  pub volume_quote: f64,
  pub price_quote: f64,
  pub account_addr: String,
  pub account_trades_open: usize,
  pub account_won: u64,
  pub account_lost: u64,
  pub account_unrealized_pnl: f64,
  pub account_realized_pnl: f64,
  pub account_external_pnl: f64,
  pub account_open_interest_base: f64
}

impl TradeTx {
  pub fn new(
    tx_hash: String, block_num: u64, block_time: String, side: Side, account_addr: String
  ) -> Self { 
    Self { tx_hash, block_num, block_time, side, volume_base: 0.0, volume_quote: 0.0, price_quote: 0.0, account_addr, account_won: 0, 
      account_lost: 0, account_trades_open: 0, account_unrealized_pnl: 0.0, account_realized_pnl: 0.0, account_external_pnl: 0.0, 
      account_open_interest_base: 0.0 }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Analysis {
  pub dollar_bars: Option<Vec<DollarBar>>,
  pub volume_bars: Option<Vec<VolumeBar>>,
  pub pnl_bars: Option<Vec<PnlBar>>,
  pub transactions: Option<Vec<TradeTx>>
}

impl TimeBars for Analysis {
  fn new() -> Self {
    Self {
      dollar_bars: None, volume_bars: None, pnl_bars: None, transactions: None
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Criteria {
  pub is_dollar_bars: bool,
  pub is_volume_bars: bool,
  pub is_pnl_bars: bool,
  pub is_transactions_bars: bool,
}
