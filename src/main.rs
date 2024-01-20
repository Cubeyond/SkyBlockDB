mod structs;
mod fetch_data;
mod data_processing;

use fetch_data::fetch_auction;
use structs::AuctionItem;
use data_processing::json_to_polars;

use std::time::Instant;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let mut all_auctions: Vec<AuctionItem> = Vec::new();

    fetch_auction(&mut all_auctions).await?;

    match json_to_polars(&all_auctions) {
        Ok(df) => println!("Polars Dataframe: {:?}", df),
        Err(e) => eprintln!("Error: {}", e),
    }

    let elapsed_time = start_time.elapsed();

    println!("Total elapsed time: {:?}", elapsed_time);

    Ok(())
}