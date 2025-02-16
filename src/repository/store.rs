//! 配置存储

use std::{path::PathBuf, sync::LazyLock};

use tokio::{fs, io::AsyncWriteExt};

use crate::settings;

/// 本地存储地址
pub const ROOT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(settings::get_root_path()));

pub async fn del(path: &PathBuf) -> anyhow::Result<()> {
    fs::remove_file(&path).await?;
    Ok(())
}

/// 读取文件
pub async fn read<T>(path: &PathBuf) -> anyhow::Result<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    let json = fs::read(path).await?;
    let list: T = serde_yaml::from_slice(json.as_slice())?;
    Ok(list)
}

/// 写入文件
pub async fn write<T>(path: &PathBuf, model: T) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let yaml = serde_yaml::to_string(&model)?;
    fs::write(path, yaml).await?;
    Ok(())
}

/// 添加
pub async fn write_append<T>(path: &PathBuf, model: T) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let yaml = serde_yaml::to_string(&model)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write(yaml.as_bytes()).await?;
    Ok(())
}
