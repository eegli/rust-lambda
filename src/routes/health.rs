use lambda_http::{
    http::{header, StatusCode},
    Response,
};

use crate::RequestResponse;

pub fn health() -> RequestResponse {
    let status = StatusCode::OK;
    Ok(Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body("Ok".into())?)
}
