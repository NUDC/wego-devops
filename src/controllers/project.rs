//! 项目管理接口

use std::sync::Arc;

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

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
