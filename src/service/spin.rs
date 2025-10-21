use crate::service::types::SpinResponse;

pub fn run(id: u32) -> SpinResponse {
    println!("Spin id: {}", id);
    let outcome = 42; // example outcome
    SpinResponse { outcome }
}
