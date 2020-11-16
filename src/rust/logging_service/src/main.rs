use std::{convert::Infallible, net::SocketAddr};
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;

use chrono::Utc;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use uuid::Uuid;

use hyper_tls::HttpsConnector;

fn log_error(remote_addr: SocketAddr, message: impl AsRef<[u8]>) {
    let timestamp = Utc::now().format("%Y-%m-%d__%H:%M");
    let ip = remote_addr.ip();
    let uuid = Uuid::new_v4();
    let file_name = format!("{}__{}__{}", timestamp, ip, uuid);
    fs::create_dir_all("log").unwrap();
    fs::write(Path::new("log").join(file_name), message).unwrap();
}

async fn handle_request
(remote_addr: SocketAddr, request: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (request.uri().path(), request.method()) {
        ("/log", &Method::POST) => {
            // TODO: Check if the type is plain text
            let error_message = hyper::body::to_bytes(request.into_body()).await.unwrap();
            log_error(remote_addr, error_message);
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::from(""))
                .unwrap())
        },
        ("/log", _) => {
            Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::from(""))
                .unwrap())
        },
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap())
        },
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 20060));

    let make_service = make_service_fn(|connection: &AddrStream| {
        let addr = connection.remote_addr();
        let service = service_fn(move |req| { handle_request(addr, req) });
        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::try_bind(&addr).unwrap().serve(make_service);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
