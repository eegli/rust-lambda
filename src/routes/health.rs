use crate::respond;

pub async fn health() -> respond::Response {
    respond::json(serde_json::json!({ "status": "ok" }))
}
