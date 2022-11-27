#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let workspace = "your workspace";
    let collection_uid = "your collection uid";
    let response = client
        .create_a_fork(workspace, collection_uid)
        .label("your label")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
