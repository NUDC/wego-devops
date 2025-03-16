//! ws通讯
use axum::{
    extract::{ws, Path},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{events, AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws/{id}", get(handler))
        .with_state(state)
}

async fn handler(Path(id): Path<i64>, upgrade: ws::WebSocketUpgrade) -> impl IntoResponse {
    upgrade
        .on_failed_upgrade(|err| tracing::warn!("websocket conn error:{}", err))
        .on_upgrade(move |sockect| websocket(id, sockect))
}

async fn websocket(id: i64, socket: ws::WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    let tx = events::CHAT.clone();
    let mut rx = tx.subscribe();

    tracing::info!("ws id:{}", id);
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

            if payload.id != id {
                continue;
            }

            match sender.send(ws::Message::Text(json.clone().into())).await {
                Ok(_) => tracing::debug!("发送消息成功:{}", payload.event_name),
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
            ws::Message::Text(text) => serde_json::from_slice::<ReceivePayload>(text.as_bytes())
                .unwrap_or_else(|err| {
                    tracing::warn!("解析消息错误：{err}");
                    ReceivePayload::default()
                }),
            ws::Message::Close(_) => break,
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Binary(_) => todo!(),
        };

        _ = events::emit(id, &payload.event_name, &payload.args).await;
    }
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub id: i64,
    pub event_name: String,
    pub args: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReceivePayload {
    pub event_name: String,
    pub args: String,
}
