use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .create_user()
        .schemas(&["your schemas"])
        .user_name("your user name")
        .active(true)
        .external_id("your external id")
        .groups(&["your groups"])
        .locale("your locale")
        .name(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
