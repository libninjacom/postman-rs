use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let response = client
        .get_test_relations(api_id, api_version_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
