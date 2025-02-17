//! 项目管理接口

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    models,
    repository::{self, *},
    AppState,
};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/getprojects", post(get_projects))
        .route("/delproject", post(del_project))
        .route("/getprojectinfo", post(get_project_info))
        .route("/setprojectinfo", post(set_project_info))
        .route("/run", post(run))
        .route("/getlogs", post(get_log_all))
        .route("/getlog", post(get_log))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AddProject {
    pub name: String,
    pub remark: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct EditScript {
    pub name: String,
    pub contents: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDto {
    pub name: String,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectLog {
    pub path: String,
}

/// 获取所有项目
async fn get_projects() -> models::Result<Vec<ProjectIndex>> {
    let list = repository::project::get_all().await?;
    Ok(list).into()
}
/// 删除项目
async fn del_project(Json(dto): Json<Vec<String>>) -> models::Result<()> {
    repository::project::del(dto).await?;
    Ok(()).into()
}
/// 获取项目信息
async fn get_project_info(Json(dto): Json<ProjectDto>) -> models::Result<ProjectInfo> {
    let Some(project) = repository::project::get_by_name(&dto.name).await? else {
        return anyhow::anyhow!("not found").into();
    };
    project.get().await.into()
}
/// 编辑项目信息
async fn set_project_info(Json(dto): Json<ProjectInfo>) -> models::Result<()> {
    match repository::project::get_by_name(&dto.name).await? {
        Some(o) => o.set(dto, false).await?,
        None => {
            let o = ProjectIndex {
                name: dto.name.clone(),
                remark: dto.remark.clone(),
                created: chrono::Local::now().naive_local(),
                ..Default::default()
            };
            repository::project::add(o.clone()).await?;
            o.set(dto, true).await?
        }
    };

    Ok(()).into()
}
/// 执行脚本
async fn run(Json(dto): Json<ProjectDto>) -> models::Result<()> {
    let Some(project) = repository::project::get_by_name(&dto.name).await? else {
        return anyhow::anyhow!("not found").into();
    };
    project.run().await?;
    Ok(()).into()
}

/// 获取日志
async fn get_log_all(Json(dto): Json<ProjectDto>) -> models::Result<Vec<PathBuf>> {
    let Some(project) = repository::project::get_by_name(&dto.name).await? else {
        return anyhow::anyhow!("not found").into();
    };
    let files = project.get_log_files().await?;
    Ok(files).into()
}

/// 读取日志
async fn get_log(Json(dto): Json<ProjectLog>) -> models::Result<String> {
    let contents = fs::read(&dto.path)
        .await
        .with_context(|| format!("Failed to read file: {}", dto.path))?;

    // 将字节数据转换为字符串
    let text = String::from_utf8(contents)
        .with_context(|| format!("Failed to convert file content to UTF-8: {}", dto.path))?;

    Ok(text).into()
}
