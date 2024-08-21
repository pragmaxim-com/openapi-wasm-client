use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[wasm_bindgen(start)]
pub fn run() {
    // Use `spawn_local` to run the async code.
    spawn_local(async {
        match main_async().await {
            Ok(_) => console::log_1(&"Success!".into()),
            Err(e) => console::log_1(&format!("Error: {:?}", e).into()),
        }
    });
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:8082");

    let new_block = &types::Block {
        block_id: "foo".to_string(),
        height: 1,
    };

    console::log_1(&JsValue::from_str("Storing new block"));
    let _ = client.store_block(&new_block).await?;
    console::log_1(&JsValue::from_str("Getting existing blocks"));
    let blocks = client.get_blocks().await?;

    console::log_1(&format!("Size {}", blocks.len()).into());

    Ok(())
}

#[wasm_bindgen]
pub struct JsClient {
    client: Client,
}

#[wasm_bindgen]
impl JsClient {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: &str) -> JsClient {
        JsClient {
            client: Client::new(base_url),
        }
    }

    #[wasm_bindgen]
    pub async fn get_blocks(&self) -> Result<JsValue, JsValue> {
        match self.client.get_blocks().await {
            Ok(blocks) => match serde_wasm_bindgen::to_value(&blocks.into_inner()) {
                Ok(val) => Ok(val),
                Err(err) => Err(JsValue::from_str(&format!(
                    "Serialization error: {:?}",
                    err
                ))),
            },
            Err(err) => Err(JsValue::from_str(&format!("{:?}", err))),
        }
    }
}
