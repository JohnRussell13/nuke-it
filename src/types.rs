use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use std::sync::Arc;
use tokio::sync::Mutex;

type Tx = Arc<Mutex<SplitSink<WebSocket, Message>>>;
pub type SharedClients = Arc<Mutex<Vec<Tx>>>;
