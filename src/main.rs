mod datamanager;
mod models;

use models::address::AddressRecords;
use models::general::Side;

use std::collections::HashMap;

const WORKING_DIR: &str = "/Users/shaun/Code/DEVELOPMENT/degentest";
const TOKEN: &str = "0xa41d2f8ee4f47d3b860a149765a7df8c3287b7f0";
const POOL: &str = "0x197d7010147df7b99e9025c724f13723b29313f8";
const NETWORK: &str = "eth";
const OFFSET: i32 = 0;
const LIMIT: i32 = 10000;

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

    // Initialize variables
    let mut blocks: Vec<String> = vec![];
    let mut block_times: Vec<String> = vec![];
    let mut transactions: Vec<String> = vec![];
    let mut accounts: Vec<String> = vec![];
    let mut amounts_base: Vec<f64> = vec![];
    let mut amounts_quote: Vec<f64> = vec![];
    let mut sides: Vec<String> = vec![];
    let mut prices: Vec<f64> = vec![];

    let mut internal_realized_pnls: Vec<f64> = vec![];
    let mut external_realized_pnls: Vec<f64> = vec![];
    let mut unrealized_pnls: Vec<f64> = vec![];
    let mut open_interests: Vec<f64> = vec![];

    let mut volumes_quote_buy: Vec<f64> = vec![];
    let mut volumes_quote_sell: Vec<f64> = vec![];

    let mut unique_address_trade_counts_hm: HashMap<String, u64> = HashMap::new();
    let mut unique_address_trades: Vec<u64> = vec![];

    let mut address_records_hm: HashMap<String, AddressRecords> = HashMap::new();

    // Calculate metrics for each trade
    for trade in trades_data {
        let block_num: String = trade.block.number;
        let block_time: String = trade.block.time;
        let tx_hash: String = trade.transaction.hash;
        let account_addr: String = trade.transaction.from;
        let side: Side = if trade.trade.buyer != POOL { Side::Buy } else { Side::Sell };
        let amount_base: f64 = trade.trade.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let amount_quote: f64 = trade.trade.side.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let price_quote: f64 = amount_quote / amount_base;

        // Calculate Volume
        let volume_quote_buy: f64 = if side == Side::Buy { amount_quote } else { 0.0 };
        let volume_quote_sell: f64 = if side == Side::Sell { amount_quote } else { 0.0 };
        volumes_quote_buy.push(volume_quote_buy);
        volumes_quote_sell.push(volume_quote_sell);

        // Calculate Count of Trades for Given Address
        let current_address_count_opt = unique_address_trade_counts_hm.get(account_addr.as_str());
        let addr_trade_count: u64 = match current_address_count_opt {
            Some(&count) => count + 1,
            None => 1
        };
        unique_address_trade_counts_hm.insert(account_addr.clone(), addr_trade_count);
        unique_address_trades.push(addr_trade_count);

        // Update records with current trade and increment realized P&L
        let mut realized_internal_pnl = 0.0;
        let mut realized_external_pnl = 0.0;
        let addr_record_opt = address_records_hm.get(account_addr.as_str());
        let mut record: AddressRecords = if let Some(addr_record) = addr_record_opt { addr_record.clone() } else { AddressRecords::new() };
        if side == Side::Buy {
            record.open_position(amount_base, price_quote);
        } else {
            let (internal_pnl, external_pnl) = record.close_positions(amount_base, price_quote);
            realized_internal_pnl = internal_pnl;
            realized_external_pnl = external_pnl;
        }
        address_records_hm.insert(account_addr.clone(), record);

        // Update unrealized records
        let mut unrealized_pnl = 0.0;
        let mut open_interest_base = 0.0;
        for (_, record_obj) in &address_records_hm {
            open_interest_base += record_obj.get_open_interest();
            unrealized_pnl += record_obj.calculate_unrealized_position(price_quote);
        }

        
        blocks.push(block_num);
        block_times.push(block_time);
        transactions.push(tx_hash);
        accounts.push(account_addr);
        amounts_base.push(amount_base);
        amounts_quote.push(amount_quote);
        sides.push(side.to_string());
        prices.push(price_quote);

        internal_realized_pnls.push(realized_internal_pnl);
        external_realized_pnls.push(realized_external_pnl);
        unrealized_pnls.push(unrealized_pnl);
        open_interests.push(open_interest_base);
    }

    // Save to csv
    let file_path = "/Users/shaun/Code/DEVELOPMENT/degentest/prices.csv";
    let file = std::fs::File::create(file_path).unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    // Writing the column headings
    wtr.write_record(&[
        "Block", 
        "Block Time", 
        "Transaction", 
        "Side",
        "Account", 
        "Amount Base", 
        "Amount Quote", 
        "Volume Buy",
        "Volume Sell",
        "Internal Realized PnLs",
        "External Realized PnLs",
        "Unrealized PnLs",
        "Open Interest",
        "Prices"
    ]).unwrap();

    for (((((((((((((
        &ref block, 
        &ref block_time), 
        &ref transaction), 
        &ref side), 
        &ref account), 
        &ref amount_base), 
        &ref amount_quote), 
        &ref volume_buy), 
        &ref volume_sell), 
        &ref int_realized_pnl), 
        &ref ext_realized_pnl), 
        &ref unrealized_pnl), 
        &ref open_interest), 
        &ref prices
    ) 
    in blocks.iter()
        .zip(block_times.iter())
        .zip(transactions.iter())
        .zip(sides.iter())
        .zip(accounts.iter())
        .zip(amounts_base.iter())
        .zip(amounts_quote.iter())
        .zip(volumes_quote_buy.iter())
        .zip(volumes_quote_sell.iter())
        .zip(internal_realized_pnls.iter())
        .zip(external_realized_pnls.iter())
        .zip(unrealized_pnls.iter()) 
        .zip(open_interests.iter()) 
        .zip(prices.iter()) 
    {
        wtr.write_record(&[
            block, 
            block_time, 
            transaction, 
            side, 
            &account, 
            &amount_base.to_string(), 
            &amount_quote.to_string(), 
            &volume_buy.to_string(), 
            &volume_sell.to_string(), 
            &int_realized_pnl.to_string(), 
            &ext_realized_pnl.to_string(), 
            &unrealized_pnl.to_string(),
            &open_interest.to_string(),
            &prices.to_string(),
        ]).unwrap();
    }
    wtr.flush().unwrap();

}
