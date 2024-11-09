use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};


#[derive(new, Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    key: String,
    delay: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Config {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Config>()
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
