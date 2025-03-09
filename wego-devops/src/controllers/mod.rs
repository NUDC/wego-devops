use std::sync::Arc;

use axum::Router;

use crate::AppState;
pub(super) use chathub::Payload;

mod chathub;
mod health;
mod project;
mod server;
mod static_service;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().merge(static_service::routes()).nest(
        "/api",
        Router::new()
            .nest("/health", health::routes())
            .nest("/event", chathub::routes(state.clone()))
            .nest("/server", server::routes(state.clone()))
            .nest("/project", project::routes(state.clone())),
    )
}
