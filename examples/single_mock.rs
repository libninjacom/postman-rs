use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let mock_uid = "your mock uid";
    let response = client.single_mock(mock_uid).send().await.unwrap();
    println!("{:#?}", response);
}
