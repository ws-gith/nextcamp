pub mod user;

use crate::middleware::debug_logger;
use axum::{middleware::from_fn, Router};

pub type Result<T> = core::result::Result<T, ()>;

pub fn router() -> Router<()> {
    let app = Router::<()>::new().nest("/user", user::router());

    Router::<()>::new()
        .nest("/api", app)
        .layer(from_fn(debug_logger))
}
