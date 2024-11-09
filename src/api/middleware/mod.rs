use axum::{extract::Request, middleware::Next, response::Response};

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
