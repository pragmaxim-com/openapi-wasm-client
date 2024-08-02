use crate::db::init_db;
use crate::handlers::{retrieve_data, store_data};
use warp::Filter;

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
