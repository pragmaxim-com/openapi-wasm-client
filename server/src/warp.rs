use crate::db::init_db;
use serde_json::json;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::Filter;
use warp::Rejection;

use crate::db::{get_data, insert_data, Db};
use crate::models::Data;

pub async fn store_data(data: Data, db: Db) -> Result<impl warp::Reply, Rejection> {
    match insert_data(db, data).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Data stored successfully",
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            "Failed to store data",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn retrieve_data(db: Db) -> Result<Json, Rejection> {
    match get_data(db).await {
        Ok(data) => Ok(warp::reply::json(&data)),
        Err(_) => Ok(warp::reply::json(
            &json!({"error": "Failed to retrieve data"}),
        )),
    }
}

pub async fn run_warp_server() {
    let db = init_db().await;

    let store_route = warp::post()
        .and(warp::path("store"))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(store_data);

    let retrieve_route = warp::get()
        .and(warp::path("retrieve"))
        .and(with_db(db.clone()))
        .and_then(retrieve_data);

    let routes = store_route.or(retrieve_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(
    db: crate::db::Db,
) -> impl Filter<Extract = (crate::db::Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
