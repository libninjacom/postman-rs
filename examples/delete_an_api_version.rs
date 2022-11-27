#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let response = client
        .delete_an_api_version(api_id, api_version_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
