use crate::structs::{AuctionResponse, AuctionItem};

use reqwest::Client;
use std::sync::{Arc, Mutex};


pub async fn fetch_auction(all_auctions: Arc<Mutex<Vec<AuctionItem>>>) -> Result<(), reqwest::Error> {
  let client = Client::new();

  let url = "https://api.hypixel.net/v2/skyblock/auctions";
  let response: AuctionResponse = client.get(url).send().await?.json().await?;

  for page in 0..response.total_pages() {
      let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
      let response: AuctionResponse = client.get(&url).send().await?.json().await?;

      for item in response.auctions().iter().filter(|i| i.bin()).cloned() {
          all_auctions.lock().unwrap().push(item);
      }
  }

  Ok(())
}