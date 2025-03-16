use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::{fs, task};

use crate::{settings, store};

use super::{server, ssh, ProjectUniqueId};

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub remark: String,
    pub name: String,
    pub group: String,
    pub build_script: String,
    pub deploy: Vec<Deploy>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    pub code: Option<String>,
    pub ip: String,
    pub deploy_script: String,
}

/// 获取项目yaml配置
fn get_project_yaml(id: &ProjectUniqueId) -> anyhow::Result<PathBuf> {
    settings::create_file_dir(|o| {
        o.join("projects")
            .join(&id.group)
            .join(&id.name)
            .join("index.yml")
    })
}

/// 获取项目源代码文件夹
fn get_src_dir(id: &ProjectUniqueId) -> anyhow::Result<PathBuf> {
    settings::create_dir(|o| {
        o.join("projects")
            .join(&id.group)
            .join(&id.name)
            .join("src")
    })
}
/// 获取项目日志文件夹
fn get_logs_dir(id: &ProjectUniqueId) -> anyhow::Result<PathBuf> {
    settings::create_dir(|o| {
        o.join("projects")
            .join(&id.group)
            .join(&id.name)
            .join("logs")
    })
}

/// 获取日志文件列表
pub async fn get_log_files(id: &ProjectUniqueId) -> anyhow::Result<Vec<PathBuf>> {
    let log_dir = get_logs_dir(id)?;
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
pub async fn get_config(id: &ProjectUniqueId) -> anyhow::Result<ProjectConfig> {
    let path = get_project_yaml(id)?;
    let config: ProjectConfig = match store::read(&path).await {
        Ok(o) => o,
        Err(_) => ProjectConfig {
            ..Default::default()
        },
    };
    Ok(config)
}

impl ProjectConfig {
    fn get_id(&self) -> ProjectUniqueId {
        ProjectUniqueId {
            group: self.group.clone(),
            name: self.name.clone(),
        }
    }
    /// 保存配置
    pub async fn save(&self) -> anyhow::Result<()> {
        let id = self.get_id();
        get_src_dir(&id)?; // 初始化文件夹
        let path = get_project_yaml(&id)?;
        store::write(&path, self).await
    }
    /// 创建日志文件
    pub fn create_log_file(&self) -> anyhow::Result<String> {
        let id = self.get_id();
        let path = get_logs_dir(&id)?;
        let log_name = format!("{}.log", chrono::Local::now().format("%Y%m%d%H%M%S"));
        let log_file_path = path.join(log_name);
        let log_file = format!("{:?}", log_file_path);
        Ok(log_file)
    }
    /// 构建
    async fn run_build(&self, log: Option<String>) -> anyhow::Result<()> {
        let id = self.get_id();
        let pro = self.clone();
        let dir = get_src_dir(&id)?;
        let log_file = match log {
            Some(o) => o,
            None => self.create_log_file()?,
        };
        ssh::run_local_shell(&pro.build_script, &log_file, &format!("{:?}", dir)).await?;
        Ok(())
    }
    /// 部署
    async fn run_deploy(&self, code: Vec<String>, log: Option<String>) -> anyhow::Result<()> {
        let log_file = match log {
            Some(o) => o,
            None => self.create_log_file()?,
        };
        let deploies = if code.len() > 0 {
            self.deploy
                .iter()
                .filter(|oo| code.contains(&oo.code.clone().unwrap_or_default()))
                .cloned()
                .collect()
        } else {
            self.deploy.clone()
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
    pub async fn deploy(&self, codes: Vec<String>, log: Option<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        task::spawn(async move {
            _ = pro.run_deploy(codes, log).await;
        });
        Ok(())
    }
    /// 执行构建+部署
    pub async fn run(&self, codes: Vec<String>) -> anyhow::Result<()> {
        let pro = self.clone();
        let log = pro.create_log_file()?;
        task::spawn(async move {
            if pro.run_build(Some(log.clone())).await.is_err() {
                return;
            }
            _ = pro.run_deploy(codes, Some(log.clone())).await;
        });
        Ok(())
    }
}
