use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let mock_uid = "your mock uid";
    let response = client
        .update_mock(mock_uid)
        .mock(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
