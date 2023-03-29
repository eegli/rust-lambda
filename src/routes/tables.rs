use crate::{respond, Response};
use aws_sdk_dynamodb::Client as DynamoClient;

pub async fn list_tables() -> Response {
    /// This client needs to be initialized once...
    let conf = aws_config::from_env()
        .profile_name("nougats-spotify")
        .region("eu-central-1")
        .load()
        .await;
    let client = DynamoClient::new(&conf);
    let req = client.list_tables().limit(10);
    let resp = req.send().await;
    let table_names = resp?.table_names().unwrap().to_vec();
    respond::json(&table_names)
}
