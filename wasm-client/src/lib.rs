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
    let blocks = client.get_blocks().await?;

    console::log_1(&format!("Size {}", blocks.len()).into());

    Ok(())
}
