#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
use postman::request::CreateCollectionFromSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = CreateCollectionFromSchemaRequired {
        api_id: "your api id",
        schema_id: "your schema id",
        relations: vec![::serde_json::json!({})],
        name: "your name",
        api_version_id: "your api version id",
    };
    let response = client
        .create_collection_from_schema(args)
        .workspace_id("your workspace id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
