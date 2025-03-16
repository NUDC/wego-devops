//! 项目索引

use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::{datetime_format, datetime_option_format};
use crate::{settings, store};

/// 项目索引
#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndex {
    pub group: String,
    pub name: String,
    pub remark: String,
    pub status: ProjectStatus,
    #[serde(with = "datetime_option_format")]
    pub build_time: Option<DateTime>,
    #[serde(with = "datetime_format")]
    pub created: DateTime,
}
impl ProjectIndex {
    pub fn unique_id(&self) -> String {
        format!("{}_{}", self.group, self.name)
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUniqueId {
    pub group: String,
    pub name: String,
}
impl ProjectUniqueId {
    pub fn unique_id(&self) -> String {
        format!("{}_{}", self.group, self.name)
    }
}

/// 项目状态
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

const ROOT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| settings::create_file_dir(|o| o.join("projects").join("index.yml")).unwrap());

/// 获取所有项目
pub async fn get_all() -> anyhow::Result<Vec<ProjectIndex>> {
    let list: Vec<ProjectIndex> = match store::read(&ROOT_PATH).await {
        Ok(o) => o,
        Err(_) => vec![],
    };
    Ok(list)
}

/// 根据name获取项目
pub async fn get_by_id(id: ProjectUniqueId) -> anyhow::Result<Option<ProjectIndex>> {
    let all = get_all().await?;
    let model = all
        .iter()
        .find(|o| o.unique_id() == id.unique_id())
        .cloned();
    Ok(model)
}

/// 去重
fn distinct(list: Vec<ProjectIndex>) -> Vec<ProjectIndex> {
    let mut all: Vec<ProjectIndex> = vec![];
    let mut dict: HashMap<String, ()> = HashMap::new();
    for item in list {
        let id = item.unique_id().clone();
        if dict.contains_key(&id) {
            continue;
        }
        dict.insert(id, ());
        all.push(item);
    }
    all
}

pub async fn set(list: Vec<ProjectIndex>) -> anyhow::Result<()> {
    if list.is_empty() {
        return store::del(&ROOT_PATH).await;
    }
    store::write(&ROOT_PATH, distinct(list)).await
}

pub async fn add(model: ProjectIndex) -> anyhow::Result<()> {
    let all = get_all().await?;
    set([all, vec![model]].concat()).await
}

pub async fn del(id_list: Vec<ProjectUniqueId>) -> anyhow::Result<()> {
    let mut list = get_all().await?;
    list = list
        .iter()
        .filter(|o| id_list.iter().any(|oo| oo.unique_id() == o.unique_id()) == false)
        .cloned()
        .collect();

    set(list).await
}
