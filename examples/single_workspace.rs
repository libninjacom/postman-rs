#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let workspace_id = "your workspace id";
    let response = client.single_workspace(workspace_id).send().await.unwrap();
    println!("{:#?}", response);
}
