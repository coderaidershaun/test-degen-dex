mod datamanager;
mod filereader;
mod rpcalls;
mod models;

use ethers::prelude::{Provider, Http};
use ethers::types::TxHash;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/C7I2zeFgRWRF1CjB7L9uvB9WZDWVNcQk";

#[tokio::main]
async fn main() {

    let read_path = "/Users/shaun/Code/DEVELOPMENT/degentest/dex_transactions.csv";
    let write_path = "/Users/shaun/Code/DEVELOPMENT/degentest/tx_address_lookup.csv";

    let txs = filereader::read_first_column_from_csv(read_path).expect("Failed to extract column from csv");
    let unique_txs = filereader::into_unique_transactions(txs);

    let provider = Provider::<Http>::try_from(RPC_URL).expect("Failed to connect to provider");
    
    let mut transactions: Vec<String> = vec![];
    let mut addresses: Vec<String> = vec![];
    let mut counts = 1;
    let total_counts = unique_txs.len();
    for tx in unique_txs {
        sleep(Duration::from_millis(100));
        let tx_hash = TxHash::from_str(&tx).expect("Failed to convert transaction hash");
        let address_addr = rpcalls::get_address_from_transaction(&provider, tx_hash).await.expect("Failed to retrieve transaction");
        let address_str = format!("{:?}", address_addr);
        transactions.push(tx);
        addresses.push(address_str);
        let _: () = filereader::save_to_csv(transactions.clone(), addresses.clone(), write_path).expect("Failed to save file");
        counts += 1;
        let msg = format!("tx {} of {}", counts, total_counts);
        dbg!(&msg);
    }

    dbg!(res);
}
