use crate::structs::AuctionItem;

use polars::prelude::*;


// TODO: Use Regex extract attribute_type and tier from item_lore
// TODO: Sort the price
pub fn json_to_polars(auction_items: &[AuctionItem]) -> Result<DataFrame, Box<dyn std::error::Error>> {
  let uuid: Vec<&str> = auction_items.iter().map(|item| item.uuid.as_str()).collect();
  let item_name: Vec<&str> = auction_items.iter().map(|item| item.item_name.as_str()).collect();
  let item_lore: Vec<&str> = auction_items.iter().map(|item| item.item_lore.as_str()).collect();
  let price: Vec<u64> = auction_items.iter().map(|item| item.starting_bid).collect();

  let df = DataFrame::new(vec![
      Series::new("uuid", &uuid),
      Series::new("item_name", &item_name),
      Series::new("item_lore", &item_lore),
      Series::new("price", &price),
  ])?;

  Ok(df)
}