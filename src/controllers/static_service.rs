//! 健康检查接口

use axum::{routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::settings;

pub fn routes() -> Router {
    let root = settings::get_web_static();
    tracing::debug!("web static dir:{}", root);
    let assets_service = get_service(ServeDir::new(format!("{}/assets", root)));
    let favicon_ico = get_service(ServeFile::new(format!("{}/favicon.ico", root)));
    let favicon_svg = get_service(ServeFile::new(format!("{}/favicon.svg", root)));
    let index_html = get_service(ServeFile::new(format!("{}/index.html", root)));

    Router::new()
        .nest_service("/assets", assets_service)
        .nest_service("/favicon.ico", favicon_ico)
        .nest_service("/favicon.svg", favicon_svg)
        .fallback_service(index_html)
}
