use crate::middleware::debug_logger;
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};

mod auth;
mod campaign;
mod contacts;
mod dashboard;
mod user;

pub(crate) fn router() -> Router<()> {
    let user_router = Router::new().route("/", get(user::GET));
    // Api handle the auth and security
    let auth_router = Router::new()
        .route("/sign-in", post(auth::sign_in::POST))
        .route("/sign-up", post(auth::sign_up::POST))
        .route("/reset-password", post(auth::reset_password::POST));

    // Delegate the work to the nextcamp crates
    let campaign_router = Router::new().route("/", get(campaign::GET));
    let contacts_router = Router::new().route("/", get(contacts::GET));
    let dashboard_router = Router::new().route("/", get(dashboard::GET));

    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/user", user_router)
                .nest("/authenticate", auth_router)
                .nest("/contacts", contacts_router)
                .nest("/campaign", campaign_router)
                .nest("/dashboard", dashboard_router),
        )
        .layer(from_fn(debug_logger))
}
