use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub(super) enum Error {}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub(super) async fn GET(header: HeaderMap) -> Result<&'static str> {
    println!("{:#?}", header);

    Ok("Ok, this is contact!")
}
