//! 健康检查接口

use axum::{routing::*, Router};

use crate::models;

pub fn routes() -> Router {
    Router::new().route("/health", get(check).post(check))
}

/// 健康检查
async fn check() -> models::Result<()> {
    Ok(()).into()
}
