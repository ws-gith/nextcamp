#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp;

use axum::Router;
use nextcamp::api::route::client_router;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn app() {
    let Ok(addr) = "127.0.0.1:3000s".parse::<SocketAddr>() else {
        error!("Error: Unable to parse socket address");
        return;
    };

    info!("SERVER {:?}", addr);
    if let Err(err) = axum::serve(TcpListener::bind(&addr).await.unwrap(), client_router()).await {
        error!("Error: {:?}", err);
    }
}

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Config {
    key: String,
    delay: i64,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .pretty()
            .with_file(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_max_level(tracing::Level::DEBUG)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .init();

        app().await;
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .pretty()
            .without_time()
            .with_file(false)
            .with_line_number(false)
            .with_max_level(tracing::Level::DEBUG)
            .init();

        app().await;
    }
}
