use std::path::PathBuf;

use anyhow::Context;
use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use tokio::fs;

use crate::serializer::*;

pub mod project;
mod store;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndex {
    pub name: String,
    pub remark: String,
    pub status: ProjectStatus,
    #[serde(with = "datetime_option_format")]
    pub build_time: Option<DateTime>,
    #[serde(with = "datetime_format")]
    pub created: DateTime,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ProjectStatus {
    Default = 0,
    Success = 1,
    Error = 2,
    Running = 3,
}
impl Default for ProjectStatus {
    fn default() -> Self {
        Self::Default
    }
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
