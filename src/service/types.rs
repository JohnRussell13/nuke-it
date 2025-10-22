use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientPayload {
    Spin {
        wallet_id: String,
        player_id: i32,
        amount: f64,
    },
    Fetch {},
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
    pub outcome: i32,
}

#[derive(Serialize, Debug)]
pub struct FetchResponse {
    pub items: Vec<LeaderBoard>,
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

pub fn create_error(msg: String) -> ServerMessage<ResponsePayload> {
    err(ResponsePayload::Error(ErrorResponse {
        message: String::from(msg),
    }))
}

#[derive(Serialize, Debug)]
pub struct LeaderBoard {
    player_id: i32,
    player_name: String,
    total_score: i64,
    total_staked: f64,
    rank: i64,
}

impl LeaderBoard {
    fn from_row(row: &Row) -> Self {
        Self {
            player_id: row.get("player_id"),
            player_name: row.get("player_name"),
            total_score: row.get("total_score"),
            total_staked: row.get::<_, f64>("total_staked"),
            rank: row.get("rank"),
        }
    }
}

pub fn rows_to_leaderboard(rows: &[Row]) -> Vec<LeaderBoard> {
    rows.iter().map(LeaderBoard::from_row).collect()
}
