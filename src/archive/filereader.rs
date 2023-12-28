use csv::{Reader, Writer};
use std::error::Error;
use std::fs::File;
use std::collections::HashSet;

/// Extract transaction ids
pub fn read_first_column_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let file = File::open(file_path)?;
  let mut rdr = Reader::from_reader(file);
  let mut first_column_values: Vec<String> = Vec::new();
  for result in rdr.records() {
    let record = result?;
    if let Some(first_column) = record.get(0) {
      first_column_values.push(first_column.to_string());
    }
  }
  Ok(first_column_values)
}

/// Convert transactions into a unique set
pub fn into_unique_transactions(transactions: Vec<String>) -> HashSet<String> {
  transactions.into_iter().collect()
}

/// Save transactions with addresses as lookup table
pub fn save_to_csv(column1: Vec<String>, column2: Vec<String>, file_path: &str) -> Result<(), Box<dyn Error>> {
  let file = File::create(file_path)?;
  let mut wtr = Writer::from_writer(file);
  for (item1, item2) in column1.iter().zip(column2.iter()) {
      wtr.write_record(&[item1, item2])?;
  }
  wtr.flush()?;
  Ok(())
}
