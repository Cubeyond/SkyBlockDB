mod structs;
mod fetch_data;
mod data_proceessing;

use fetch_data::fetch_auction;
use data_proceessing::json_to_polars;

use std::sync::{Arc, Mutex};
use std::time::Instant;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let all_auctions = Arc::new(Mutex::new(Vec::new()));

    fetch_auction(all_auctions.clone()).await.expect("Failed to fetch auctions");

    let auctions = all_auctions.lock().unwrap();

    match json_to_polars(&auctions) {
        Ok(df) => println!("Polars Dataframe: {:?}", df),
        Err(e) => eprintln!("Error: {}", e),
    }

    let elapsed_time = start_time.elapsed();
    println!("Total elapsed time: {:?}", elapsed_time);

    Ok(())
}