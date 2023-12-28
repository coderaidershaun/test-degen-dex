use ethers::prelude::{Middleware, Provider, Http};
use ethers::types::{TxHash, Address};
use std::error::Error;

/// Extracts Tr
pub async fn get_address_from_transaction(provider: &Provider<Http>, tx_hash: TxHash) -> Result<Address, Box<dyn Error>> {
  let tx = provider.get_transaction(tx_hash).await?
    .ok_or("Transaction not found")?;
  Ok(tx.from)
}

