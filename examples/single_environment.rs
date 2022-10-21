#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let environment_uid = "your environment uid";
    let response = client.single_environment(environment_uid).send().await.unwrap();
    println!("{:#?}", response);
}
