#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let monitor_uid = "your monitor uid";
    let response = client.delete_monitor(monitor_uid).send().await.unwrap();
    println!("{:#?}", response);
}
