mod actix;
mod db;

#[tokio::main]
async fn main() {
    actix::run_actix_server().await.unwrap()
}
