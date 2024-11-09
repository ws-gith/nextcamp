use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

type Result<T> = core::result::Result<T, Error>;

pub(super) fn router() -> Router {
    Router::new().route("/", get(GET))
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn GET() -> Result<&'static str> {
    todo!()
}

#[derive(Debug)]
enum Error {}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}
