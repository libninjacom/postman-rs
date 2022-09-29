use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let response = client
        .create_schema(api_id, api_version_id)
        .schema(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
