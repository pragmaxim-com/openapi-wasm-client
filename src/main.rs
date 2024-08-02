mod actix;
mod db;
mod handlers;
mod models;
mod warp;

use std::env;

#[tokio::main]
async fn main() {
    // Use an environment variable to select the server
    let server_type = env::var("SERVER_TYPE").unwrap_or_else(|_| "actix".to_string());

    match server_type.as_str() {
        "warp" => warp::run_warp_server().await,
        "actix" => {
            actix::run_actix_server().await.unwrap();
        }
        _ => eprintln!("Unknown server type. Use 'warp' or 'actix'."),
    }
}
