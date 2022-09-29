use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .merge_a_fork()
        .destination("your destination")
        .source("your source")
        .strategy("your strategy")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
