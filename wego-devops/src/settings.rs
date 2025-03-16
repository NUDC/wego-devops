use std::{fs, path::PathBuf};

pub fn create_dir<F>(func: F) -> anyhow::Result<PathBuf>
where
    F: Fn(PathBuf) -> PathBuf,
{
    let root_path = PathBuf::new().join(get_value("RootPath"));
    // 获取绝对路径
    let root_path = if root_path.starts_with(".") {
        let current_dir = std::env::current_dir()?;
        current_dir.join(root_path)
    } else {
        root_path
    };

    let dir = func(root_path);
    if dir.exists() == false {
        fs::create_dir_all(dir.clone())?;
    }
    Ok(dir)
}
pub fn create_file_dir<F>(func: F) -> anyhow::Result<PathBuf>
where
    F: Fn(PathBuf) -> PathBuf,
{
    let root_path = PathBuf::new().join(get_value("RootPath"));
    let dir = func(root_path);
    let Some(path) = dir.parent() else {
        return Err(anyhow::anyhow!("not found parent dir:{:?}", dir));
    };
    if path.exists() == false {
        fs::create_dir_all(path)?;
    }
    Ok(dir)
}

pub fn init() {
    dotenvy::dotenv().ok();
}
pub fn get_value(key: &str) -> String {
    let Ok(value) = dotenvy::var(key) else {
        tracing::error!("{}不存在，请添加配置", key);
        return String::new();
    };
    value
}
pub fn host() -> String {
    get_value("HOST")
}
pub fn log() -> String {
    get_value("LOG")
}
pub fn get_shell_env() -> String {
    get_value("SHELL")
}
pub fn get_web_static() -> String {
    get_value("WEB")
}

pub fn get_shell_timeout() -> u64 {
    let timeout = get_value("SHELL_TIMEOUT");
    match timeout.parse::<u64>() {
        Ok(num) => num,
        Err(_) => 30,
    }
}
