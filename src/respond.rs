use lambda_http::{
    http::{header, StatusCode},
    Body, Error as LambdaErr, Response as LambdaResponse,
};

pub type Response = Result<LambdaResponse<Body>, LambdaErr>;

pub fn error(status: StatusCode, err: &str) -> Response {
    Ok(LambdaResponse::builder()
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

pub fn json<T>(body: T) -> Response
where
    T: serde::Serialize,
{
    Ok(LambdaResponse::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body)?.into())
        .map_err(Box::new)?)
}
