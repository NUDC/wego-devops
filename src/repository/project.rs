use std::{collections::HashMap, path::PathBuf, time::Duration};

use tokio::{fs, process::Command, task, time::timeout};

use crate::settings;

use super::{
    create_dir, get_projects_dir, store, ProjectConfig, ProjectIndex, ProjectInfo, ProjectStatus,
};

async fn get_yaml() -> anyhow::Result<PathBuf> {
    let dir = get_projects_dir().await?;
    Ok(dir.join("index.yml"))
}

/// 获取所有项目
pub async fn get_all() -> anyhow::Result<Vec<ProjectIndex>> {
    let path = get_yaml().await?;
    let list: Vec<ProjectIndex> = match store::read(&path).await {
        Ok(o) => o,
        Err(_) => vec![],
    };
    Ok(list)
}

/// 根据name获取项目
pub async fn get_by_name(name: &str) -> anyhow::Result<Option<ProjectIndex>> {
    let all = get_all().await?;
    let model = all.iter().find(|o| o.name == name).cloned();
    Ok(model)
}

/// 去重
fn distinct(list: Vec<ProjectIndex>) -> Vec<ProjectIndex> {
    let mut all: Vec<ProjectIndex> = vec![];
    let mut dict: HashMap<String, ()> = HashMap::new();
    for item in list {
        let id = item.name.clone();
        if dict.contains_key(&id) {
            continue;
        }
        dict.insert(id, ());
        all.push(item);
    }
    all
}
/// 保存
async fn set(list: Vec<ProjectIndex>) -> anyhow::Result<()> {
    let path = get_yaml().await?;
    if list.is_empty() {
        return store::del(&path).await;
    }
    store::write(&path, distinct(list)).await
}

/// 添加
pub async fn add(model: ProjectIndex) -> anyhow::Result<()> {
    let all = get_all().await?;
    set([all, vec![model]].concat()).await
}
/// 删除
pub async fn del(names: Vec<String>) -> anyhow::Result<()> {
    let mut list = get_all().await?;
    list = list
        .iter()
        .filter(|o| names.contains(&o.name) == false)
        .cloned()
        .collect();

    set(list).await
}

impl ProjectIndex {
    /// 获取项目根目录
    async fn get_dir(&self) -> anyhow::Result<PathBuf> {
        let r_path = get_projects_dir().await?;
        let dir = r_path.join(self.name.clone());
        create_dir(dir.clone()).await?;
        Ok(dir)
    }
    async fn get_children_dir(&self, name: &str) -> anyhow::Result<PathBuf> {
        let r_path = self.get_dir().await?;
        let dir = r_path.join(name);
        create_dir(dir.clone()).await?;
        Ok(dir)
    }
    /// 获取项目源代码目录
    async fn get_src_dir(&self) -> anyhow::Result<PathBuf> {
        self.get_children_dir("src").await
    }

    /// 获取项目配置文件路径
    async fn get_yaml(&self) -> anyhow::Result<PathBuf> {
        let dir = self.get_dir().await?;
        Ok(dir.join("project.yml"))
    }
    /// 读取项目配置
    async fn get_config(&self) -> anyhow::Result<ProjectConfig> {
        let path = self.get_yaml().await?;
        let config: ProjectConfig = match store::read(&path).await {
            Ok(o) => o,
            Err(_) => ProjectConfig {
                ..Default::default()
            },
        };
        Ok(config)
    }
    /// 设置项目配置
    async fn set_config(&self, config: ProjectConfig) -> anyhow::Result<()> {
        let path = self.get_yaml().await?;
        store::write(&path, config).await
    }

    /// 获取日志目录
    async fn get_logs_dir(&self) -> anyhow::Result<PathBuf> {
        self.get_children_dir("logs").await
    }
    /// 创建日志文件
    async fn create_log_file(&self) -> anyhow::Result<String> {
        let logs_dir = self.get_logs_dir().await?;
        let log_name = format!("{}.log", chrono::Local::now().format("%Y%m%d%H%M%S"));
        let log_file_path = logs_dir.join(log_name);
        let log_file = format!("{:?}", log_file_path);
        Ok(log_file)
    }
    /// 获取日志文件列表
    pub async fn get_log_files(&self) -> anyhow::Result<Vec<PathBuf>> {
        let log_dir = self.get_logs_dir().await?;
        let mut entries = fs::read_dir(log_dir).await?;
        let mut pathes = vec![];

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            pathes.push(path);
        }
        pathes.sort_by(|a, b| b.cmp(a));
        Ok(pathes)
    }

    /// 获取构建脚本文件路径
    async fn get_build_file(&self) -> anyhow::Result<PathBuf> {
        let path = self.get_dir().await?;
        Ok(path.join("build.sh"))
    }
    /// 编辑构建脚本
    async fn set_build(&self, contents: &str) -> anyhow::Result<()> {
        let path = self.get_build_file().await?;
        fs::write(path, contents).await?;
        Ok(())
    }
    /// 读取构建脚本
    async fn get_build(&self) -> anyhow::Result<String> {
        let path = self.get_build_file().await?;
        let contents = fs::read_to_string(path).await?;
        Ok(contents)
    }

    /// 获取部署脚本文件路径
    async fn get_deploy_file(&self) -> anyhow::Result<PathBuf> {
        let path = self.get_dir().await?;
        Ok(path.join("deploy.sh"))
    }
    /// 编辑部署脚本
    async fn set_deploy(&self, contents: &str) -> anyhow::Result<()> {
        let path = self.get_deploy_file().await?;
        fs::write(path, contents).await?;
        Ok(())
    }
    /// 读取部署脚本
    async fn get_deploy(&self) -> anyhow::Result<String> {
        let path = self.get_deploy_file().await?;
        let contents = fs::read_to_string(path).await?;
        Ok(contents)
    }
    /// 执行构建脚本
    async fn build(&self, log_file: &str) -> anyhow::Result<()> {
        let build_file = self.get_build_file().await?;
        let script_file = format!("{:?}", build_file);
        let src = self.get_src_dir().await?;
        let dir = format!("{:?}", src);
        run_shell(&script_file, log_file, &dir).await
    }
    /// 执行部署脚本
    async fn deploy(&self, log_file: &str) -> anyhow::Result<()> {
        let deploy_file = self.get_deploy_file().await?;
        let script_file = format!("{:?}", deploy_file);
        let src = self.get_src_dir().await?;
        let dir = format!("{:?}", src);
        run_shell(&script_file, &log_file, &dir).await
    }
    /// 执行构建+部署脚本
    async fn run_scripts(&self) -> anyhow::Result<String> {
        let log_file = self.create_log_file().await?;
        self.build(&log_file).await?;
        self.deploy(&log_file).await?;
        Ok(log_file)
    }
    /// 执行构建+部署脚本
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut project = self.clone();
        project.build_time = Some(chrono::Local::now().naive_local());
        project.status = ProjectStatus::Running;
        add(project.clone()).await?;

        task::spawn(async move {
            project.status = match project.run_scripts().await {
                Ok(_) => ProjectStatus::Success,
                Err(_) => ProjectStatus::Error,
            };
            // 修改为执行完成状态
            _ = add(project.clone()).await;
        });

        Ok(())
    }
    /// 获取项目详情
    pub async fn get(&self) -> anyhow::Result<ProjectInfo> {
        let buid_script = self.get_build().await?;
        let depoly_script = self.get_deploy().await?;
        let model = ProjectInfo {
            name: self.name.clone(),
            remark: self.remark.clone(),
            build_script: buid_script,
            deploy_script: depoly_script,
        };
        Ok(model)
    }

    /// 保存项目详情
    pub async fn set(&self, info: ProjectInfo, is_new: bool) -> anyhow::Result<()> {
        if self.remark != info.remark {
            let mut project = self.clone();
            project.remark = info.remark.clone();
            add(project).await?;
        }
        if is_new || self.get_build().await? != info.build_script {
            self.set_build(&info.build_script).await?;
        }
        if is_new || self.get_deploy().await? != info.deploy_script {
            self.set_deploy(&info.deploy_script).await?;
        }

        Ok(())
    }
}

async fn run_shell(script_file: &str, log_file: &str, dir: &str) -> anyhow::Result<()> {
    let shell_command = format!("cd {} && sh -xe {} >> {} 2>&1", dir, script_file, log_file);
    tracing::info!("执行脚本：{}", shell_command);

    timeout(Duration::from_secs(settings::get_shell_timeout()), async {
        Command::new(settings::get_shell_env())
            .arg("-c")
            .arg(&shell_command)
            .status()
            .await
    })
    .await??;

    tracing::info!("finished:{}", script_file);
    Ok(())
}
