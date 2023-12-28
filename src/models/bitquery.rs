use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitQueryData {
  pub data: HashMap<String, EVMData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EVMData {
  #[serde(rename = "DEXTradeByTokens")]
  pub dex_trade_by_tokens: Vec<TradeInfo>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeInfo {
  #[serde(rename = "Block")]
  pub block: BlockInfo,
  #[serde(rename = "ChainId")]
  pub chain_id: String,
  #[serde(rename = "Trade")]
  pub trade: Trade,
  #[serde(rename = "Transaction")]
  pub transaction: Transaction
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockInfo {
  #[serde(rename = "Number")]
  pub number: String,
  #[serde(rename = "Time")]
  pub time: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
  #[serde(rename = "Amount")]
  pub amount: String,
  #[serde(rename = "Buyer")]
  pub buyer: String,
  #[serde(rename = "Currency")]
  pub currency: Currency,
  #[serde(rename = "Dex")]
  pub dex: Dex,
  #[serde(rename = "Price")]
  pub price: f64,
  #[serde(rename = "Seller")]
  pub seller: String,
  #[serde(rename = "Side")]
  pub side: Side
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Currency {
  #[serde(rename = "SmartContract")]
  pub smart_contract: String,
  #[serde(rename = "Symbol")]
  pub symbol: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dex {
  #[serde(rename = "ProtocolName")]
  pub protocol_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Side {
  #[serde(rename = "Amount")]
  pub amount: String,
  #[serde(rename = "Currency")]
  pub currency: Currency
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
  #[serde(rename = "Hash")]
  pub hash: String,
  #[serde(rename = "From")]
  pub from: String
}
