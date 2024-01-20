use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]

#[derive(Debug, Deserialize)]
pub struct AuctionResponse {
    pub totalPages: u8,
    pub auctions: Vec<AuctionItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionItem {
    pub uuid: String,
    pub item_name: String,
    pub item_lore: String,
    pub starting_bid: u64,
    pub bin: bool,
}