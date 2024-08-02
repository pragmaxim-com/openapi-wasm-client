use crate::db::{get_data, init_db, insert_data, Db};
use crate::models::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn store_data_actix(data: web::Json<Data>, db: web::Data<Db>) -> impl Responder {
    match insert_data(db.get_ref().clone(), data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Data stored successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to store data"),
    }
}

async fn retrieve_data_actix(db: web::Data<Db>) -> impl Responder {
    match get_data(db.get_ref().clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve data"),
    }
}

pub async fn run_actix_server() -> std::io::Result<()> {
    let db = init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/store", web::post().to(store_data_actix))
            .route("/retrieve", web::get().to(retrieve_data_actix))
    })
    .bind("127.0.0.1:3031")?
    .run()
    .await
}
