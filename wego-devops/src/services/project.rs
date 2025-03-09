use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::{fs, task};

use crate::{settings, store};

use super::{server, ssh};

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub remark: String,
    pub name: String,
    pub group: Option<String>,
    pub build_script: String,
    pub deploy: Vec<Deploy>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    pub ip: String,
    pub deploy_script: String,
}

/// 获取项目yaml配置
fn get_project_yaml(project_name: &str, group_name: Option<String>) -> anyhow::Result<PathBuf> {
    let g_name = group_name.unwrap_or("default".to_string());
    settings::create_file_dir(|o| {
        o.join("projects")
            .join(g_name.clone())
            .join(project_name)
            .join("index.yml")
    })
}

/// 获取项目源代码文件夹
fn get_src_dir(project_name: &str, group_name: Option<String>) -> anyhow::Result<PathBuf> {
    let g_name = group_name.unwrap_or("default".to_string());
    settings::create_dir(|o| {
        o.join("projects")
            .join(g_name.clone())
            .join(project_name)
            .join("src")
    })
}
/// 获取项目日志文件夹
fn get_logs_dir(project_name: &str, group_name: Option<String>) -> anyhow::Result<PathBuf> {
    let g_name = group_name.unwrap_or("default".to_string());
    settings::create_dir(|o| {
        o.join("projects")
            .join(g_name.clone())
            .join(project_name)
            .join("logs")
    })
}

/// 获取日志文件列表
pub async fn get_log_files(
    project_name: &str,
    group_name: Option<String>,
) -> anyhow::Result<Vec<PathBuf>> {
    let log_dir = get_logs_dir(project_name, group_name)?;
    let mut entries = fs::read_dir(log_dir).await?;
    let mut pathes = vec![];

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        pathes.push(path);
    }
    pathes.sort_by(|a, b| b.cmp(a));
    Ok(pathes)
}

/// 读取项目配置
pub async fn get_config(
    project_name: &str,
    group_name: Option<String>,
) -> anyhow::Result<ProjectConfig> {
    let path = get_project_yaml(project_name, group_name)?;
    let config: ProjectConfig = match store::read(&path).await {
        Ok(o) => o,
        Err(_) => ProjectConfig {
            ..Default::default()
        },
    };
    Ok(config)
}

impl ProjectConfig {
    /// 保存配置
    pub async fn save(&self) -> anyhow::Result<()> {
        let path = get_project_yaml(&self.name, self.group.clone())?;
        store::write(&path, self).await
    }
    /// 创建日志文件
    pub fn create_log_file(&self) -> anyhow::Result<String> {
        let path = get_logs_dir(&self.name, self.group.clone())?;
        let log_name = format!("{}.log", chrono::Local::now().format("%Y%m%d%-H%M%S"));
        let log_file_path = path.join(log_name);
        let log_file = format!("{:?}", log_file_path);
        Ok(log_file)
    }
    /// 构建
    async fn run_build(&self, log: Option<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        let dir = get_src_dir(&pro.name, pro.group.clone())?;
        let log_file = match log {
            Some(o) => o,
            None => self.create_log_file()?,
        };
        ssh::run_local_shell(&pro.build_script, &log_file, &format!("{:?}", dir)).await?;
        Ok(())
    }
    /// 部署
    async fn run_deploy(&self, ip: Option<String>, log: Option<String>) -> anyhow::Result<()> {
        let log_file = match log {
            Some(o) => o,
            None => self.create_log_file()?,
        };
        let deploies = match ip {
            Some(o) => self
                .deploy
                .iter()
                .filter(|oo| oo.ip == o)
                .cloned()
                .collect(),
            None => self.deploy.clone(),
        };
        for item in deploies {
            let Some(serve) = server::get_by_ip(&item.ip).await? else {
                return Err(anyhow::anyhow!("not found server: {}", &item.ip));
            };
            let host = format!("{}@{}", serve.username, serve.ip);
            ssh::run_remote_shell(&host, &item.deploy_script, &log_file).await?;
        }
        Ok(())
    }

    /// 构建
    pub fn build(&self, log: Option<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        task::spawn(async move {
            _ = pro.run_build(log).await;
        });
        Ok(())
    }
    /// 部署
    pub async fn deploy(&self, ip: Option<String>, log: Option<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        task::spawn(async move {
            _ = pro.run_deploy(ip, log).await;
        });
        Ok(())
    }
    /// 执行构建+部署
    pub async fn run(&self, ip: Option<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        let log = pro.create_log_file()?;
        task::spawn(async move {
            if pro.run_build(Some(log.clone())).await.is_err() {
                return;
            }
            _ = pro.run_deploy(ip, Some(log.clone())).await;
        });
        Ok(())
    }
}
