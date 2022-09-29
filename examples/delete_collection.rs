use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let collection_uid = "your collection uid";
    let response = client.delete_collection(collection_uid).send().await.unwrap();
    println!("{:#?}", response);
}
