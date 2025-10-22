use axum::{
    Router,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
};
use axum_extra::TypedHeader;
use dotenv::dotenv;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

use axum::extract::connect_info::ConnectInfo;

use futures_util::stream::StreamExt;

use crate::types::SharedClients;

mod db;
mod service;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let ws_clients: SharedClients = Arc::new(Mutex::new(Vec::new()));
    let db_client = db::connect::run().await;

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", {
            let ws_clients = ws_clients.clone();
            any(move |ws, ua, info| ws_handler(ws, ua, info, ws_clients.clone(), db_client.clone()))
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
    ws_clients: SharedClients,
    db_client: Arc<Mutex<Client>>,
) -> impl IntoResponse {
    println!("{addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, ws_clients, db_client))
}

async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    ws_clients: SharedClients,
    db_client: Arc<Mutex<Client>>,
) {
    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(Mutex::new(sender));

    {
        let mut locked = ws_clients.lock().await;
        locked.push(sender.clone());
    }

    while let Some(Ok(msg)) = receiver.next().await {
        let sender_clone = sender.clone();

        if service::process_message(
            msg,
            who,
            &sender_clone,
            ws_clients.clone(),
            db_client.clone(),
        )
        .await
        .is_break()
        {
            break;
        }
    }

    {
        let mut locked = ws_clients.lock().await;
        locked.retain(|s| !Arc::ptr_eq(s, &sender));
    }

    println!("WebSocket {who} disconnected");
}
