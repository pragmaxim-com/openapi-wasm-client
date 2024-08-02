use serde_json::json;
use warp::http::StatusCode;
use warp::reply::Json;
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
