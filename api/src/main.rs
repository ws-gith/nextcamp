#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp_api;
#[macro_use]
extern crate std_plus;

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
enum Error {
    EnvNotFound,
    DatabaseError,
}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::*;

        match self {
            // Server Error
            // Log error and return internal server error
            EnvNotFound | DatabaseError => {
                //
                todo!()
            }
        }
    }
}

type Result<T> = core::result::Result<T, Error>;

async fn app() {
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

// Debug logger middleware
#[inline]
pub async fn debug_logger(req: Request, next: Next) -> Response {
    #[cfg(debug_assertions)]
    {
        let (parts, body) = req.into_parts();

        tracing::info!("Got a request with parts: {:#?}", parts);
        next.run(Request::from_parts(parts, body)).await
    }

    #[cfg(not(debug_assertions))]
    {
        next.run(req).await
    }
}

// Auth Router

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn auth_get() -> Result<&'static str> {
    todo!()
}

// User Router

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn user_get() -> Result<&'static str> {
    todo!()
}
