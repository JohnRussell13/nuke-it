use crate::service::types::FetchResponse;

pub fn run() -> FetchResponse {
    FetchResponse {
        items: vec!["item1".into(), "item2".into()],
    }
}
