mod db;
mod handlers;
mod models;

use db::init_db;
use handlers::{retrieve_data, store_data};
use warp::Filter;

#[tokio::main]
async fn main() {
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
    db: db::Db,
) -> impl Filter<Extract = (db::Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
