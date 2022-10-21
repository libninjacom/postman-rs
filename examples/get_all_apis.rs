#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .get_all_apis()
        .workspace("your workspace")
        .since("your since")
        .until("your until")
        .created_by("your created by")
        .updated_by("your updated by")
        .is_public(true)
        .name("your name")
        .summary("your summary")
        .description("your description")
        .sort("your sort")
        .direction("your direction")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
