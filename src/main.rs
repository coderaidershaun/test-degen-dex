mod datamanager;
mod models;

use models::address::AddressRecords;
use models::bitquery::Trade;
use models::general::{Analysis, Criteria, DollarBar, PnlBar, Side, TradeTx, VolumeBar};
use models::traits::TimeBars;

use std::collections::HashMap;
use std::io::Write;

const WORKING_DIR: &str = "/Users/shaun/Code/DEVELOPMENT/degentest";
const TOKEN: &str = "0xa41d2f8ee4f47d3b860a149765a7df8c3287b7f0";
const POOL: &str = "0x197d7010147df7b99e9025c724f13723b29313f8";
const NETWORK: &str = "eth";
const OFFSET: i32 = 0;
const LIMIT: i32 = 10000;
const DOLLAR_BAR_LIMIT: f64 = 10.0;
const criteria: Criteria = Criteria { is_dollar_bars: true, is_volume_bars: true, is_pnl_bars: true, is_transactions_bars: true };

#[tokio::main]
async fn main() {

    // Load or Fetch Data
    let mut dm = datamanager::DataManager::new(NETWORK, LIMIT, OFFSET, TOKEN, POOL);
    let extract_res = dm.load_or_get_new_trade_data().await;
    let _: () = match extract_res {
        Ok(dm) => dm,
        Err(e) => panic!("{}", e)
    };
    let Some(trades_data) = dm.dex_trade_data else { panic!("No data was found") };

    // // Initialize variables
    // let mut blocks: Vec<String> = vec![];
    // let mut block_times: Vec<String> = vec![];
    // let mut transactions: Vec<String> = vec![];
    // let mut accounts: Vec<String> = vec![];
    // let mut amounts_base: Vec<f64> = vec![];
    // let mut amounts_quote: Vec<f64> = vec![];
    // let mut sides: Vec<String> = vec![];
    // let mut prices: Vec<f64> = vec![];

    // let mut internal_realized_pnls: Vec<f64> = vec![];
    // let mut external_realized_pnls: Vec<f64> = vec![];
    // let mut unrealized_pnls: Vec<f64> = vec![];
    // let mut open_interests: Vec<f64> = vec![];
    // let mut volumes_quote_buy: Vec<f64> = vec![];
    // let mut volumes_quote_sell: Vec<f64> = vec![];
    // let mut unique_address_trades: Vec<u64> = vec![];

    let mut unique_address_trade_counts_hm: HashMap<String, u64> = HashMap::new();
    let mut address_records_hm: HashMap<String, AddressRecords> = HashMap::new();

    // Initialize for Dollar Bars
    let mut analysis: Analysis = Analysis::new();
    let mut dollar_bars: Vec<DollarBar> = vec![];
    let mut volume_bars: Vec<VolumeBar> = vec![];
    let mut pnl_bars: Vec<PnlBar> = vec![];
    let mut trade_transactions: Vec<TradeTx> = vec![];

    let mut is_init: bool = true;
    let mut cumulative_qty: f64 = 0.0;
    let mut dollar_bar: DollarBar = DollarBar::new();
    let mut volume_bar: VolumeBar = VolumeBar::new();
    let mut pnl_bar: PnlBar = PnlBar::new();

    // Calculate metrics for each trade
    for trade in trades_data {
        let block_num: u64 = trade.block.number.parse::<u64>().unwrap();
        let side: Side = if trade.trade.buyer != POOL { Side::Buy } else { Side::Sell };
        let mut trade_tx: TradeTx = TradeTx::new(trade.transaction.hash, block_num, trade.block.time, side, trade.transaction.from);

        let amount_base: f64 = trade.trade.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let amount_quote: f64 = trade.trade.side.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let price_quote: f64 = amount_quote / amount_base;
        trade_tx.volume_base = amount_base;
        trade_tx.volume_quote = amount_quote;
        trade_tx.price_quote = price_quote;

        // Calculate Volume
        let volume_quote_buy: f64 = if trade_tx.side == Side::Buy { amount_quote } else { 0.0 };
        let volume_quote_sell: f64 = if trade_tx.side == Side::Sell { amount_quote } else { 0.0 };

        // Calculate Count of Trades for Given Address
        let current_address_count_opt = unique_address_trade_counts_hm.get(trade_tx.account_addr.as_str());
        let addr_trade_count: u64 = match current_address_count_opt {
            Some(&count) => count + 1,
            None => 1
        };
        unique_address_trade_counts_hm.insert(trade_tx.account_addr.clone(), addr_trade_count);

        // Update records with current trade and increment realized P&L
        let mut account_realized_internal_pnl = 0.0;
        let mut account_realized_external_pnl = 0.0;
        let addr_record_opt = address_records_hm.get(trade_tx.account_addr.as_str());
        let mut record: AddressRecords = if let Some(addr_record) = addr_record_opt { addr_record.clone() } else { AddressRecords::new() };
        if trade_tx.side == Side::Buy {
            record.open_position(amount_base, price_quote);
        } else {
            let (internal_pnl, external_pnl) = record.close_positions(amount_base, price_quote);
            account_realized_internal_pnl = internal_pnl;
            account_realized_external_pnl = external_pnl;
        }
        trade_tx.account_won = record.count_profit;
        trade_tx.account_lost = record.count_loss;
        trade_tx.account_realized_pnl = account_realized_internal_pnl;
        trade_tx.account_external_pnl = account_realized_external_pnl;
        address_records_hm.insert(trade_tx.account_addr.clone(), record);

        // Update unrealized records
        let mut account_unrealized_pnl = 0.0;
        let mut account_open_interest_base = 0.0;
        let mut account_trades_open = 0;
        for (_, record_obj) in &address_records_hm {
            account_open_interest_base += record_obj.get_open_interest();
            account_unrealized_pnl += record_obj.calculate_unrealized_position(price_quote);
            account_trades_open = record_obj.count_open_positions();
        }
        trade_tx.account_unrealized_pnl = account_unrealized_pnl;
        trade_tx.account_open_interest_base = account_open_interest_base;
        trade_tx.account_trades_open = account_trades_open;

        // Update dollar bars
        if criteria.is_dollar_bars || criteria.is_pnl_bars || criteria.is_volume_bars {
            volume_bar.volume_buys += volume_quote_buy;
            volume_bar.volume_sells += volume_quote_sell;
    
            pnl_bar.internal_realized_pnl = account_realized_internal_pnl;
            pnl_bar.external_realized_pnl = account_realized_external_pnl;
    
            dollar_bar.close = price_quote;
    
            if is_init == true {
                dollar_bar.datetime = trade_tx.block_time.clone();
                volume_bar.datetime = trade_tx.block_time.clone();
                pnl_bar.datetime = trade_tx.block_time.clone();
    
                dollar_bar.open = price_quote;
                dollar_bar.high = price_quote;
                dollar_bar.low = price_quote;
    
                is_init = false;
            }
    
            if price_quote > dollar_bar.high { dollar_bar.high = price_quote; }
            if price_quote < dollar_bar.low { dollar_bar.low = price_quote; }
    
            cumulative_qty += amount_base;
    
            if cumulative_qty >= DOLLAR_BAR_LIMIT {
                dollar_bars.push(dollar_bar.clone());
                volume_bars.push(volume_bar);
                pnl_bars.push(pnl_bar);
                dollar_bar = DollarBar::new();
                volume_bar = VolumeBar::new();
                pnl_bar = PnlBar::new();
                cumulative_qty = 0.0;
                is_init = true;
            }
        }

        // blocks.push(block_num);
        // block_times.push(block_time);
        // transactions.push(tx_hash);
        // accounts.push(account_addr);
        // amounts_base.push(amount_base);
        // amounts_quote.push(amount_quote);
        // sides.push(side.to_string());
        // prices.push(price_quote);

        // volumes_quote_buy.push(volume_quote_buy);
        // volumes_quote_sell.push(volume_quote_sell);
        // unique_address_trades.push(addr_trade_count);

        // internal_realized_pnls.push(realized_internal_pnl);
        // external_realized_pnls.push(realized_external_pnl);
        // unrealized_pnls.push(unrealized_pnl);
        // open_interests.push(open_interest_base);
    }

    // Update analysis bars
    if criteria.is_dollar_bars { analysis.dollar_bars = Some(dollar_bars); }
    if criteria.is_volume_bars { analysis.volume_bars = Some(volume_bars); }
    if criteria.is_pnl_bars { analysis.pnl_bars = Some(pnl_bars); }
    if criteria.is_transactions_bars { analysis.transactions = Some(trade_transactions); }

    // let file_name = "dollarbars.txt";
    // let db_str: String = serde_json::to_string::<Vec<DollarBar>>(&analysis.dollar_bars.unwrap()).expect("Failed to serialize Dollar Bars");
    // let mut file = std::fs::File::create(file_name).expect("Failed to create file");
    // file.write_all(db_str.as_bytes()).expect("Failed to write to file");

    // for (i, qty) in amounts_quote.iter().enumerate() {
    //     let current_price = prices[i];

    //     volume_bar.volume_buys += volumes_quote_buy[i];
    //     volume_bar.volume_sells += volumes_quote_sell[i];

    //     pnl_bar.internal_realized_pnl = internal_realized_pnls[i];
    //     pnl_bar.external_realized_pnl = external_realized_pnls[i];

    //     dollar_bar.close = current_price;
    //     if is_init == true {
    //         dollar_bar.datetime = block_times[i].clone();
    //         volume_bar.datetime = block_times[i].clone();
    //         pnl_bar.datetime = block_times[i].clone();

    //         dollar_bar.open = current_price;
    //         dollar_bar.high = current_price;
    //         dollar_bar.low = current_price;

    //         is_init = false;
    //     }

    //     if current_price > dollar_bar.high { dollar_bar.high = current_price; }
    //     if current_price < dollar_bar.low { dollar_bar.low = current_price; }

    //     cumulative_qty += qty;

    //     if cumulative_qty >= DOLLAR_BAR_LIMIT {
    //         dollar_bars.push(dollar_bar.clone());
    //         volume_bars.push(volume_bar);
    //         pnl_bars.push(pnl_bar);
    //         dollar_bar = DollarBar::new();
    //         volume_bar = VolumeBar::new();
    //         pnl_bar = PnlBar::new();
    //         cumulative_qty = 0.0;
    //         is_init = true;
    //     }
    // }



    // // Save to csv
    // let file_path = "/Users/shaun/Code/DEVELOPMENT/degentest/prices.csv";
    // let file = std::fs::File::create(file_path).unwrap();
    // let mut wtr = csv::Writer::from_writer(file);

    // // Writing the column headings
    // wtr.write_record(&[
    //     "Block", 
    //     "Block Time", 
    //     "Transaction", 
    //     "Side",
    //     "Account", 
    //     "Amount Base", 
    //     "Amount Quote", 
    //     "Volume Buy",
    //     "Volume Sell",
    //     "Internal Realized PnLs",
    //     "External Realized PnLs",
    //     "Unrealized PnLs",
    //     "Open Interest",
    //     "Prices"
    // ]).unwrap();

    // for (((((((((((((
    //     &ref block, 
    //     &ref block_time), 
    //     &ref transaction), 
    //     &ref side), 
    //     &ref account), 
    //     &ref amount_base), 
    //     &ref amount_quote), 
    //     &ref volume_buy), 
    //     &ref volume_sell), 
    //     &ref int_realized_pnl), 
    //     &ref ext_realized_pnl), 
    //     &ref unrealized_pnl), 
    //     &ref open_interest), 
    //     &ref prices
    // ) 
    // in blocks.iter()
    //     .zip(block_times.iter())
    //     .zip(transactions.iter())
    //     .zip(sides.iter())
    //     .zip(accounts.iter())
    //     .zip(amounts_base.iter())
    //     .zip(amounts_quote.iter())
    //     .zip(volumes_quote_buy.iter())
    //     .zip(volumes_quote_sell.iter())
    //     .zip(internal_realized_pnls.iter())
    //     .zip(external_realized_pnls.iter())
    //     .zip(unrealized_pnls.iter()) 
    //     .zip(open_interests.iter()) 
    //     .zip(prices.iter()) 
    // {
    //     wtr.write_record(&[
    //         block, 
    //         block_time, 
    //         transaction, 
    //         side, 
    //         &account, 
    //         &amount_base.to_string(), 
    //         &amount_quote.to_string(), 
    //         &volume_buy.to_string(), 
    //         &volume_sell.to_string(), 
    //         &int_realized_pnl.to_string(), 
    //         &ext_realized_pnl.to_string(), 
    //         &unrealized_pnl.to_string(),
    //         &open_interest.to_string(),
    //         &prices.to_string(),
    //     ]).unwrap();
    // }
    // wtr.flush().unwrap();

}
