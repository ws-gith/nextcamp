pub mod auth;
mod leads;
mod letter;
mod smtp;
pub mod user;

use super::middleware::debug_logger;
use axum::{middleware::from_fn, Router};

pub fn router() -> Router<()> {
    let app = Router::new()
        .nest("/user", user::router())
        .nest("/smtp", smtp::router())
        .nest("/leads", leads::router())
        .nest("/letter", letter::router())
        .nest("/authenticate", auth::router());

    Router::new().nest("/api", app).layer(from_fn(debug_logger))
}
