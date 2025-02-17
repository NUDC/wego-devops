//! ws通讯
use axum::{
    extract::{ws, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().route("/ws", get(handler)).with_state(state)
}

async fn handler(
    State(state): State<Arc<AppState>>,
    upgrade: ws::WebSocketUpgrade,
) -> impl IntoResponse {
    upgrade
        .on_failed_upgrade(|err| tracing::warn!("websocket conn error:{}", err))
        .on_upgrade(move |sockect| websocket(state, sockect))
}

async fn websocket(state: Arc<AppState>, socket: ws::WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    let tx = state.chat.clone();
    let mut rx = tx.subscribe();

    // 发送消息
    tokio::spawn(async move {
        while let Ok(payload) = rx.recv().await {
            let json = match serde_json::to_string(&payload) {
                Ok(data) => data,
                Err(err) => {
                    tracing::warn!("发送消息失败：{err}");
                    continue;
                }
            };

            match sender.send(ws::Message::Text(json.clone().into())).await {
                Ok(_) => tracing::debug!("发送消息成功{:?}", json),
                Err(err) => {
                    tracing::warn!("发送消息失败：{err}");
                    break;
                }
            }
        }
    });

    // 接收消息
    while let Some(res) = receiver.next().await {
        let msg = match res {
            Ok(data) => data,
            Err(err) => {
                tracing::warn!("websocket读取消息失败:{err}");
                break;
            }
        };
        let payload = match msg {
            ws::Message::Text(text) => serde_json::from_slice::<Payload>(text.as_bytes())
                .unwrap_or_else(|err| {
                    tracing::warn!("解析消息错误：{err}");
                    Payload::default()
                }),
            ws::Message::Close(_) => break,
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Binary(_) => todo!(),
        };
        tracing::info!("解析数据：{:?}", payload);
        let _ = tx.send(payload);
    }
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub event_name: String,
    pub args: Option<String>,
}
