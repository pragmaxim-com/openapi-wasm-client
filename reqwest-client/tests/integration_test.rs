use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_store_and_get_addresses() {
    use model::Address;
    use reqwest_client::ApiClient;
    let base_url = "http://localhost:8082".to_string();

    let client = ApiClient::new(base_url.clone());

    // Store an address
    let address = Address {
        address: "123 Rust Ave".to_string(),
        balance: 1,
    };
    client.store_address(address).await.unwrap();

    // Get the addresses
    let addresses = client.get_addresses().await.unwrap();
    assert_eq!(addresses.len(), 1);
}

#[wasm_bindgen_test]
async fn test_store_and_get_blocks() {
    use model::Block;
    use reqwest_client::ApiClient;
    let base_url = "http://localhost:8082".to_string();

    let client = ApiClient::new(base_url.clone());

    // Store an address
    let block = Block {
        block_id: "123 Rust Ave".to_string(),
        height: 1,
    };
    client.store_block(block).await.unwrap();

    // Get the addresses
    let blocks = client.get_blocks().await.unwrap();
    assert_eq!(blocks.len(), 1);
}
