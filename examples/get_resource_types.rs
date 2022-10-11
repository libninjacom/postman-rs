use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client.get_resource_types().send().await.unwrap();
    println!("{:#?}", response);
}
