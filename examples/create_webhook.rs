#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .create_webhook()
        .workspace_id("your workspace id")
        .webhook(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
