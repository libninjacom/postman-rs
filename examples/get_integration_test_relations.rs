use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let response = client
        .get_integration_test_relations(api_id, api_version_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
