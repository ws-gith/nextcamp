use axum::{routing::get, Router};

pub type Result<T> = core::result::Result<T, ()>;

pub fn router() -> Router<()> {
    Router::new().route("/", get(GET))
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn GET() -> Result<&'static str> {
    Ok("Hey welcome to user")
}
