use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::service::types::{self, ResponsePayload, ServerMessage, SpinResponse};

pub async fn run(
    wallet_id: String,
    player_id: i32,
    amount: f64,
    db_client: Arc<Mutex<Client>>,
) -> ServerMessage<ResponsePayload> {
    let outcome = 42; // example outcome
    let game_id = 1;

    let query = "
        INSERT INTO rolls (game_id, wallet_id, player_id, amount, roll) VALUES
        ($1, $2, $3, $4, $5);
    ";

    let response = db_client
        .lock()
        .await
        .execute(
            query,
            &[&game_id, &wallet_id, &player_id, &amount, &outcome],
        )
        .await;

    match response {
        Ok(_) => {
            return types::ok(ResponsePayload::Spin(SpinResponse { outcome }));
        }
        Err(msg) => return types::create_error(format!("Bad insert query: {}", msg.to_string())),
    };
}
