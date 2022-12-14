#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let user_id = "your user id";
    let response = client
        .update_user_state(user_id)
        .schemas(&["your schemas"])
        .operations(vec![::serde_json::json!({})])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
