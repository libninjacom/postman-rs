#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client.service_provider_config().send().await.unwrap();
    println!("{:#?}", response);
}
