use std::fs;

use lambda_http::http::{header, Response, StatusCode};

use crate::RequestResponse;

lazy_static! {
    static ref PAYLOAD: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("fixtures/payload.json").unwrap()).unwrap();
}

pub fn data() -> RequestResponse {
    let status = StatusCode::OK;
    Ok(Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::json!({ "data": *PAYLOAD }).to_string().into())?)
}
