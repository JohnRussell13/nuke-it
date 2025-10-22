use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use std::ops::ControlFlow;
use std::{net::SocketAddr, sync::Arc};

use futures_util::sink::SinkExt;

use crate::service::types::{ClientPayload, ResponsePayload, ServerMessage};
use crate::types::SharedClients;

mod fetch;
mod spin;
mod types;

pub async fn process_message(
    msg: Message,
    who: SocketAddr,
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    ws_clients: SharedClients,
    db_client: Arc<Mutex<Client>>,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(package) => {
            dispatch(package, sender, db_client.clone()).await;
            broadcast(&ws_clients, fetch::run(db_client.clone()).await).await;
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

async fn dispatch(
    package: Utf8Bytes,
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    db_client: Arc<Mutex<Client>>,
) {
    let req_raw = package.as_str();
    let req_json: Result<ClientPayload, _> = serde_json::from_str(req_raw);

    let req_json = match req_json {
        Ok(m) => m,
        Err(e) => {
            let err_msg = types::create_error(format!("Bad message format: {e:?}"));
            let data = serde_json::to_string(&err_msg).unwrap();
            send(sender, data).await;
            return;
        }
    };

    let res_json = match req_json {
        ClientPayload::Spin {
            wallet_id,
            game_id,
            player_id,
            amount,
        } => spin::run(wallet_id, game_id, player_id, amount, db_client).await,
        ClientPayload::Fetch {} => return,
    };

    let res_raw = match serde_json::to_string(&res_json) {
        Ok(json) => json,
        Err(_) => {
            let err_msg = types::create_error(String::from("Server error!"));
            let data = serde_json::to_string(&err_msg).unwrap();
            send(sender, data).await;
            return;
        }
    };

    send(sender, res_raw).await;
}

async fn send(sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>, data: String) {
    let mut locked_sender = sender.lock().await;
    if locked_sender
        .send(Message::Text(data.into()))
        .await
        .is_err()
    {
        eprintln!("Error sending message to");
    }
}

async fn broadcast(ws_clients: &SharedClients, res_json: ServerMessage<ResponsePayload>) {
    let res_raw = match serde_json::to_string(&res_json) {
        Ok(json) => json,
        Err(_) => {
            return;
        }
    };

    let locked = ws_clients.lock().await;

    for sender in locked.iter() {
        let mut sink = sender.lock().await;
        let _ = sink.send(Message::Text(res_raw.clone().into())).await;
    }
}
