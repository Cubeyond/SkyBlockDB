use crate::structs::{AuctionResponse, AuctionItem};

use reqwest::Client;


pub async fn fetch_auction(all_auctions: &mut Vec<AuctionItem>) -> Result<(), reqwest::Error> {
  let client = Client::new();

  let url = "https://api.hypixel.net/v2/skyblock/auctions";
  let response: AuctionResponse = client.get(url).send().await?.json().await?;

  for page in 0..response.totalPages {
      let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
      let response: AuctionResponse = client.get(&url).send().await?.json().await?;

      for item in response.auctions.into_iter().filter(|i| i.bin) {
          all_auctions.push(item);
      }
  }

  Ok(())
}