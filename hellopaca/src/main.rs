use axum::Router;
use axum::routing::get;
use std::env::var;
use std::io::Result;
use tokio::net::TcpListener;

const BIND: &str = "[::]:49211";

async fn index() -> &'static str {
    "Hellopaca, World!\n"
}

#[tokio::main]
async fn main() -> Result<()> {
    let address = var("BIND").unwrap_or(BIND.into());
    let listener = TcpListener::bind(address).await?;
    let routes = Router::new().route("/", get(index));

    axum::serve(listener, routes).await
}
