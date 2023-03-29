use lambda_http::{http::StatusCode, Request, RequestExt};

use crate::{debug::DebugResponse, payload::Payload, respond};

pub async fn index(event: Request) -> respond::Response {
    let payload = event.payload::<Payload>();

    if let Err(err) = payload {
        return respond::error(StatusCode::BAD_REQUEST, &err.to_string());
    }

    let payload = payload.unwrap();

    let debug_res: DebugResponse<Payload> = DebugResponse::new()
        .with_path(event.raw_http_path())
        .with_context(event.lambda_context())
        .with_headers(event.headers())
        .with_method(event.method())
        .with_query(event.query_string_parameters())
        .with_payload(&payload);

    respond::json(&debug_res)
}
