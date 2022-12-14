#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let user_id = "your user id";
    let response = client.fetch_user_resource(user_id).send().await.unwrap();
    println!("{:#?}", response);
}
