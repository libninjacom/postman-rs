use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let user_id = "your user id";
    let response = client
        .update_user_information(user_id)
        .schemas(&["your schemas"])
        .name(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
