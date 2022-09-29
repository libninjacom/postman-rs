use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .all_collections()
        .workspace_id("your workspace id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
