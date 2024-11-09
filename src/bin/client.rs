#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp;

use axum::Extension;
use nextcamp::{api::route::client_router, Config};
use std::net::SocketAddr;
use tokio::{fs::read_to_string, net::TcpListener};

async fn app() {
    let Ok(config_file) = read_to_string("./config.toml").await else {
        error!("Error: Unable to read the config file!");
        return;
    };

    let Ok(addr) = "127.0.0.1:3000".parse::<SocketAddr>() else {
        error!("Error: Unable to parse socket address");
        return;
    };

    let Ok(config) = toml::from_str::<Config>(&config_file) else {
        error!("Error: Unable to parse the config information you provided");
        return;
    };

    let router = client_router().layer(Extension(config));

    info!("SERVER {:?}", addr);
    if let Err(err) = axum::serve(TcpListener::bind(&addr).await.unwrap(), router).await {
        error!("Error: {:?}", err);
    }
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
