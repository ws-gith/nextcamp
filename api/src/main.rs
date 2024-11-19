#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp_api;
#[macro_use]
extern crate std_plus;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract::Request, middleware::Next, response::Response};
use axum::{http::StatusCode, middleware::from_fn, Router};
use axum::{serve, Json};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

const SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
const BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;

#[derive(Debug)]
enum Error {}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

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

    let user_router = Router::new().route("/", get(user_get));
    let auth_router = Router::new().route("/", get(auth_get));

    let router = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/user", user_router)
                .nest("/authenticate", auth_router),
        )
        .layer(from_fn(debug_logger));

    info!("SERVER {:?}", addr);
    if let Err(err) = serve(TcpListener::bind(&addr).await.unwrap(), router).await {
        error!("Error: {:?}", err);
    }
}

#[inline]
pub async fn debug_logger(req: Request, next: Next) -> Response {
    if !cfg!(debug_assertions) {
        return next.run(req).await;
    }

    let (parts, body) = req.into_parts();
    tracing::info!("Got a request with parts: {:#?}", parts);
    next.run(Request::from_parts(parts, body)).await
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn auth_get(header: HeaderMap) -> Result<&'static str> {
    println!("{:#?}", header);

    Ok("Ok!")
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn user_get() -> Result<&'static str> {
    todo!()
}
