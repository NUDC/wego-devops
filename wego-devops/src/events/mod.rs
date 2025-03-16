//! 双向事件通信

use anyhow::Context;
use std::sync::LazyLock;
use tokio::{
    fs,
    sync::broadcast::{channel, Sender},
};

pub use dispatcher::*;
pub use ws::*;

mod dispatcher;
mod ws;

pub static CHAT: LazyLock<Sender<Payload>> = LazyLock::new(|| {
    let (chat, _) = channel::<Payload>(32);
    chat
});

// 事件通知
pub fn publish(id: i64, event_name: &str, args: &str) -> anyhow::Result<()> {
    let chat = CHAT.clone();
    chat.send(Payload {
        id,
        event_name: event_name.to_string(),
        args: args.to_string(),
    })?;
    Ok(())
}

// 监听事件
pub async fn listen() {
    tracing::info!("events start listen now!");

    // 实时读取日志返回
    let _ = on("getLog", |id, args| async move {
        tracing::info!("开始读取日志{:?}", args);
        let filepath = args.to_string();
        let contents = fs::read(filepath.clone())
            .await
            .with_context(|| format!("Failed to read file: {}", filepath))?;

        let text = String::from_utf8(contents)
            .with_context(|| format!("Failed to convert file content to UTF-8: {}", filepath))?;
        _ = publish(id, "logData", &text);
        Ok(())
    })
    .await;
}
