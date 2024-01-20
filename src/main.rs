use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use polars::prelude::*;


#[allow(non_snake_case)]

#[derive(Debug, Deserialize)]
struct AuctionResponse {
    totalPages: usize,
    auctions: Vec<AuctionItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuctionItem {
    uuid: String,
    item_name: String,
    item_lore: String,
    starting_bid: u64,
    bin: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let client = Client::new();

    let mut all_auctions: Vec<AuctionItem> = Vec::new();

    let url = "https://api.hypixel.net/v2/skyblock/auctions";
    let response: AuctionResponse = client.get(url).send().await?.json().await?;

    for page in 0..response.totalPages {
        let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
        let response: AuctionResponse = client.get(&url).send().await?.json().await?;

        for item in response.auctions.into_iter().filter(|i| i.bin) {
            all_auctions.push(item);
        }
    }

    match json_to_polars(&all_auctions) {
        Ok(df) => println!("Polars Dataframe: {:?}", df),
        Err(e) => eprintln!("Error: {}", e),
    }

    let elapsed_time = start_time.elapsed();

    println!("Total elapsed time: {:?}", elapsed_time);

    Ok(())
}

// TODO: Use Regex extract attribute_type and tier from item_lore
// TODO: Sort the price
fn json_to_polars(auction_items: &[AuctionItem]) -> Result<DataFrame, Box<dyn std::error::Error>> {
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