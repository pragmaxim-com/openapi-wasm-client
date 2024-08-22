use gloo_net::http::Request;
use models::{Block, ErrorResponse};
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

    pub async fn store_block(&self, block: Block) -> Result<Block, ErrorResponse> {
        let response = Request::post(&format!("{}/blocks", self.base_url))
            .json(&block)
            .expect("Failed to serialize block")
            .send()
            .await
            .expect("Failed to send request");

        if response.ok() {
            let block: Block = response.json().await.expect("Failed to parse response");
            Ok(block)
        } else {
            let error: ErrorResponse = response.json().await.expect("Failed to parse error");
            Err(error)
        }
    }
}

#[wasm_bindgen]
pub async fn store_block_wasm(block: JsValue) -> Result<JsValue, JsValue> {
    let block: Block = serde_wasm_bindgen::from_value(block).unwrap();
    let client = ApiClient::new("http://localhost:8080");

    match client.store_block(block).await {
        Ok(block) => Ok(serde_wasm_bindgen::to_value(&block).unwrap()),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err).unwrap()),
    }
}
