use axum::{extract::Request, middleware::Next, response::Response};

#[inline]
pub(crate) async fn debug_logger(req: Request, next: Next) -> Response {
    #[cfg(not(debug_assertions))]
    {
        return next.run(req).await;
    }

    #[cfg(debug_assertions)]
    {
        let (parts, body) = req.into_parts();
        tracing::info!("Got a request with parts: {:#?}", parts);
        next.run(Request::from_parts(parts, body)).await
    }
}
