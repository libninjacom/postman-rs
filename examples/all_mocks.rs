use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client.all_mocks().send().await.unwrap();
    println!("{:#?}", response);
}
