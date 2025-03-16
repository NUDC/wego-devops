use std::sync::Arc;

use axum::Router;

use crate::{events, AppState};

mod health;
mod project;
mod server;
mod static_service;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        // 页面
        .merge(static_service::routes())
        // 接口
        .nest(
            "/api",
            Router::new()
                .nest("/health", health::routes())
                .nest("/event", events::routes(state.clone()))
                .nest("/server", server::routes(state.clone()))
                .nest("/project", project::routes(state.clone())),
        )
}
