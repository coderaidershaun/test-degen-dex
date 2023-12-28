use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct PositionOpen {
  pub purchase_amount_base_qty: f64,
  pub remaining_amount_base: f64,
  pub purchase_price_quote: f64,
}

#[derive(Debug)]
pub struct PositionClosed {
  pub selling_amount_base_qty: f64,
  pub selling_price_quote: f64,
}

#[derive(Debug)]
pub struct AddressRecords {
  pub positions_open: Vec<PositionOpen>,
  pub positions_closed: Vec<PositionClosed>,
  pub unrealized_quote_pnl: f64, // calculate on every price movement: (Current Market Price per Token * Remaining Quantity) - (Buying Price per Token * Remaining Quantity)
  pub realized_quote_pnl: f64, // calculate on closed trade: (Selling Price per Token * Quantity Sold) - (Buying Price per Token * Quantity Sold)
  pub external_origin_profit: f64, // In case of excess close to what was in open (probably a dump of tokens or arbitrage trade). This could be added to realized_quote_pnl if desired
  pub count_profit: u64, // calculate on closed trade
  pub count_loss: u64, // calculate on closed trade
}

impl AddressRecords {
  pub fn new() -> Self {
    Self { positions_open: vec![], positions_closed: vec![], unrealized_quote_pnl: 0.0, realized_quote_pnl: 0.0, external_origin_profit: 0.0, count_profit: 0, count_loss: 0 }
  }

  /// Add Open Position
  /// Adds an open position
  fn add_open_position(&mut self, purchase_amount_base_qty: f64, purchase_price_quote: f64) {
    let position: PositionOpen = PositionOpen{ 
      purchase_amount_base_qty,
      remaining_amount_base: purchase_amount_base_qty,
      purchase_price_quote 
    };
    self.positions_open.push(position);
  }

  /// Remove Open Position
  /// Removes open position if closed
  fn remove_open_position(&mut self, index: usize) {
    if index < self.positions_open.len() {
      self.positions_open.remove(index);
    } else {
      panic!("No open position found");
    }
  }

  fn close_positions(&mut self, sell_base_qty: f64, sell_quote_price: f64) {
    let mut remaining_sell_qty: f64 = sell_base_qty;

    // Handle if Open Positions exist
    for open_pos in &mut self.positions_open {

      // If utilizing all of an open position
      if remaining_sell_qty >= open_pos.remaining_amount_base {
        let realized_profit = (sell_quote_price * remaining_sell_qty) - (open_pos.purchase_price_quote * remaining_sell_qty);
        remaining_sell_qty -= open_pos.remaining_amount_base;
        open_pos.remaining_amount_base = 0.0;
      } else {

      }
    }

    // Handle if Sell Quantity still has remaining value (probably a dump of tokens or arbitrage trade from another pool)
  }
}