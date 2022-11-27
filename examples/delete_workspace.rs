#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let workspace_id = "your workspace id";
    let response = client.delete_workspace(workspace_id).send().await.unwrap();
    println!("{:#?}", response);
}
