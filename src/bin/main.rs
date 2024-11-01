#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp;
#[macro_use]
extern crate std_plus;

use axum::{
    extract::Request,
    middleware::{from_fn, Next},
    response::Response,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

async fn app() -> Result<()> {
    let addr = "127.0.0.1:3000".parse::<SocketAddr>()?;

    let app = Router::<()>::new()
        .nest("/user", nextcamp::user::api::router())
        .layer(from_fn(debug_logger));

    info!("SERVER \n{:?}", addr);
    axum::serve(TcpListener::bind(&addr).await?, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
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

        app().await?;
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .pretty()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        app().await?;
    }

    Ok(())
}
#[inline]
pub async fn debug_logger(req: Request, next: Next) -> Response {
    #[cfg(debug_assertions)]
    {
        let (parts, body) = req.into_parts();

        info!("Got a request with parts: {:#?}", parts);
        next.run(Request::from_parts(parts, body)).await
    }

    #[cfg(not(debug_assertions))]
    {
        next.run(req).await
    }
}
