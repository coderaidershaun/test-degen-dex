mod datamanager;
mod models;

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

    let mut volumes_base_buy: Vec<f64> = vec![];
    let mut volumes_base_sell: Vec<f64> = vec![];

    let mut volumes_quote_buy: Vec<f64> = vec![];
    let mut volumes_quote_sell: Vec<f64> = vec![];

    let mut unique_address_counts: HashMap<String, u64> = HashMap::new();

    let mut address_records: HashMap<String, String> = HashMap::new();

    for item in trades_data {
        let block_num: String = item.block.number;
        let block_time: String = item.block.time;
        let tx_hash: String = item.transaction.hash;
        let account_addr: String = item.transaction.from;
        let side: Side = if item.trade.buyer != POOL { Side::Buy } else { Side::Sell };
        let amount_base: f64 = item.trade.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let amount_quote: f64 = item.trade.side.amount.parse::<f64>().expect("Failed to convert String amount to f64");
        let price_quote: f64 = amount_quote / amount_base;

        let volume_base_buy: f64 = if side == Side::Buy { amount_base } else { 0.0 };
        let volume_base_sell: f64 = if side == Side::Sell { amount_base } else { 0.0 };
        volumes_base_buy.push(volume_base_buy);
        volumes_base_sell.push(volume_base_sell);

        let volume_quote_buy: f64 = if side == Side::Buy { amount_quote } else { 0.0 };
        let volume_quote_sell: f64 = if side == Side::Sell { amount_quote } else { 0.0 };
        volumes_quote_buy.push(volume_quote_buy);
        volumes_quote_sell.push(volume_quote_sell);

        let current_address_count_opt = unique_address_counts.get(account_addr.as_str());
        let addr_count: u64 = match current_address_count_opt {
            Some(&count) => count + 1,
            None => 1
        };
        unique_address_counts.insert(account_addr.clone(), addr_count);

        blocks.push(block_num);
        block_times.push(block_time);
        transactions.push(tx_hash);
        accounts.push(account_addr);
        amounts_base.push(amount_base);
        amounts_quote.push(amount_quote);
        sides.push(side.to_string());
        prices.push(price_quote);
    }

    // Save to csv
    let file_path = "/Users/shaun/Code/DEVELOPMENT/degentest/prices.csv";
    let file = std::fs::File::create(file_path).unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    for ((((((
        (&ref block, &ref block_time), &ref transaction), 
        &ref account), &ref amount_base), 
        &ref amount_quote), 
        &ref side), 
        &ref price
    ) 
    in blocks.iter()
        .zip(block_times.iter())
        .zip(transactions.iter())
        .zip(accounts.iter())
        .zip(amounts_base.iter())
        .zip(amounts_quote.iter())
        .zip(sides.iter())
        .zip(prices.iter()) 
    {
        wtr.write_record(&[
            block, 
            block_time, 
            transaction, 
            &account, 
            &amount_base.to_string(), 
            &amount_quote.to_string(), 
            side, 
            &price.to_string(),
        ]).unwrap();
    }
    wtr.flush().unwrap();

}
