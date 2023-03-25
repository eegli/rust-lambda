use lambda_http::{
    http::{header, StatusCode},
    Body, Error, Response,
};

pub fn payload_error(status: StatusCode, err: &str) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            serde_json::json!({
                "error": status.to_string(),
                "status_code" : status.as_u16(),
                "message": err
            })
            .to_string()
            .into(),
        )
        .map_err(Box::new)?)
}
