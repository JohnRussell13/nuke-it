use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;

use std::ops::ControlFlow;
use std::{net::SocketAddr, sync::Arc};

use futures_util::sink::SinkExt;

use crate::service::types::{ClientPayload, ResponsePayload};

mod fetch;
mod spin;
mod types;

pub async fn process_message(
    msg: Message,
    who: SocketAddr,
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            let package = t.as_str();
            let response = dispatch(package);

            let mut locked_sender = sender.lock().await;
            if locked_sender
                .send(Message::Text(response.into()))
                .await
                .is_err()
            {
                eprintln!("Error sending message to {who}");
            }
        }
        Message::Binary(d) => {
            println!(">>> {who} sent {} bytes: {d:?}", d.len());
        }
        Message::Close(_c) => {
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

fn dispatch(req_raw: &str) -> String {
    let req_json: Result<ClientPayload, _> = serde_json::from_str(req_raw);

    let req_json = match req_json {
        Ok(m) => m,
        Err(_) => {
            let err_msg = types::create_error("Bad message format!");
            return serde_json::to_string(&err_msg).unwrap();
        }
    };

    let res_json = match req_json {
        ClientPayload::Spin { id } => types::ok(ResponsePayload::Spin(spin::run(id))),
        ClientPayload::Fetch {} => types::ok(ResponsePayload::Fetch(fetch::run())),
    };

    let res_raw = match serde_json::to_string(&res_json) {
        Ok(json) => json,
        Err(_) => {
            let err_msg = types::create_error("Server error!");
            return serde_json::to_string(&err_msg).unwrap();
        }
    };

    res_raw
}
