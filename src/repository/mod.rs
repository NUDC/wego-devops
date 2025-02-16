use std::path::PathBuf;

use anyhow::Context;
use tokio::fs;

use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};

use crate::serializer::*;

pub mod project;
mod store;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndex {
    pub name: String,
    pub remark: String,
    pub status: i32,
    #[serde(with = "datetime_option_format")]
    pub build_time: Option<DateTime>,
    #[serde(with = "datetime_format")]
    pub created: DateTime,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub name: String,
    pub remark: String,
    pub build_script: String,
    pub deploy_script: String,
}

/// 检查根路径是否存在，如果不存在则创建
async fn create_dir(dir: PathBuf) -> anyhow::Result<()> {
    let path = dir.as_path();
    if fs::try_exists(path).await? {
        return Ok(());
    }
    fs::create_dir_all(path)
        .await
        .with_context(|| format!("Failed to create directory: {:?}", dir))?;
    Ok(())
}

async fn get_projects_dir() -> anyhow::Result<PathBuf> {
    let dir = store::ROOT_PATH.join("projects");
    create_dir(dir.clone()).await?;
    Ok(dir)
}
