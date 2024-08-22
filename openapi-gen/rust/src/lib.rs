use apis::{configuration, default_api};
use models::Block1;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

pub mod apis;
pub mod models;

#[wasm_bindgen(start)]
pub fn run() {
    spawn_local(async {
        match main_async().await {
            Ok(_) => console::log_1(&"Success!".into()),
            Err(e) => console::log_1(&format!("Error: {:?}", e).into()),
        }
    });
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = configuration::Configuration::default();

    let new_block = Block1 {
        block_id: "foo".to_string(),
        height: 1,
    };

    console::log_1(&JsValue::from_str("Storing new block"));
    default_api::store_block(&configuration, new_block).await?;

    console::log_1(&JsValue::from_str("Getting existing blocks"));
    let blocks = default_api::get_blocks(&configuration).await?;

    console::log_1(&format!("Size {}", blocks.len()).into());

    Ok(())
}
