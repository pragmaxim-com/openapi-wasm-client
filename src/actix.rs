use crate::{
    db::{get_addresses, get_blocks, init_db, insert_address, insert_block, Db},
    models::{Address, Block},
};
use actix_files as fs;
use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

async fn store_address(address: web::Json<Address>, db: web::Data<Db>) -> impl Responder {
    match insert_address(db.get_ref().clone(), address.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Data stored successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to store data"),
    }
}

async fn store_block(block: web::Json<Block>, db: web::Data<Db>) -> impl Responder {
    match insert_block(db.get_ref().clone(), block.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Data stored successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to store data"),
    }
}

async fn retrieve_addresses(db: web::Data<Db>) -> impl Responder {
    let addresses = get_addresses(db.get_ref().clone()).await;
    HttpResponse::Ok().json(addresses)
}

async fn retrieve_blocks(db: web::Data<Db>) -> impl Responder {
    let blocks = get_blocks(db.get_ref().clone()).await;
    HttpResponse::Ok().json(blocks)
}

pub async fn run_actix_server() -> std::io::Result<()> {
    let db = init_db().await;

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/api-docs", "./").index_file("openapi.yaml"))
            .service(fs::Files::new("/", "./swagger-ui").index_file("index.html"))
            .app_data(web::Data::new(db.clone()))
            .route("/blocks", web::get().to(retrieve_blocks))
            .route("/addresses", web::get().to(retrieve_addresses))
            .route("/blocks", web::post().to(store_block))
            .route("/addresses", web::post().to(store_address))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
