use ehttp::{Request as EhttpRequest, Response as EhttpResponse};
use model::{Address, Block, ErrorResponse};
use std::sync::mpsc::{channel, Receiver};

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
        }
    }

    pub fn store_block(&self, block: Block) -> Receiver<Result<(), ErrorResponse>> {
        let (tx, rx) = channel();
        let url = format!("{}/blocks", self.base_url);
        let request = EhttpRequest::post(url, serde_json::to_string(&block).unwrap().into_bytes());

        ehttp::fetch(request, move |response: ehttp::Result<EhttpResponse>| {
            let result = match response {
                Ok(response) => {
                    if response.status == 201 {
                        Ok(())
                    } else {
                        let error: ErrorResponse =
                            serde_json::from_slice(&response.bytes).expect("Failed to parse error");
                        Err(error)
                    }
                }
                Err(_) => Err(ErrorResponse::new("Request failed")),
            };
            tx.send(result).expect("Failed to send result");
        });

        rx
    }

    pub fn store_address(&self, address: Address) -> Receiver<Result<(), ErrorResponse>> {
        let (tx, rx) = channel();
        let url = format!("{}/addresses", self.base_url);
        let request =
            EhttpRequest::post(url, serde_json::to_string(&address).unwrap().into_bytes());

        ehttp::fetch(request, move |response: ehttp::Result<EhttpResponse>| {
            let result = match response {
                Ok(response) => {
                    if response.status == 201 {
                        Ok(())
                    } else {
                        let error: ErrorResponse =
                            serde_json::from_slice(&response.bytes).expect("Failed to parse error");
                        Err(error)
                    }
                }
                Err(_) => Err(ErrorResponse::new("Request failed")),
            };
            tx.send(result).expect("Failed to send result");
        });

        rx
    }

    pub fn get_blocks(&self) -> Receiver<Result<Vec<Block>, ErrorResponse>> {
        let (tx, rx) = channel();
        let url = format!("{}/blocks", self.base_url);
        let request = EhttpRequest::get(url);

        ehttp::fetch(request, move |response: ehttp::Result<EhttpResponse>| {
            let result = match response {
                Ok(response) => {
                    if response.status == 200 {
                        let blocks: Vec<Block> = serde_json::from_slice(&response.bytes)
                            .expect("Failed to parse response");
                        Ok(blocks)
                    } else {
                        let error: ErrorResponse =
                            serde_json::from_slice(&response.bytes).expect("Failed to parse error");
                        Err(error)
                    }
                }
                Err(_) => Err(ErrorResponse::new("Request failed")),
            };
            tx.send(result).expect("Failed to send result");
        });

        rx
    }

    pub fn get_addresses(&self) -> Receiver<Result<Vec<Address>, ErrorResponse>> {
        let (tx, rx) = channel();
        let url = format!("{}/addresses", self.base_url);
        let request = EhttpRequest::get(url);

        ehttp::fetch(request, move |response: ehttp::Result<EhttpResponse>| {
            let result = match response {
                Ok(response) => {
                    if response.status == 200 {
                        let addresses: Vec<Address> = serde_json::from_slice(&response.bytes)
                            .expect("Failed to parse response");
                        Ok(addresses)
                    } else {
                        let error: ErrorResponse =
                            serde_json::from_slice(&response.bytes).expect("Failed to parse error");
                        Err(error)
                    }
                }
                Err(_) => Err(ErrorResponse::new("Request failed")),
            };
            tx.send(result).expect("Failed to send result");
        });

        rx
    }
}
