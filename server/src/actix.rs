use crate::db::{get_addresses, get_blocks, init_db, insert_address, insert_block, Db};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use model::{Address, Block};

async fn store_address(address: web::Json<Address>, db: web::Data<Db>) -> impl Responder {
    let a = address.into_inner();
    println!("Storing address {} at height {}", a.address, a.balance);
    match insert_address(db.get_ref().clone(), a).await {
        Ok(_) => {
            println!("Address stored...");
            HttpResponse::Created().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn store_block(block: web::Json<Block>, db: web::Data<Db>) -> impl Responder {
    let b = block.into_inner();
    println!("Storing block {} at height {}", b.block_id, b.height);
    match insert_block(db.get_ref().clone(), b).await {
        Ok(_) => {
            println!("Block stored...");
            HttpResponse::Created().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn retrieve_addresses(db: web::Data<Db>) -> impl Responder {
    let addresses = get_addresses(db.get_ref().clone()).await;
    println!("Retreiving {} addresses", addresses.len());
    HttpResponse::Ok().json(addresses)
}

async fn retrieve_blocks(db: web::Data<Db>) -> impl Responder {
    let blocks = get_blocks(db.get_ref().clone()).await;
    println!("Retreiving {} blocks", blocks.len());
    HttpResponse::Ok().json(blocks)
}

pub async fn run_actix_server() -> std::io::Result<()> {
    let db = init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .max_age(3600),
            )
            .app_data(web::Data::new(db.clone()))
            .route(
                "/openapi.json",
                web::get().to(|| async { fs::NamedFile::open("./openapi.json") }),
            )
            .service(fs::Files::new("/swagger", "./swagger-ui").index_file("index.html"))
            .service(fs::Files::new("/client", "./client").index_file("index.html"))
            .service(fs::Files::new("/client/pkg", "./client/pkg"))
            .service(fs::Files::new("/progenitor", "./progenitor").index_file("index.html"))
            .service(fs::Files::new("/progenitor/pkg", "./progenitor/pkg"))
            .service(fs::Files::new("/openapi-gen", "./openapi-gen/rust").index_file("index.html"))
            .service(fs::Files::new("/openapi-gen/pkg", "./openapi-gen/rust/pkg"))
            .route("/blocks", web::get().to(retrieve_blocks))
            .route("/addresses", web::get().to(retrieve_addresses))
            .route("/blocks", web::post().to(store_block))
            .route("/addresses", web::post().to(store_address))
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
