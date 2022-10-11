use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let collection_uid = "your collection uid";
    let response = client.single_collection(collection_uid).send().await.unwrap();
    println!("{:#?}", response);
}
