use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]

#[derive(Debug, Deserialize)]
pub struct AuctionResponse {
    totalPages: u8,
    auctions: Vec<AuctionItem>,
}

impl AuctionResponse {
    pub fn total_pages(&self) -> u8 {
        self.totalPages
    }

    pub fn auctions(&self) -> &Vec<AuctionItem> {
        &self.auctions
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionItem {
    uuid: String,
    item_name: String,
    item_lore: String,
    starting_bid: u64,
    bin: bool,
}

impl AuctionItem {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn item_name(&self) -> &str {
        &self.item_name
    }

    pub fn item_lore(&self) -> &str {
        &self.item_lore
    }

    pub fn price(&self) -> u64 {
        self.starting_bid
    }

    pub fn bin(&self) -> bool {
        self.bin
    }
}