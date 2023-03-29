use std::fs;

use crate::respond;

lazy_static! {
    static ref PAYLOAD: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("fixtures/payload.json").unwrap()).unwrap();
}

pub async fn data() -> respond::Response {
    respond::json(serde_json::json!({ "data": *PAYLOAD }))
}
