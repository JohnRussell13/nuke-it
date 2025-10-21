use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientPayload {
    Spin { id: u32 },
}

#[derive(Serialize, Debug)]
pub struct ServerMessage<T: Serialize> {
    pub status: Status,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ResponsePayload {
    Spin(SpinResponse),
    Fetch(FetchResponse),
    Error(ErrorResponse),
}

#[derive(Serialize, Debug)]
pub struct SpinResponse {
    pub outcome: u32,
}

#[derive(Serialize, Debug)]
pub struct FetchResponse {
    pub items: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Err,
}

pub fn ok<T: Serialize>(payload: T) -> ServerMessage<T> {
    ServerMessage {
        status: Status::Ok,
        payload,
    }
}

fn err<T: Serialize>(payload: T) -> ServerMessage<T> {
    ServerMessage {
        status: Status::Err,
        payload,
    }
}

pub fn create_error(msg: &str) -> ServerMessage<ResponsePayload> {
    err(ResponsePayload::Error(ErrorResponse {
        message: String::from(msg),
    }))
}
