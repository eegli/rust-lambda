use lambda_http::{http::Method, Error as LambdaErr, Request, RequestExt};

mod debug;
mod payload;
mod respond;
mod routes;

pub use respond::Response;

#[macro_use]
extern crate lazy_static;

async fn handler(event: Request) -> Response {
    let path = event.raw_http_path();
    let mut path_iter = path.split("/").into_iter().skip(1);

    let is_get = event.method() == Method::GET;

    let res = match path_iter.next() {
        Some("health") if is_get => routes::health::health().await?,
        Some("data") if is_get => routes::data::data().await?,
        Some("tables") if is_get => routes::tables::list_tables().await?,
        _ => routes::index::index(event).await?,
    };

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), LambdaErr> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    // list_tables().await?;

    let func = lambda_http::service_fn(handler);
    lambda_http::run(func).await?;
    Ok(())
}
