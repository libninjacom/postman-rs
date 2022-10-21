#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .create_collection()
        .workspace_id("your workspace id")
        .collection(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
