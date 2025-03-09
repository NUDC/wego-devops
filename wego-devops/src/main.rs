#![feature(try_trait_v2)]
#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use tokio::sync::broadcast::channel;
use tokio::{signal, sync::broadcast::Sender};

use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use result::Result;
pub use serializer::{datetime_format, datetime_option_format};
mod controllers;
mod result;
mod serializer;
mod services;
mod settings;
mod store;

#[tokio::main]
async fn main() {
    settings::init();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from(settings::log()))
        .init();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .merge(controllers::routes(state.clone()))
        .layer(tower_http::compression::CompressionLayer::new().gzip(true))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
            3,
        )))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );

    let listener = tokio::net::TcpListener::bind(settings::host())
        .await
        .unwrap();

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    tracing::info!("关闭");
}

#[derive(Clone)]
pub struct AppState {
    pub chat: Sender<controllers::Payload>,
}

impl AppState {
    fn new() -> AppState {
        let (chat, _) = channel::<controllers::Payload>(32);

        Self { chat }
    }
}
