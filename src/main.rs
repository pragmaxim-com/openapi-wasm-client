mod actix;
mod db;
mod models;

#[tokio::main]
async fn main() {
    actix::run_actix_server().await.unwrap()
}
