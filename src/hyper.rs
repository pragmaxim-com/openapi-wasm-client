use crate::db::{get_data, init_db, insert_data, Db};
use crate::models::Data;
use bytes::Buf;
use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::{combinators::BoxBody, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    db: Db,
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/store") => {
            let whole_body = req.collect().await?.to_bytes();
            let data: Result<Data, _> = serde_json::from_reader(whole_body.reader());
            match data {
                Ok(data) => match insert_data(db.clone(), data).await {
                    Ok(_) => Ok(Response::new(
                        Full::new(Bytes::from("Data stored successfully")).boxed(),
                    )),
                    Err(_) => Ok(Response::new(
                        Full::new(Bytes::from("Failed to store data")).boxed(),
                    )),
                },
                Err(_) => Ok(Response::new(
                    Full::new(Bytes::from("Invalid data format")).boxed(),
                )),
            }
        }
        (&Method::GET, "/retrieve") => match get_data(db.clone()).await {
            Ok(data) => {
                let json_response = serde_json::to_string(&data).unwrap();
                Ok(Response::new(Full::new(Bytes::from(json_response)).boxed()))
            }
            Err(_) => Ok(Response::new(
                Full::new(Bytes::from("Failed to retrieve data")).boxed(),
            )),
        },
        _ => Ok(Response::new(Full::new(Bytes::from("Not Found")).boxed())),
    }
}

pub async fn run_hyper_server() {
    let db = init_db().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3032));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let io = TokioIo::new(stream);
        let db = db.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| handle_request(req, db.clone())))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
