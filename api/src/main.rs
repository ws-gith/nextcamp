// #![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp;

mod middleware;
#[allow(non_snake_case)]
mod routes;
mod security;

use axum::serve;
use routes::router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// we control the type we use across all our apps here
type User = nextcamp::user::Data<String>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_file(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    let Ok(addr) = "127.0.0.1:3000".parse::<SocketAddr>() else {
        error!("Error: Unable to parse socket address");
        return;
    };

    info!("SERVER {:?}", addr);
    if let Err(err) = serve(TcpListener::bind(&addr).await.unwrap(), router()).await {
        error!("Error: {:?}", err);
    }
}
