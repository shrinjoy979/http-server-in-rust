use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::serve;

async fn print_hello() -> &'static str {
    "Hello from rust!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(print_hello));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum server is running on http://{}", address);

    let listener = TcpListener::bind(address).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
