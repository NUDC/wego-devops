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
        .route("/removeLog", post(remove_log))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDeployDto {
    pub name: String,
    pub group: String,
    pub codes: Vec<String>,
}
impl ProjectDeployDto {
    pub fn get_id(&self) -> ProjectUniqueId {
        ProjectUniqueId {
            group: self.group.clone(),
            name: self.name.clone(),
        }
    }
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
    let data = project::get_config(&dto).await?;
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
    let project = services::project::get_config(&dto).await?;
    project.build(None)?;
    Ok(()).into()
}
/// 执行部署脚本
async fn deploy(Json(dto): Json<ProjectDeployDto>) -> Result<()> {
    let project = services::project::get_config(&dto.get_id()).await?;
    project.deploy(dto.codes, None).await?;
    Ok(()).into()
}
/// 执行所有脚本
async fn run(Json(dto): Json<ProjectDeployDto>) -> Result<()> {
    let project = services::project::get_config(&dto.get_id()).await?;
    project.run(dto.codes).await?;
    Ok(()).into()
}

/// 获取日志列表
async fn get_log_all(Json(dto): Json<ProjectUniqueId>) -> Result<Vec<PathBuf>> {
    let files = services::project::get_log_files(&dto).await?;
    Ok(files).into()
}

/// 删除日志
async fn remove_log(Json(dto): Json<ProjectLog>) -> Result<()> {
    fs::remove_file(dto.path)
        .await
        .with_context(|| "删除日志失败")?;

    Ok(()).into()
}
