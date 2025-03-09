//! 项目管理接口

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    services::{self, *},
    AppState, Result,
};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/getall", post(get_projects))
        .route("/remove", post(del_project))
        .route("/getinfo", post(get_project_info))
        .route("/save", post(set_project_info))
        .route("/build", post(build))
        .route("/deploy", post(deploy))
        .route("/run", post(run))
        .route("/getlogs", post(get_log_all))
        .route("/getlog", post(get_log))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDeployDto {
    pub name: String,
    pub group: Option<String>,
    pub ip: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectLog {
    pub path: String,
}

/// 获取所有项目
async fn get_projects() -> Result<Vec<ProjectIndex>> {
    let list = services::group::get_all().await?;
    Ok(list).into()
}
/// 删除项目
async fn del_project(Json(dto): Json<Vec<ProjectUniqueId>>) -> Result<()> {
    services::group::del(dto).await?;
    Ok(()).into()
}
/// 获取项目信息
async fn get_project_info(Json(dto): Json<ProjectUniqueId>) -> Result<ProjectConfig> {
    let Some(project) = services::group::get_by_id(dto).await? else {
        return anyhow::anyhow!("not found").into();
    };
    let data = project::get_config(&project.name, project.group).await?;
    Ok(data).into()
}
/// 编辑项目信息
async fn set_project_info(Json(dto): Json<ProjectConfig>) -> Result<()> {
    match services::group::get_by_id(ProjectUniqueId {
        group: dto.group.clone(),
        name: dto.name.clone(),
    })
    .await?
    {
        Some(o) => {
            services::group::add(o).await?;
            dto.save().await?;
        }
        None => {
            let o = ProjectIndex {
                name: dto.name.clone(),
                group: dto.group.clone(),
                remark: dto.remark.clone(),
                created: chrono::Local::now().naive_local(),
                ..Default::default()
            };
            services::group::add(o.clone()).await?;
            dto.save().await?;
        }
    };

    Ok(()).into()
}

/// 执行构建脚本
async fn build(Json(dto): Json<ProjectUniqueId>) -> Result<()> {
    let project = services::project::get_config(&dto.name, dto.group).await?;
    project.build(None)?;
    Ok(()).into()
}
/// 执行部署脚本
async fn deploy(Json(dto): Json<ProjectDeployDto>) -> Result<()> {
    let project = services::project::get_config(&dto.name, dto.group).await?;
    project.deploy(dto.ip, None).await?;
    Ok(()).into()
}
/// 执行所有脚本
async fn run(Json(dto): Json<ProjectDeployDto>) -> Result<()> {
    let project = services::project::get_config(&dto.name, dto.group).await?;
    project.run(dto.ip).await?;
    Ok(()).into()
}

/// 获取日志列表
async fn get_log_all(Json(dto): Json<ProjectUniqueId>) -> Result<Vec<PathBuf>> {
    let files = services::project::get_log_files(&dto.name, dto.group).await?;
    Ok(files).into()
}

/// 读取日志
async fn get_log(Json(dto): Json<ProjectLog>) -> Result<String> {
    let contents = fs::read(&dto.path)
        .await
        .with_context(|| format!("Failed to read file: {}", dto.path))?;

    // 将字节数据转换为字符串
    let text = String::from_utf8(contents)
        .with_context(|| format!("Failed to convert file content to UTF-8: {}", dto.path))?;

    Ok(text).into()
}
