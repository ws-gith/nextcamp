use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

type Result<T> = core::result::Result<T, Error>;
const SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
const BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;

pub(super) fn router() -> Router {
    Router::new().route("/", get(GET))
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
async fn GET() -> Result<&'static str> {
    // Ok("Hey welcome to user")

    Err(Error::IncorrectPayload {
        data: string!("User shouldn't be here!"),
    })
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    Failed,
    IncorrectPayload { data: String },
}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Failed => (SERVER_ERROR, string!("Internal server error")).into_response(),
            Error::IncorrectPayload { data } => {
                #[derive(Serialize, new)]
                struct Data {
                    data: String,
                }

                (BAD_REQUEST, Json(Data::new(data))).into_response()
            }
        }
    }
}
