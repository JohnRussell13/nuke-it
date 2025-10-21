use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
};
use axum_extra::TypedHeader;
use tokio::sync::Mutex;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

use axum::extract::connect_info::ConnectInfo;

use futures_util::SinkExt;
use futures_util::stream::{SplitSink, StreamExt};

mod service;

type Tx = Arc<Mutex<SplitSink<WebSocket, Message>>>;
type SharedClients = Arc<Mutex<Vec<Tx>>>;

#[tokio::main]
async fn main() {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let clients: SharedClients = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", {
            let clients = clients.clone();
            any(move |ws, ua, info| ws_handler(ws, ua, info, clients.clone()))
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    _user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    clients: SharedClients,
) -> impl IntoResponse {
    println!("{addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, clients))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, clients: SharedClients) {
    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(Mutex::new(sender));

    {
        let mut locked = clients.lock().await;
        locked.push(sender.clone());
    }

    while let Some(Ok(msg)) = receiver.next().await {
        let sender_clone = sender.clone();
        let msg_clone = msg.clone();

        if service::process_message(msg_clone, who, &sender_clone)
            .await
            .is_break()
        {
            break;
        }
    }

    {
        let mut locked = clients.lock().await;
        locked.retain(|s| !Arc::ptr_eq(s, &sender));
    }

    println!("WebSocket {who} disconnected");
}

async fn broadcast(clients: &SharedClients, msg: &str) {
    let locked = clients.lock().await;

    for sender in locked.iter() {
        let mut sink = sender.lock().await;
        let _ = sink.send(Message::Text(msg.to_string().into())).await;
    }
}
