use gloo_net::http::Request;
use model::{Address, Block, ErrorResponse};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
        }
    }

    pub async fn store_block(&self, block: Block) -> Result<(), ErrorResponse> {
        let response = Request::post(&format!("{}/blocks", self.base_url))
            .json(&block)
            .expect("Failed to serialize block")
            .send()
            .await
            .expect("Failed to send request");

        if response.status() == 201 {
            Ok(())
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }

    pub async fn store_address(&self, address: Address) -> Result<(), ErrorResponse> {
        let response = Request::post(&format!("{}/addresses", self.base_url))
            .json(&address)
            .expect("Failed to serialize address")
            .send()
            .await
            .expect("Failed to send request");

        if response.status() == 201 {
            Ok(())
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }

    pub async fn get_blocks(&self) -> Result<Vec<Block>, ErrorResponse> {
        let response = Request::get(&format!("{}/blocks", self.base_url))
            .send()
            .await
            .expect("Failed to send request");

        if response.ok() {
            let blocks: Vec<Block> = response.json().await.expect("Failed to parse response");
            Ok(blocks)
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }

    pub async fn get_addresses(&self) -> Result<Vec<Address>, ErrorResponse> {
        let response = Request::get(&format!("{}/addresses", self.base_url))
            .send()
            .await
            .expect("Failed to send request");

        if response.ok() {
            let addresses: Vec<Address> = response.json().await.expect("Failed to parse response");
            Ok(addresses)
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }
}

#[wasm_bindgen]
pub async fn store_block_wasm(block: JsValue) -> Result<JsValue, JsValue> {
    let block: Block = serde_wasm_bindgen::from_value(block).unwrap();
    let client = ApiClient::new("http://localhost:8082");

    match client.store_block(block).await {
        Ok(_) => Ok(JsValue::undefined()), // No content to return, so return undefined
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[wasm_bindgen]
pub async fn store_address_wasm(address: JsValue) -> Result<JsValue, JsValue> {
    let address: Address = serde_wasm_bindgen::from_value(address).unwrap();
    let client = ApiClient::new("http://localhost:8082");

    match client.store_address(address).await {
        Ok(_) => Ok(JsValue::undefined()), // No content to return, so return undefined
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[wasm_bindgen]
pub async fn get_blocks_wasm() -> Result<JsValue, JsValue> {
    let client = ApiClient::new("http://localhost:8082");

    match client.get_blocks().await {
        Ok(blocks) => Ok(serde_wasm_bindgen::to_value(&blocks).unwrap()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[wasm_bindgen]
pub async fn get_addresses_wasm() -> Result<JsValue, JsValue> {
    let client = ApiClient::new("http://localhost:8082");

    match client.get_addresses().await {
        Ok(addresses) => Ok(serde_wasm_bindgen::to_value(&addresses).unwrap()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}
