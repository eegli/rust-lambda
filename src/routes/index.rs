use lambda_http::{
    http::{header, StatusCode},
    Request, RequestExt, Response,
};

use crate::{debug::DebugResponse, error, payload::Payload, RequestResponse};

pub fn index(event: Request) -> RequestResponse {
    let payload = event.payload::<Payload>();

    if let Err(err) = payload {
        return error::payload_error(StatusCode::BAD_REQUEST, &err.to_string());
    }
    let payload = payload.unwrap();

    let debug_res: DebugResponse<Payload> = DebugResponse::new()
        .with_path(event.raw_http_path())
        .with_context(&event.lambda_context())
        .with_headers(event.headers())
        .with_method(event.method())
        .with_query(&event.query_string_parameters())
        .with_payload(payload);

    Ok(Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&debug_res)?.into())
        .map_err(Box::new)?)
}
