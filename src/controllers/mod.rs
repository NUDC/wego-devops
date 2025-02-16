use std::sync::Arc;

use axum::Router;

use crate::AppState;
pub(super) use chathub::Payload;

mod chathub;
mod health;
mod project;
mod static_service;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(static_service::routes())
        .merge(chathub::routes(state.clone()))
        .merge(project::routes(state.clone()))
}
