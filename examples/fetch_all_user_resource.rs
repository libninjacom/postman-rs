#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .fetch_all_user_resource()
        .start_index(1.0)
        .count(1.0)
        .filter("your filter")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
