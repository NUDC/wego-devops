//! 健康检查接口

use axum::{routing::*, Router};

use crate::Result;

pub fn routes() -> Router {
    Router::new().route("/check", get(check).post(check))
}

/// 健康检查
async fn check() -> Result<()> {
    Ok(()).into()
}
