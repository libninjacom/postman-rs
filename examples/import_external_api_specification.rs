use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let body = ::serde_json::json!({});
    let response = client
        .import_external_api_specification(body)
        .workspace_id("your workspace id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
