use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};

use crate::{settings, store};

/// 服务器配置
#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// IP地址
    pub ip: String,
    /// 账号
    pub username: String,
    /// 备注
    pub remark: String,
}

const ROOT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| settings::create_file_dir(|o| o.join("server.yml")).unwrap());

/// 去重
fn distinct(list: Vec<Server>) -> Vec<Server> {
    let mut all: Vec<Server> = vec![];
    let mut dict: HashMap<String, ()> = HashMap::new();
    for item in list {
        let id = item.ip.clone();
        if dict.contains_key(&id) {
            continue;
        }
        dict.insert(id, ());
        all.push(item);
    }
    all
}

async fn set(list: Vec<Server>) -> anyhow::Result<()> {
    if list.is_empty() {
        return store::del(&ROOT_PATH).await;
    }
    store::write(&ROOT_PATH, distinct(list)).await
}

pub async fn get_all() -> anyhow::Result<Vec<Server>> {
    let list: Vec<Server> = match store::read(&ROOT_PATH).await {
        Ok(o) => o,
        Err(_) => vec![],
    };
    Ok(list)
}

pub async fn get_by_ip(ip: &str) -> anyhow::Result<Option<Server>> {
    let all = get_all().await?;
    let model = all.iter().find(|o| o.ip == ip).cloned();
    Ok(model)
}
pub async fn add(model: Server) -> anyhow::Result<()> {
    let all = get_all().await?;
    set([vec![model], all].concat()).await
}

pub async fn del(ip_list: Vec<String>) -> anyhow::Result<()> {
    let mut list = get_all().await?;
    list = list
        .iter()
        .filter(|o| ip_list.contains(&o.ip) == false)
        .cloned()
        .collect();

    set(list).await
}
