pub mod user;

use super::middleware::debug_logger;
use axum::{middleware::from_fn, Router};

pub type Result<T> = core::result::Result<T, ()>;

pub fn api_router() -> Router<()> {
    let app = Router::new().nest("/user", user::router());

    Router::new().nest("/api", app).layer(from_fn(debug_logger))
}

pub fn client_router() -> Router<()> {
    Router::new().layer(from_fn(debug_logger))
}
