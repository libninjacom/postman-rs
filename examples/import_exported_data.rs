use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client.import_exported_data().send().await.unwrap();
    println!("{:#?}", response);
}
