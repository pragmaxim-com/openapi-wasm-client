use model::{Address, Block, ErrorResponse};
use reqwest::Client;

#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        ApiClient {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn store_block(&self, block: Block) -> Result<(), ErrorResponse> {
        let response = self
            .client
            .post(&format!("{}/blocks", self.base_url))
            .json(&block)
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
        let response = self
            .client
            .post(&format!("{}/addresses", self.base_url))
            .json(&address)
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
        let response = self
            .client
            .get(&format!("{}/blocks", self.base_url))
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let blocks: Vec<Block> = response.json().await.expect("Failed to parse response");
            Ok(blocks)
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }

    pub async fn get_addresses(&self) -> Result<Vec<Address>, ErrorResponse> {
        let response = self
            .client
            .get(&format!("{}/addresses", self.base_url))
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let addresses: Vec<Address> = response.json().await.expect("Failed to parse response");
            Ok(addresses)
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn store_block_wasm(base_url: String, block: JsValue) -> Result<JsValue, JsValue> {
    let block: Block = serde_wasm_bindgen::from_value(block).unwrap();
    let client = ApiClient::new(base_url);

    match client.store_block(block).await {
        Ok(_) => Ok(JsValue::undefined()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn store_address_wasm(base_url: String, address: JsValue) -> Result<JsValue, JsValue> {
    let address: Address = serde_wasm_bindgen::from_value(address).unwrap();
    let client = ApiClient::new(base_url);

    match client.store_address(address).await {
        Ok(_) => Ok(JsValue::undefined()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn get_blocks_wasm(base_url: String) -> Result<JsValue, JsValue> {
    let client = ApiClient::new(base_url);

    match client.get_blocks().await {
        Ok(blocks) => Ok(serde_wasm_bindgen::to_value(&blocks).unwrap()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn get_addresses_wasm(base_url: String) -> Result<JsValue, JsValue> {
    let client = ApiClient::new(base_url);

    match client.get_addresses().await {
        Ok(addresses) => Ok(serde_wasm_bindgen::to_value(&addresses).unwrap()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}
