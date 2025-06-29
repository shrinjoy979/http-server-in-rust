use axum::{extract::Query, routing::get, Json, Router, serve};
use serde::Deserialize;
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;
use tokio::net::TcpListener;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[derive(Deserialize)]
struct BalancyQuery {
    address: String,
}

async fn get_balance(query: Query<BalancyQuery>) -> Json<serde_json::Value> {
    let client = RpcClient::new(String::from(RPC_URL));

    let address = &query.0.address;
    let pubkey = match address.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "error": "Invalid public key" })),
    };

    match client.get_balance(&pubkey) {
        Ok(lamports) => Json(json!({
            "address": address,
            "lamports": lamports,
            "SOL": lamports as f64 / 1_000_000_000.0,
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_balance));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum server is running on http://{}", address);

    let listener = TcpListener::bind(address).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
