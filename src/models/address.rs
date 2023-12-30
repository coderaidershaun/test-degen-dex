use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionOpen {
  pub purchase_amount_base_qty: f64,
  pub remaining_amount_base: f64,
  pub purchase_price_quote: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionClosed {
  pub selling_amount_base_qty: f64,
  pub selling_price_quote: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddressRecords {
  pub positions_open: Vec<PositionOpen>,
  pub positions_closed: Vec<PositionClosed>,
  pub count_profit: u64,
  pub count_loss: u64,
}

impl AddressRecords {
  pub fn new() -> Self {
    Self { 
      positions_open: vec![], 
      positions_closed: vec![],
      count_profit: 0, 
      count_loss: 0 
    }
  }

  /// Open Position
  /// Adds an open position
  pub fn open_position(&mut self, purchase_amount_base_qty: f64, purchase_price_quote: f64) {
    let position: PositionOpen = PositionOpen{ 
      purchase_amount_base_qty,
      remaining_amount_base: purchase_amount_base_qty,
      purchase_price_quote 
    };
    self.positions_open.push(position);
  }

  /// Calculate Unrealized PnL
  /// Calculates unrealized profit and loss
  pub fn calculate_unrealized_position(&self, current_quote_price: f64) -> f64 {
    let mut unrealized_pnl = 0.0;
    for open_pos in &self.positions_open {
      if open_pos.remaining_amount_base != 0.0 {
        let open_pnl: f64 = (current_quote_price * open_pos.remaining_amount_base) - (open_pos.purchase_price_quote * open_pos.remaining_amount_base);
        unrealized_pnl += open_pnl;
      }
    }
    unrealized_pnl
  }

  /// Count Open Positions
  /// Returns the number of open positions
  pub fn count_open_positions(&self) -> usize {
    let mut counts = 0;
    for open_pos in &self.positions_open {
      if open_pos.remaining_amount_base > 0.0 {
        counts += 1;
      }
    }
    counts
  }

  /// Get Open Interest
  /// Returns open interest
  pub fn get_open_interest(&self) -> f64 {
    let mut open_interest = 0.0;
    for open_pos in &self.positions_open {
      open_interest += open_pos.remaining_amount_base;
    }
    open_interest
  }

  /// Close Position
  /// Closes any open positions and increments realized pnl
  /// Returns realized pnl and external pnl
  pub fn close_positions(&mut self, sell_base_qty: f64, sell_quote_price: f64) -> (f64, f64) {
    let mut remaining_sell_qty: f64 = sell_base_qty;

    // Handle if Open Positions exist
    // If utilizing all of an open position
    // If only partial close of a position (fully utilize remaining_sell_qty)
    let mut internal_pnl: f64 = 0.0;
    let mut external_pnl: f64 = 0.0;
    for open_pos in &mut self.positions_open {
      
      let open_pos_pnl: f64;
      if remaining_sell_qty >= open_pos.remaining_amount_base {
        open_pos_pnl = (sell_quote_price * open_pos.remaining_amount_base) - (open_pos.purchase_price_quote * open_pos.remaining_amount_base);
        remaining_sell_qty -= open_pos.remaining_amount_base;
        open_pos.remaining_amount_base = 0.0;
      } else {
        open_pos_pnl = (sell_quote_price * remaining_sell_qty) - (open_pos.purchase_price_quote * remaining_sell_qty);
        open_pos.remaining_amount_base -= remaining_sell_qty;
        remaining_sell_qty = 0.0;
      }

      if open_pos_pnl > 0.0 { self.count_profit += 1 };
      if open_pos_pnl < 0.0 { self.count_loss += 1 };
      internal_pnl += open_pos_pnl;
    }

    // Handle if Sell Quantity still has remaining value (probably a dump of tokens or arbitrage trade from another pool)
    if remaining_sell_qty > 0.0 {
      external_pnl += sell_quote_price * remaining_sell_qty;
    }

    // Return internal and external pnl
    (internal_pnl, external_pnl)
  }
}
