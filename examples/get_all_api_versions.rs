#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let response = client.get_all_api_versions(api_id).send().await.unwrap();
    println!("{:#?}", response);
}
