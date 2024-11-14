#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate desktop_client;
#[macro_use]
extern crate std_plus;

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    serve, Router,
};
use std::{net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug)]
enum Error {}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

async fn app() {
    let Ok(addr) = "127.0.0.1:3000".parse::<SocketAddr>() else {
        error!("Error: Unable to parse socket address");
        return;
    };

    let static_file_service =
        ServeDir::new("v0.1-experimental").append_index_html_on_directories(true);

    let router = Router::new()
        .fallback_service(static_file_service)
        .route("/config", get(config_get))
        .route("/content", get(content_get));

    info!("SERVER {:?}", addr);
    if let Err(err) = serve(TcpListener::bind(&addr).await.unwrap(), router).await {
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

        app().await
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

        app().await
    }
}

// Config
#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn config_get() -> Result<&'static str> {
    Ok("We got it lol")
}

// Content
#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn content_get() -> Result<&'static str> {
    todo!()
}
