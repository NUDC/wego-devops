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
pub fn get_root_path() -> String {
    get_value("RootPath")
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
