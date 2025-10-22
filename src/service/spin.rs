use std::sync::Arc;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::service::types::{self, ResponsePayload, ServerMessage, SpinResponse};

pub async fn run(
    wallet_id: String,
    player_id: i32,
    amount: f64,
    db_client: Arc<Mutex<Client>>,
) -> ServerMessage<ResponsePayload> {
    let outcome = roll();
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

pub fn roll() -> i32 {
    let mut rng = ChaCha20Rng::from_os_rng();
    rng.random_range(1..=6)
}
