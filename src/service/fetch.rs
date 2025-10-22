use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::service::types::{self, FetchResponse, ResponsePayload, ServerMessage};

pub async fn run(db_client: Arc<Mutex<Client>>) -> ServerMessage<ResponsePayload> {
    let query = "
        SELECT
            p.id AS player_id,
            p.name AS player_name,
            COALESCE(SUM(r.roll), 0) AS total_score,
            COALESCE(SUM(r.amount), 0) AS total_staked,
            RANK() OVER (ORDER BY COALESCE(SUM(r.roll), 0) DESC) AS rank
        FROM players p
        LEFT JOIN rolls r
            ON p.id = r.player_id
        WHERE p.game_id = 1
        GROUP BY p.id, p.name
        ORDER BY total_score DESC;
    ";

    let response = db_client.lock().await.query(query, &[]).await;

    match response {
        Ok(val) => {
            return types::ok(ResponsePayload::Fetch(FetchResponse {
                items: types::rows_to_leaderboard(&val),
            }));
        }
        Err(msg) => return types::create_error(msg.to_string()),
    };
}
