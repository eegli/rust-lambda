use lambda_http::{http::Method, run, service_fn, Body, Error, Request, RequestExt, Response};

mod debug;
mod error;
mod payload;
mod routes;

#[macro_use]
extern crate lazy_static;

pub type RequestResponse = Result<Response<Body>, Error>;

async fn handler(event: Request) -> RequestResponse {
    let path = event.raw_http_path();
    let mut path_iter = path.split("/").into_iter().skip(1);

    let is_get = event.method() == Method::GET;

    match path_iter.next() {
        Some("health") if is_get => return routes::health::health(),
        Some("data") if is_get => return routes::data::data(),
        _ => return routes::index::index(event),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(handler)).await
}
