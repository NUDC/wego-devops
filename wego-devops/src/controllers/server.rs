//! 服务器管理接口

use std::sync::Arc;

use axum::{routing::post, Json, Router};

use crate::{
    services::{self, *},
    AppState, Result,
};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/getall", post(get_all))
        .route("/remove", post(del))
        .route("/save", post(save))
        .with_state(state)
}

async fn get_all() -> Result<Vec<Server>> {
    let list = services::server::get_all().await?;
    Ok(list).into()
}

async fn save(Json(dto): Json<Server>) -> Result<()> {
    services::server::add(dto).await?;
    Ok(()).into()
}
async fn del(Json(dto): Json<Vec<String>>) -> Result<()> {
    services::server::del(dto).await?;
    Ok(()).into()
}
