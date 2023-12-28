use crate::WORKING_DIR;
use crate::models::bitquery::{BitQueryData, TradeInfo};
use reqwest::Client;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::io::Write;

#[derive(Debug, Serialize, Clone)]
pub struct QueryVariables {
  pub network: String,
  pub limit: i32,
  pub offset: i32,
  pub token: String,
  pub pool: String,
}

#[derive(Serialize)]
struct GraphQLQuery {
  pub query: String,
  pub variables: QueryVariables,
}

#[derive(Debug)]
pub struct DataManager {
  pub query_vars: QueryVariables,
  pub dex_trade_data: Option<Vec<TradeInfo>>
}

impl DataManager {
  pub fn new(network: &str, limit: i32, offset: i32, token: &str, pool: &str) -> Self {
    Self {
      query_vars: QueryVariables {
        network: network.to_string(),
        limit,
        offset,
        token: token.to_string(),
        pool: pool.to_string()
      },
      dex_trade_data: None
    }
  }

  /// Get Trade Data
  /// Fetches trade data from BitQuery
  async fn get_trade_data(&self) -> Result<BitQueryData, reqwest::Error> {
    let client = Client::new();
    let graphql_url = "https://streaming.bitquery.io/graphql";
  
    let query = GraphQLQuery {
      query: "
        query ($network: evm_network, $limit: Int!, $token: String!, $pool: String!) {
          EVM(network: $network, dataset: combined) {
            DEXTradeByTokens(
              orderBy: {descending: Block_Number}
              limit: {count: $limit}
              where: {Trade: {Currency: {SmartContract: {is: $token}, ProtocolName: {}}, Dex: {Pair: {SmartContract: {is: $pool}}}}}
            ) {
              ChainId
              Block {
                Number
                Time
              }
              Trade {
                Dex {
                  ProtocolName
                }
                Seller
                Buyer
                Amount
                Currency {
                  SmartContract
                  Symbol
                }
                Price
                Side {
                  Amount
                  Currency {
                    SmartContract
                    Symbol
                  }
                }
              }
              Transaction {
                Hash
                From
                Index
              }
            }
          }
        }
      ".to_string(),
      variables: self.query_vars.clone(),
    };
  
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-API-KEY", "BQYPxLzWNw6hBdlhegCZWhkEnN5DNH9o".parse().unwrap());
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());
  
    let response = client
      .post(graphql_url)
      .headers(headers)
      .json(&query)
      .send()
      .await?
      .json::<BitQueryData>()
      .await?;
  
    Ok(response)
  }

  /// Get or Create Data
  /// Retrieves data if not exists otherwise loads it
  pub async fn load_or_get_new_trade_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = format!("{}/{}.json", WORKING_DIR, self.query_vars.pool);
    let trades_data: BitQueryData = if Path::new(file_path.as_str()).exists() {
      println!("loading data...");
      let data_text: String = fs::read_to_string(file_path).expect("Could not read file");
      serde_json::from_str::<BitQueryData>(&data_text).expect("Failed to read data from file")
    } else {
      println!("fetching data...");
      let trade_data = self.get_trade_data().await?;
      let td_text: String = serde_json::to_string_pretty(&trade_data).expect("Failed to convert data to string");
      let mut file_save = fs::File::create(file_path).expect("Unable to create file");
      file_save.write_all(td_text.as_bytes()).expect("Unable to write data");
      trade_data
    };

    let Some(trades) = trades_data.data.get("EVM") else { panic!("No EVM data was found") };
    let trades_reversed: Vec<TradeInfo> = trades.dex_trade_by_tokens.iter().rev().cloned().collect();
    self.dex_trade_data = Some(trades_reversed);
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{NETWORK, TOKEN, POOL};

  #[tokio::test]
  async fn it_gets_data() {
    let network: &str = NETWORK;
    let limit: i32 = 2;
    let offset: i32 = 0;
    let _: () = DataManager::new(network, limit, offset, TOKEN, POOL).load_or_get_new_trade_data().await
      .expect("Failed to get or load data");
  }
}
