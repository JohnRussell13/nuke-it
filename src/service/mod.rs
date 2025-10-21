use axum::extract::ws::Message;

use std::net::SocketAddr;
use std::ops::ControlFlow;

use futures_util::{Sink, sink::SinkExt};

mod fetch;
mod spin;

pub async fn process_message(
    msg: Message,
    who: SocketAddr,
    sender: &mut (impl Sink<Message> + Unpin),
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            let package = t.as_str();

            let response = match dispatch(package) {
                Ok(s) => format!("OK,{s}"),
                Err(s) => format!("ERR,{s}"),
            };

            if sender.send(Message::Text(response.into())).await.is_err() {
                eprintln!("Error sending message");
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

fn dispatch(package: &str) -> Result<String, String> {
    let package_parts: Vec<&str> = package.split(",").collect();
    match package_parts[0] {
        "spin" => spin::run(package_parts),
        "fetch" => fetch::run(),
        _ => Err(format!("Unknown action: {package_parts:?}")),
    }
}
