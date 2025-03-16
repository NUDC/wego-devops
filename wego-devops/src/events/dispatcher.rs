use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

// 全局事件处理器存储（懒加载）
type AsyncHandler = Arc<dyn Fn(i64, String) -> Result<()> + Send + Sync>;

static HANDLERS: LazyLock<Arc<Mutex<HashMap<String, AsyncHandler>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

// 注册事件处理器
pub async fn on<F, Fut>(event_name: &str, handler: F) -> Result<()>
where
    F: Fn(i64, String) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    let mut handlers = HANDLERS.lock().await;

    let async_handler: AsyncHandler = Arc::new(move |id: i64, args: String| {
        let future = handler(id, args);
        tokio::spawn(async move {
            if let Err(e) = future.await {
                eprintln!("Handler error: {}", e);
            }
        });
        Ok(())
    });

    handlers.insert(event_name.to_string(), async_handler);
    Ok(())
}

// 触发事件
pub async fn emit(id: i64, event_name: &str, data: &str) -> Result<()> {
    let handlers = HANDLERS.lock().await;

    if let Some(handler) = handlers.get(event_name) {
        handler(id, data.to_string())?;
    }
    Ok(())
}
