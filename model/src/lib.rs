use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Address {
    pub address: String,
    pub balance: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub block_id: String,
    pub height: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}
